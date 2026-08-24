#!/usr/bin/env python3
"""LightGBM stack over the quantamental catalog — can a non-linear combination
of sub-cost strategy signals (or their continuous indicator features) extract
anything the screen could not?

Runs a 2x2 grid — {discrete strategy signals, continuous indicator features} x
{lambdarank, regression on z-scored forward returns} — on the 2025-01-01..
2026-08-01 1h universe.  Train = first 13 months (last 3 as the early-stop
validation slice), the last 6 months (2026-02..08) held out as the arbiter.

Every run is judged the protocol's way (README.md):
  * per-bar cross-sectional Spearman rank IC vs 12h forward returns
    (train | val | forward);
  * overlap-free long-short net bps after --cost (top/bottom --k per bar);
  * lottery-alpha share (top-3 trades) on the forward window;
  * per-month forward net bps so a month-concentrated edge is visible.

This is constraint 1.5 pushed to a non-linear model: at best the catalog
supplies conditional gate/size features, not standalone alpha.  Prior
(gate_combos.py) says ridge-linear tops out ~+0.016pp with train/test
disagreeing — the forward window is the kill.  The insight deliverables are
the feature-importance rankings and the correlation of the 4 runs' predictions
(do the objectives and feature sets agree?).

Run:
    uv run python lgbm_stack.py                                       # full grid
    uv run python lgbm_stack.py --features signals --objectives lambdarank
    uv run python lgbm_stack.py --strategies rsi,ma_crossover \
        --features indicators --objectives regression --max-rounds 5   # smoke test
"""

from __future__ import annotations

import argparse
import os
import sys
from itertools import pairwise

import numpy as np
import polars as pl

_HERE = os.path.dirname(os.path.abspath(__file__))
_RESEARCH = os.path.abspath(os.path.join(_HERE, "..", ".."))
for _p in (_HERE, _RESEARCH):
    if _p not in sys.path:
        sys.path.insert(0, _p)

import quantamental as q

import screen
from indicators import (
    INDICATOR_FEATURES,
    adx_at,
    default_warmup,
)
from screen import (
    _tf_ms,
    load_universe,
    lottery_share,
    month_ranges,
    net_bps,
    precompute,
    rank_ic,
    signals_column,
    trade_rets,
)

FEATURE_SETS = ("signals", "indicators")
OBJECTIVES = ("lambdarank", "regression")

BASE_PARAMS = {
    "learning_rate": 0.03,
    "num_leaves": 31,
    "min_data_in_leaf": 200,
    "feature_fraction": 0.8,
    "bagging_fraction": 0.8,
    "bagging_freq": 1,
    "lambda_l2": 1.0,
    "verbosity": -1,
    "seed": 42,
    "num_threads": max(1, os.cpu_count() or 4),
}


def _ms(date_str: str) -> int:
    from datetime import datetime, timezone

    return int(
        datetime.strptime(date_str, "%Y-%m-%d").replace(tzinfo=timezone.utc).timestamp()
        * 1000
    )


# ── feature builders (aligned to the window-filtered universe rows) ─────────


def build_signal_matrix(uni: pl.DataFrame, win_mask: np.ndarray, ids: list[str]):
    """Discrete +1/0/-1 signals for every requested strategy; drops all-zero
    (never-firing) strategies.  Returns (X float32, feature names, dropped)."""
    cols, names, dropped = [], [], []
    for sid in ids:
        sig = signals_column(sid, uni, None).astype(np.float32)[win_mask]
        if not np.any(sig != 0):
            dropped.append(sid)
            continue
        cols.append(sig)
        names.append(sid)
    if not cols:
        raise SystemExit("no non-zero strategy signals — nothing to stack")
    return np.column_stack(cols).astype(np.float32), names, dropped


def indicator_matrix(uni: pl.DataFrame) -> dict[str, np.ndarray]:
    """Continuous indicator features per symbol (aligned to ``uni`` rows), the
    gate_combos feature set: 13 indicators + ADX at 7/14/28."""
    sym = uni["fsym"].to_numpy()
    starts = np.flatnonzero(sym[1:] != sym[:-1]) + 1
    bounds = np.concatenate(([0], starts, [len(uni)]))
    out: dict[str, np.ndarray] = {n: np.full(len(uni), np.nan) for n in FEATURE_NAMES}
    for a, b in pairwise(bounds):
        sub = uni.slice(a, b - a)
        h = sub["high"].to_numpy().astype(np.float64)
        low = sub["low"].to_numpy().astype(np.float64)
        c = sub["close"].to_numpy().astype(np.float64)
        v = sub["volume"].to_numpy().astype(np.float64)
        for n, fn in INDICATOR_FEATURES.items():
            out[n][a:b] = np.asarray(fn(h, low, c, v), dtype=float)
        for p in ADX_PERIODS:
            out[f"adx{p}"][a:b] = adx_at(h, low, c, v, p)
    return out


FEATURE_NAMES = list(INDICATOR_FEATURES) + [f"adx{p}" for p in (7, 14, 28)]
ADX_PERIODS = (7, 14, 28)


def build_indicator_matrix(uni: pl.DataFrame, win_mask: np.ndarray):
    """Continuous indicator features aligned to the window rows.  NaN feature
    rows are KEPT (LightGBM routes NaNs; the per-feature rank IC masks them)."""
    full = indicator_matrix(uni)
    X = np.column_stack([full[n] for n in FEATURE_NAMES])
    return X[win_mask], FEATURE_NAMES, np.full(len(uni), True)


# ── targets / splits ─────────────────────────────────────────────────────────


def forward_return(uni: pl.DataFrame, horizon_h: int) -> np.ndarray:
    """Timestamp-based forward return (gap-safe) per row of ``uni``."""
    ts = uni["timestamp"].to_numpy()
    close = uni["close"].to_numpy().astype(np.float64)
    sym = uni["fsym"].to_numpy()
    H = horizon_h * screen.HOUR_MS
    out = np.full(len(uni), np.nan)
    starts = np.flatnonzero(sym[1:] != sym[:-1]) + 1
    bounds = np.concatenate(([0], starts, [len(uni)]))
    for a, b in pairwise(bounds):
        t = ts[a:b]
        c = close[a:b]
        idx = np.searchsorted(t, t + H, side="right") - 1
        fin = idx >= 0
        out[a:b][fin] = c[np.clip(idx, 0, len(c) - 1)][fin] / c[fin] - 1.0
    return out


def train_z(fwd: np.ndarray, sym: np.ndarray, train_mask: np.ndarray) -> np.ndarray:
    """Per-symbol z of forward returns, normalized on TRAIN rows only (no
    forward-window stats leak into the target)."""
    z = np.full(len(fwd), np.nan)
    for s in np.unique(sym[train_mask & np.isfinite(fwd)]):
        m = (sym == s) & train_mask & np.isfinite(fwd)
        sd = float(fwd[m].std())
        if sd and np.isfinite(sd):
            z[sym == s] = (fwd[sym == s] - float(fwd[m].mean())) / sd
    return z


def relevance_grades(fwd: np.ndarray, ts: np.ndarray) -> np.ndarray:
    """Per-bar relevance grade 0..4 (within-bar percentile rank x4, rounded) —
    the lambdarank label.  Rows with non-finite fwd get -1 (excluded)."""
    df = pl.DataFrame({"ts": ts, "fwd": fwd})
    df = df.with_columns(
        pl.col("fwd").rank().over("ts").alias("r"),
        pl.col("fwd").count().over("ts").alias("nbar"),
    )
    r = df["r"].to_numpy()
    nb = df["nbar"].to_numpy()
    nan = np.isnan(fwd)
    grad = np.where(
        nan, -1, np.floor((r - 1.0) / np.maximum(nb - 1, 1) * 5).astype(np.int8)
    )
    grad = np.clip(grad, 0, 4)
    grad[nan] = -1
    return grad


# ── evaluation ──────────────────────────────────────────────────────────────


def scores_to_sigs(ts: np.ndarray, sym: np.ndarray, score: np.ndarray, k: int) -> dict:
    """Per-symbol int8 signal: +1 top-k / -1 bottom-k score per bar, 0 else."""
    df = pl.DataFrame({"ts": ts, "sym": sym, "score": score})
    df = df.with_columns(
        pl.col("score").rank().over("ts").alias("r"),
        pl.col("score").count().over("ts").alias("nbar"),
    )
    r = df["r"].to_numpy()
    nb = df["nbar"].to_numpy()
    fin = np.isfinite(score)
    long = fin & (r > nb - k)
    short = fin & (r <= k)
    sigs: dict[str, np.ndarray] = {}
    starts = np.flatnonzero(sym[1:] != sym[:-1]) + 1
    bounds = np.concatenate(([0], starts, [len(sym)]))
    for a, b in pairwise(bounds):
        arr = np.zeros(b - a, dtype=np.int8)
        arr[long[a:b]] = 1
        arr[short[a:b]] = -1
        sigs[sym[a]] = arr
    return sigs


def rank_ic_split(ts, score, fwd, mask, label: str) -> dict:
    if mask.sum() < 50:
        print(f"    {label}: too few rows")
        return None
    df = pl.DataFrame({"timestamp": ts[mask], "score": score[mask], "fwd": fwd[mask]})
    return rank_ic(df, "score", "fwd")


def pearson_eval(preds, ds):
    y = ds.get_label()
    if preds.std() == 0 or y.std() == 0 or len(preds) < 3:
        return [("pearson", 0.0, True)]
    return [("pearson", float(np.corrcoef(preds, y)[0, 1]), True)]


# ── per-run pipeline ────────────────────────────────────────────────────────


def run_one(
    feature_kind: str,
    objective: str,
    uni: pl.DataFrame,
    win_mask: np.ndarray,
    args,
    strategy_ids: list[str],
) -> dict:
    print(f"\n=== {feature_kind} x {objective} ===", file=sys.stderr, flush=True)

    if feature_kind == "signals":
        X, names, dropped = build_signal_matrix(uni, win_mask, strategy_ids)
        print(
            f"  signals: {len(names)} strategies ({len(dropped)} dropped all-zero: "
            f"{', '.join(sorted(dropped))})",
            file=sys.stderr,
        )
        row_keep = np.full(len(uni), True)
    else:
        X, names, row_keep = build_indicator_matrix(uni, win_mask)
        print(f"  indicators: {len(names)} features", file=sys.stderr)

    ts_all = uni["timestamp"].to_numpy()
    sym = uni["fsym"].to_numpy()
    close = uni["close"].to_numpy().astype(np.float64)
    fwd = forward_return(uni, args.horizon)
    win_ts = ts_all[win_mask]
    win_sym = sym[win_mask]
    win_close = close[win_mask]
    win_fwd = fwd[win_mask]
    keep = row_keep[win_mask]

    # bars with < min_syms symbols are not model rows (screen's rank-IC floor)
    cnt = (
        pl.DataFrame({"ts": win_ts, "fwd": win_fwd}).with_columns(
            pl.col("fwd").count().over("ts").alias("n")
        )
    )["n"].to_numpy()
    keep &= np.isfinite(win_fwd) & (cnt >= args.min_syms)

    ts = win_ts[keep]
    sym_w = win_sym[keep]
    close_w = win_close[keep]
    fwd_w = win_fwd[keep]
    Xw = X[keep]
    print(
        f"  model rows: {len(ts):,}  bars: {np.unique(ts).size}  "
        f"symbols: {np.unique(sym_w).size}",
        file=sys.stderr,
    )

    ranges = month_ranges(ts)
    fwd_ranges = ranges[len(ranges) - args.forward_months :]
    val_ranges = ranges[len(ranges) - args.forward_months - 3 :][:3]
    train_ranges = ranges[: len(ranges) - args.forward_months]
    val_cut = val_ranges[0][1]
    fwd_cut = fwd_ranges[0][1]
    train = ts < val_cut
    val = (ts >= val_cut) & (ts < fwd_cut)
    forward = ts >= fwd_cut
    tr_mo = month_ranges(ts[train])
    va_mo = month_ranges(ts[val])
    fw_mo = month_ranges(ts[forward])
    print(
        f"  split: train {train.sum():,} ({tr_mo[0][0]}..{tr_mo[-1][0]}) | "
        f"val {val.sum():,} ({va_mo[0][0]}..{va_mo[-1][0]}) | "
        f"forward {forward.sum():,} ({fw_mo[0][0]}..{fw_mo[-1][0]})",
        file=sys.stderr,
    )

    import lightgbm as lgb

    params = dict(BASE_PARAMS)
    callbacks = [lgb.log_evaluation(100)]
    if objective == "lambdarank":
        params.update(
            {"objective": "lambdarank", "metric": "ndcg", "ndcg_eval_at": [5, 10]}
        )
        rel = relevance_grades(fwd_w, ts)
        tr_ord = np.argsort(ts[train], kind="stable")
        va_ord = np.argsort(ts[val], kind="stable")
        _, tr_sizes = np.unique(ts[train][tr_ord], return_counts=True)
        _, va_sizes = np.unique(ts[val][va_ord], return_counts=True)
        tr_ds = lgb.Dataset(
            Xw[train][tr_ord],
            label=rel[train][tr_ord],
            group=tr_sizes,
            feature_name=names,
        )
        va_ds = lgb.Dataset(
            Xw[val][va_ord],
            label=rel[val][va_ord],
            group=va_sizes,
            reference=tr_ds,
        )
    else:
        params.update({"objective": "regression", "metric": "None"})
        z = train_z(fwd_w, sym_w, train)
        tr_ds = lgb.Dataset(Xw[train], label=z[train], feature_name=names)
        va_ds = lgb.Dataset(Xw[val], label=z[val], reference=tr_ds)
    callbacks.append(lgb.early_stopping(100, verbose=True))

    feval = pearson_eval if objective == "regression" else None
    bst = lgb.train(
        params,
        tr_ds,
        valid_sets=[va_ds],
        num_boost_round=args.max_rounds,
        feval=feval,
        callbacks=callbacks,
    )
    n_iter = bst.best_iteration if bst.best_iteration else args.max_rounds
    score = bst.predict(Xw, num_iteration=n_iter)
    print(f"  best iteration {n_iter} ({bst.best_score})", file=sys.stderr)

    # ── evaluation ──
    res = {
        "feature_kind": feature_kind,
        "objective": objective,
        "features": len(names),
        "iterations": n_iter,
    }
    print(f"\n  rank IC (per-bar cross-sectional Spearman, H={args.horizon}h):")
    for mask, label in ((train, "train"), (val, "val"), (forward, "FORWARD")):
        r = rank_ic_split(ts, score, fwd_w, mask, label)
        if r is None:
            continue
        res[f"ic_{label.lower()}"] = round(r["mean"], 5)
        print(
            f"    {label:8s} mean {r['mean']:+.5f}  t {r['t']:+7.2f}  bars {r['bars']}"
        )

    sigs = scores_to_sigs(ts, sym_w, score, args.k)
    # fwd/anchor/ts_by for the trade sim come from the model frame itself (real
    # closes) so the simulated forward returns match the training targets.
    uni_m = pl.DataFrame({"fsym": sym_w, "timestamp": ts, "close": close_w})
    fwd_d, anchor, ts_by = precompute(uni_m, args.horizon)

    tr_nb, tr_n = net_bps(fwd_d, anchor, ts_by, sigs, train_ranges, args.cost)
    fw_nb, fw_n = net_bps(fwd_d, anchor, ts_by, sigs, fwd_ranges, args.cost)
    fw_rets = trade_rets(fwd_d, anchor, ts_by, sigs, fwd_ranges)
    share = lottery_share(fw_rets)
    res.update(
        {
            "net_train": tr_nb,
            "net_forward": fw_nb,
            "n_forward": fw_n,
            "lottery_fwd": share,
        }
    )
    print(
        f"\n  net bps (H={args.horizon}h, cost={args.cost:.0f}, long/short {args.k}): "
        f"train {tr_nb:+.2f} ({tr_n}) | FORWARD {fw_nb:+.2f} ({fw_n})  "
        f"top3-share {share * 100:.0f}%"
    )
    pos = 0
    print("    per-month FORWARD net bps:")
    for (y, m), lo, hi in fwd_ranges:
        nb, n = net_bps(fwd_d, anchor, ts_by, sigs, [((y, m), lo, hi)], args.cost)
        mark = "+" if nb > 0 else " "
        if nb > 0:
            pos += 1
        print(f"      {y}-{m:02d}: {nb:+8.2f} bps ({n:5d}) {mark}")
    print(f"    forward positive months: {pos}/{len(fwd_ranges)}")

    # ── insights: importance + the leaned-on features' single-feature IC ──
    imp = bst.feature_importance("gain")
    order = np.argsort(imp)[::-1]
    print("\n  top features by gain (single-feature rank IC train | forward):")
    n_show = min(args.top_n, len(names))
    for i in order[:n_show]:
        cells = []
        fcol = Xw[:, i]
        fin = np.isfinite(fcol)
        for mask in (train, forward):
            r = rank_ic_split(ts, fcol, fwd_w, mask & fin, "")
            cells.append(f"{r['mean']:+.4f}" if r else "   --")
        print(f"    {names[i]:32s} gain {imp[i]:>9.1f}  IC {cells[0]} | {cells[1]}")

    # save predictions for the cross-run correlation + reuse
    out = pl.DataFrame({"timestamp": ts, "fsym": sym_w, "fwd": fwd_w, "score": score})
    out.write_parquet(
        os.path.join(_HERE, "_cache", f"lgbm_{feature_kind}_{objective}.parquet")
    )
    return res, {"timestamp": ts, "fsym": sym_w, "fwd": fwd_w, "score": score}


# ── main ────────────────────────────────────────────────────────────────────


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument(
        "--features", default=",".join(FEATURE_SETS), help="comma list of feature sets"
    )
    ap.add_argument(
        "--objectives", default=",".join(OBJECTIVES), help="comma list of objectives"
    )
    ap.add_argument(
        "--strategies",
        default=None,
        help="explicit strategy list (default: whole registry)",
    )
    ap.add_argument("--tf", default="1h")
    ap.add_argument("--start", default="2025-01-01")
    ap.add_argument("--end", default="2026-08-01")
    ap.add_argument("--horizon", type=int, default=12)
    ap.add_argument("--forward-months", type=int, default=6)
    ap.add_argument("--min-syms", type=int, default=20, help="rank-IC symbol floor")
    ap.add_argument("--k", type=int, default=10, help="symbols per long/short side")
    ap.add_argument("--cost", type=float, default=10.0)
    ap.add_argument("--max-rounds", type=int, default=2000)
    ap.add_argument("--top-n", type=int, default=10)
    args = ap.parse_args()

    screen.HOUR_MS = _tf_ms(args.tf)
    warmup = default_warmup(args.tf, args.start)
    print(
        f"loading {args.tf} bars {warmup}..{args.end} (warmup {warmup})...",
        file=sys.stderr,
        flush=True,
    )
    uni = load_universe(args.tf, warmup, args.end)
    win_mask = uni["timestamp"].to_numpy() >= _ms(args.start)
    n_syms = uni.filter(pl.Series(win_mask))["fsym"].n_unique()
    print(f"  window rows: {win_mask.sum():,}  symbols: {n_syms}", file=sys.stderr)

    if args.strategies:
        strategy_ids = [s.strip() for s in args.strategies.split(",") if s.strip()]
    else:
        strategy_ids = sorted(q.get_strategy_registry()["strategies"])

    results, preds = [], {}
    for feat in [f.strip() for f in args.features.split(",") if f.strip()]:
        for obj in [o.strip() for o in args.objectives.split(",") if o.strip()]:
            res, p = run_one(feat, obj, uni, win_mask, args, strategy_ids)
            results.append(res)
            preds[f"{feat}_{obj}"] = p

    # ── summary table ──
    print("\n" + "=" * 92)
    print("summary — train IC | forward IC | forward net bps | trades | lottery")
    for r in results:
        print(
            f"  {r['feature_kind']:10s} {r['objective']:10s} "
            f"{r.get('ic_train', float('nan')):+8.4f} {r.get('ic_forward', float('nan')):+8.4f} "
            f"{r['net_forward']:+9.2f} {r['n_forward']:7d} {r['lottery_fwd'] * 100:6.0f}%"
        )

    # ── cross-run prediction correlation (forward rows) ──
    if len(preds) > 1:
        all_ts = np.concatenate([p["timestamp"] for p in preds.values()])
        fwd_cut = month_ranges(all_ts)[-args.forward_months][1]
        base = None
        for key, p in preds.items():
            df = pl.DataFrame({"ts": p["timestamp"], "sym": p["fsym"], key: p["score"]})
            df = df.filter(pl.col("ts") >= fwd_cut)
            base = df if base is None else base.join(df, on=["ts", "sym"])
        cols = list(preds)
        arr = np.column_stack([base[c].to_numpy() for c in cols])
        corr = np.corrcoef(arr.T)
        print("\nprediction correlation (forward rows):")
        w = max(len(c) for c in cols) + 2
        print(f"{'':>{w}} " + "".join(f"{c[:12]:>14}" for c in cols))
        for i, a in enumerate(cols):
            print(
                f"{a[: w - 2]:>{w}} "
                + "".join(f"{corr[i, j]:>14.2f}" for j in range(len(cols)))
            )

    print(
        "\nVerdict rule: the forward window is the arbiter.  Positive forward "
        "net bps after cost with a low lottery share is a candidate for the "
        "separator/gate test — negative/zero forward is the kill."
    )


if __name__ == "__main__":
    main()
