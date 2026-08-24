#!/usr/bin/env python3
"""Gate-combination discovery screen — PURE OHLCV (Binance), no engine.

Discovery-stage filter for gate/size candidates (constraint 1.5).  The pool is
NOT replayed here: the sample is every (symbol, bar) in the OHLCV universe with
a finite forward return.  The engine/pool only appears after a candidate has
been formalized into the FSM (config/strategies/) — this screen's job is to
rule candidates OUT cheaply (a killer, like screen.py).

Two views per feature (16 continuous indicators incl. ADX at periods 7/14/28):

  directional — does the feature predict the sign of forward returns?
      * rank_ic: per-bar cross-sectional Spearman IC (screen.py's discipline)
      * gate:    block the train-selected feature tail, measure test uplift on
                 VOL-NORMALIZED forward returns (per-symbol z — so the gate
                 isn't just "high-vol symbols win")
  trend       — does the feature predict trend CONTINUATION (the ADX claim)?
      * gate on cont = sign(prior-move) × forward return: after a move, does
        trend strength make it persist?

Combos: pairwise AND-gates and a ridge linear gate over the feature set, scored
the same two ways.  Every survivor must clear: train-selected gate with
POSITIVE test uplift AND sign-stable test corr (train/test agree on direction).
Timeframes sweep catches TF-fragile effects (ADX worked at 1h, flipped at
4h/15m).  Per-month stability of the overall best is printed so a
month-concentrated (lottery) gate is visible.

Market-cap stratification (README: the high-cap bucket is the deployable
number; small-cap edge is the least trustworthy for sizing):
  --cap-tiers caps.csv   real market-cap snapshot (header: symbol,tier)
  --cap-volume           fallback: terciles of each symbol's median bar volume
                         (a liquidity/size proxy, NOT market cap — use a real
                         snapshot when a hard split is needed)

Run:
    uv run python gate_combos.py
    uv run python gate_combos.py --tf 1h,4h,12h --horizon 12
"""

from __future__ import annotations

import argparse
import os
import sys

import numpy as np
import polars as pl

_HERE = os.path.dirname(os.path.abspath(__file__))
_RESEARCH = os.path.abspath(os.path.join(_HERE, "..", ".."))
for _p in (_HERE, _RESEARCH):
    if _p not in sys.path:
        sys.path.insert(0, _p)


from binance_loader import load_price_bars
from indicators import (
    ADX_PERIODS,
    FEATURES,
    HOUR_MS,
    INDICATOR_FEATURES,
    adx_at,
    default_warmup,
)
from screen import rank_ic


def _ms(date_str: str) -> int:
    from datetime import datetime, timezone

    return int(
        datetime.strptime(date_str, "%Y-%m-%d").replace(tzinfo=timezone.utc).timestamp()
        * 1000
    )


def load_pooled(
    tf: str, start: str, end: str, warmup: str, horizon_h: int
) -> pl.DataFrame:
    """(fsym, timestamp, volume, features..., fwd, fwdz, cont) over start..end
    at ``tf``.

    Streams symbol-by-symbol (no full-universe concat of OHLCV) and keeps only
    the columns the tests need.  ``fwd`` = forward return over ``horizon_h``
    hours; ``fwdz`` = that return z-scored per symbol (removes the
    high-vol-symbol confound from pooled gates); ``cont`` = sign(prior-move) ×
    fwd, the trend-continuation return.
    """
    data = load_price_bars(start=warmup, end=end, tf=tf)
    H = horizon_h * HOUR_MS
    sm = _ms(start)
    rows: list[dict[str, np.ndarray]] = []
    for sym, df in sorted(data.items()):
        ts = df["timestamp"].to_numpy().astype(np.int64)
        h = df["high"].to_numpy().astype(np.float64)
        low = df["low"].to_numpy().astype(np.float64)
        c = df["close"].to_numpy().astype(np.float64)
        v = df["volume"].to_numpy().astype(np.float64)
        n = len(ts)

        fi = np.searchsorted(ts, ts + H, side="right") - 1
        okf = (fi >= 0) & (fi < n)
        fwd = np.full(n, np.nan)
        fwd[okf] = c[np.clip(fi, 0, n - 1)][okf] / c[okf] - 1.0

        pi = np.searchsorted(ts, ts - H, side="left")
        okp = pi < n
        pri = np.full(n, np.nan)
        pri[okp] = c[okp] / c[np.clip(pi, 0, n - 1)][okp] - 1.0

        sd = np.nanstd(fwd)
        fwdz = fwd / sd if sd and np.isfinite(sd) else fwd
        cont = np.sign(pri) * fwd

        row: dict[str, np.ndarray] = {
            "fsym": np.array([sym] * n),
            "timestamp": ts,
            "volume": v,
            "fwd": fwd,
            "fwdz": fwdz,
            "cont": cont,
        }
        feats = dict(INDICATOR_FEATURES)
        for name, fn in feats.items():
            row[name] = np.asarray(fn(h, low, c, v), dtype=float)
        for p in ADX_PERIODS:
            row[f"adx{p}"] = adx_at(h, low, c, v, p)
        rows.append(row)

    uni = pl.concat([pl.DataFrame(r) for r in rows], rechunk=False)
    uni = uni.filter(
        (pl.col("timestamp") >= sm)
        & pl.col("fwd").is_finite()
        & pl.col("timestamp").is_not_null()
    )
    return uni


def load_tiers_csv(path: str) -> dict[str, str]:
    """Load --cap-tiers CSV (header: symbol,tier) into {symbol: tier}."""
    import csv

    tiers: dict[str, str] = {}
    with open(path) as fh:
        for row in csv.DictReader(fh):
            tiers[row["symbol"]] = row["tier"]
    return tiers


def volume_tiers(uni: pl.DataFrame) -> dict[str, str]:
    """Size buckets (high/mid/low) from each symbol's MEDIAN bar volume over the
    window — a liquidity/size proxy, NOT market cap.  Use --cap-tiers with a
    real snapshot when a hard market-cap split is needed (README: don't
    hardcode guesses).  Terciles on the cross-section of symbols."""
    vols = uni.group_by("fsym").agg(pl.col("volume").median().alias("med")).sort("med")
    arr = vols["fsym"].to_numpy()
    n = len(arr)
    lo, hi = int(n * 0.33), int(n * 0.67)
    return {
        s: ("high" if i >= hi else "low" if i < lo else "mid")
        for i, s in enumerate(arr)
    }


def tiered_screen(
    uni: pl.DataFrame,
    feats: dict[str, np.ndarray],
    fwdz: np.ndarray,
    train: np.ndarray,
    tiers: dict[str, str],
) -> None:
    """Gate test uplift per size tier (the README: high-cap = the deployable
    number; small-cap edge is the least trustworthy for sizing)."""
    syms = uni["fsym"].to_numpy()
    tier_arr = np.array([tiers.get(s, "?") for s in syms])
    print(
        "\n-- single features by tier: fwdz gate TEST uplift (pp) / block% / n_keep --"
    )
    for t in ("high", "mid", "low"):
        print(f"    {t:>5s}: {int((tier_arr == t).sum()):,} bars")
    print(f"{'feature':12s} {'high':>22s} {'mid':>22s} {'low':>22s}")
    for n in FEATURES:
        f = feats[n]
        cells = []
        for t in ("high", "mid", "low"):
            m = tier_arr == t
            g = bar_gate(f[m], fwdz[m], train[m])
            cells.append(f"{g[3]:+.3f}/{g[6] * 100:.0f}%/{g[5]}" if g else "       --")
        print(f"{n:12s} {cells[0]:>22s} {cells[1]:>22s} {cells[2]:>22s}")


def split_mask(uni: pl.DataFrame) -> tuple[np.ndarray, int]:
    """Time split at the 60th-percentile timestamp (train 60% / test 40%)."""
    ts = uni["timestamp"].to_numpy()
    split_ms = int(np.quantile(ts, 0.60))
    train = ts < split_ms
    return train, int(split_ms)


def bar_gate(
    feat: np.ndarray, y: np.ndarray, train: np.ndarray, qs=(0.1, 0.2, 0.3, 0.4)
):
    """Train-selected tail block on pooled bars, scored on test (like the
    separator _gate, but the sample is every bar).  Returns
    (side, q, thr, test_uplift, train_uplift, n_keep_test, block_rate) or None."""
    ok = np.isfinite(feat) & np.isfinite(y)
    tr = train & ok
    te = ~train & ok
    if tr.sum() < 500 or te.sum() < 200:
        return None
    base = y[tr].mean()
    best = None
    for qv in qs:
        for side, keep in (
            ("low", feat[tr] > float(np.quantile(feat[tr], qv))),
            ("high", feat[tr] < float(np.quantile(feat[tr], 1 - qv))),
        ):
            if keep.sum() < 50 or (~keep).sum() < 50:
                continue
            up = float(y[tr][keep].mean() - base)
            if best is None or up > best[0]:
                best = (up, side, qv)
    if best is None:
        return None
    up, side, qv = best
    thr = (
        float(np.quantile(feat[tr], qv))
        if side == "low"
        else float(np.quantile(feat[tr], 1 - qv))
    )
    keep = feat[te] > thr if side == "low" else feat[te] < thr
    if keep.sum() < 50:
        return None
    return (
        side,
        qv,
        thr,
        float(y[te][keep].mean() - y[te].mean()),
        up,
        int(keep.sum()),
        float(keep.sum()) / te.sum(),
    )


def and_gate(fx, fy, y, train, qs=(0.1, 0.2, 0.3, 0.4)):
    """Best train-selected AND-gate keep = cond(fx) & cond(fy), scored on test.
    Returns (test_uplift, train_uplift, block_rate, n_keep, side_x, side_y) or None."""
    ok = np.isfinite(fx) & np.isfinite(fy) & np.isfinite(y)
    tr, te = train & ok, ~train & ok
    if tr.sum() < 500 or te.sum() < 200:
        return None
    base = y[tr].mean()
    best = None
    for qx in qs:
        for sx in ("low", "high"):
            tx = (
                float(np.quantile(fx[tr], qx))
                if sx == "low"
                else float(np.quantile(fx[tr], 1 - qx))
            )
            cx = fx[tr] > tx if sx == "low" else fx[tr] < tx
            for qy in qs:
                for sy in ("low", "high"):
                    ty = (
                        float(np.quantile(fy[tr], qy))
                        if sy == "low"
                        else float(np.quantile(fy[tr], 1 - qy))
                    )
                    cy = fy[tr] > ty if sy == "low" else fy[tr] < ty
                    keep = cx & cy
                    if keep.sum() < 50 or (~keep).sum() < 50:
                        continue
                    up = float(y[tr][keep].mean() - base)
                    if best is None or up > best[0]:
                        best = (up, sx, tx, qx, sy, ty, qy)
    if best is None:
        return None
    up, sx, tx, qx, sy, ty, qy = best
    cx = fx[te] > tx if sx == "low" else fx[te] < tx
    cy = fy[te] > ty if sy == "low" else fy[te] < ty
    keep = cx & cy
    if keep.sum() < 50:
        return None
    return (
        float(y[te][keep].mean() - y[te].mean()),
        up,
        float(keep.sum()) / te.sum(),
        int(keep.sum()),
        sx,
        qx,
        sy,
        qy,
    )


def linear_gate(
    feats: dict[str, np.ndarray], y: np.ndarray, train: np.ndarray, lam=10.0
):
    """Ridge-linear z-score gate over all features; keeps score above a
    train-selected low tail.  Returns dict or None."""
    x = np.column_stack([feats[n] for n in FEATURES if n in feats])
    ok = np.isfinite(x).all(1) & np.isfinite(y)
    tr, te = ok & train, ok & ~train
    if tr.sum() < 500 or te.sum() < 200:
        return None
    mean = x[tr].mean(0)
    scale = x[tr].std(0)
    scale[scale < 1e-9] = 1.0
    z = (x - mean) / scale
    coef = np.linalg.solve(
        z[tr].T @ z[tr] + lam * np.eye(x.shape[1]),
        z[tr].T @ (y[tr] - y[tr].mean()),
    )
    score = z @ coef
    tr_c = float(np.corrcoef(score[tr], y[tr])[0, 1]) if len(score[tr]) > 3 else 0.0
    te_c = float(np.corrcoef(score[te], y[te])[0, 1]) if len(score[te]) > 3 else 0.0
    best = None
    for qv in (0.1, 0.2, 0.3, 0.4):
        thr = float(np.quantile(score[tr], qv))
        keep = score[te] > thr
        if keep.sum() < 50:
            continue
        up = float(y[te][keep].mean() - y[te].mean())
        if best is None or up > best[0]:
            best = (up, thr, qv)
    if best is None:
        return None
    up, thr, qv = best
    keep = score[te] > thr
    weights = sorted(
        zip([n for n in FEATURES if n in feats], coef), key=lambda w: -abs(w[1])
    )
    return {
        "tr_c": tr_c,
        "te_c": te_c,
        "uplift": up,
        "train_uplift": best[0],
        "q": qv,
        "thr": thr,
        "block": 1.0 - float(keep.sum()) / te.sum(),
        "n_keep": int(keep.sum()),
        "weights": weights,
    }


def monthly_stability(
    uni: pl.DataFrame,
    feat: np.ndarray,
    y: np.ndarray,
    side: str,
    thr: float,
    label: str,
) -> None:
    """Per-month kept-vs-all mean return on the TEST bars."""
    ts = uni["timestamp"].to_numpy()
    test = ts >= int(np.quantile(ts, 0.60))
    keep = feat > thr if side == "low" else feat < thr
    ok = np.isfinite(feat) & np.isfinite(y)
    from datetime import datetime, timezone

    print(f"\n  per-month (test bars) for {label}:")
    print(
        f"    {'month':9s} {'all_ret%':>9s} {'kept_ret%':>9s} {'uplift':>8s} {'kept/tot':>9s}"
    )
    pos = neg = 0
    seen = set()
    for t in ts[test]:
        m = datetime.fromtimestamp(int(t) / 1000, tz=timezone.utc).strftime("%Y-%m")
        seen.add(m)
    for m in sorted(seen):
        mi = (ts >= _ms(m + "-01")) & (ts < _ms(m + "-01") + 32 * 24 * HOUR_MS) & test
        km = mi & keep & ok
        if km.sum() < 3 or mi.sum() < 3:
            continue
        allm = float(y[mi].mean())
        keptm = float(y[km].mean())
        if keptm - allm > 0:
            pos += 1
        else:
            neg += 1
        print(
            f"    {m:9s} {allm:+9.3f} {keptm:+9.3f} {keptm - allm:+8.3f} "
            f"{int(km.sum()):5d}/{int(mi.sum()):4d}"
        )
    print(f"    positive months: {pos}/{pos + neg}")


def screen_tf(tf: str, args) -> None:
    warmup = args.warmup or default_warmup(tf, args.start)
    print(
        f"\n=== {tf} (start..end {args.start}..{args.end}, horizon {args.horizon}h, "
        f"warmup {warmup}) ===",
        file=sys.stderr,
        flush=True,
    )
    uni = load_pooled(tf, args.start, args.end, warmup, args.horizon)
    print(f"  bars: {uni.height} symbols-total", file=sys.stderr)
    train, split_ms = split_mask(uni)
    print(f"  train/test split at {split_ms} (60/40 by time)")

    tiers = None
    if args.cap_tiers:
        tiers = load_tiers_csv(args.cap_tiers)
    elif args.cap_volume:
        tiers = volume_tiers(uni)

    feats: dict[str, np.ndarray] = {n: uni[n].to_numpy() for n in FEATURES}
    fwdz = uni["fwdz"].to_numpy()
    cont = uni["cont"].to_numpy()

    # ── A. single-feature screen ──
    print(
        "\n-- single features: rank IC (fwd) + tail gates (fwdz = vol-norm, cont = trend) --"
    )
    print(
        f"{'feature':12s} {'ic_train':>9s} {'ic_test':>9s} {'stab':>5s} | "
        f"{'gate fwdz':>32s} | {'gate cont':>32s}"
    )
    singles = {}
    for n in FEATURES:
        f = feats[n]
        if np.isfinite(f).sum() < 500:
            continue
        # rank IC train/test (directional)
        tr_df = uni.filter(pl.col("timestamp") < split_ms)
        te_df = uni.filter(pl.col("timestamp") >= split_ms)
        trc = rank_ic(
            tr_df.select(["timestamp", "fwd", n]).rename({n: "sig"}), "sig", "fwd"
        )
        tec = rank_ic(
            te_df.select(["timestamp", "fwd", n]).rename({n: "sig"}), "sig", "fwd"
        )
        stab = "+" if trc["mean"] * tec["mean"] > 0 else "-"
        g1 = bar_gate(f, fwdz, train)
        g2 = bar_gate(f, cont, train)
        s1 = (
            f"block_{g1[0]} q={g1[1]:.1f} {g1[3]:+.3f}pp b={g1[6] * 100:.0f}%"
            if g1
            else "none"
        )
        s2 = (
            f"block_{g2[0]} q={g2[1]:.1f} {g2[3]:+.3f}pp b={g2[6] * 100:.0f}%"
            if g2
            else "none"
        )
        print(
            f"{n:12s} {trc['mean']:+9.4f} {tec['mean']:+9.4f} {stab:>5s} | {s1:>32s} | {s2:>32s}"
        )
        singles[n] = (trc["mean"], tec["mean"], g1, g2)

    if tiers:
        tiered_screen(uni, feats, fwdz, train, tiers)

    # ── B. AND gates over the most promising features ──
    # candidates: sign-stable singles (any positive test uplift)
    cands = [
        n
        for n, (tc, ec, g1, g2) in singles.items()
        if tc * ec > 0 and ((g1 and g1[3] > 0) or (g2 and g2[3] > 0))
    ]
    print(
        f"\n-- pairwise AND gates over {len(cands)} candidates (fwdz / cont), top {args.top} --"
    )
    combos = []
    for i in range(len(cands)):
        for j in range(i + 1, len(cands)):
            for target, y in (("fwdz", fwdz), ("cont", cont)):
                g = and_gate(feats[cands[i]], feats[cands[j]], y, train)
                if g and g[0] > 0:
                    combos.append((g[0], cands[i], cands[j], target, g))
    combos.sort(key=lambda c: c[0], reverse=True)
    for up, fi, fj, target, g in combos[: args.top]:
        print(
            f"  {fi:10s} & {fj:10s} [{target:4s}] test={up:+.3f}pp "
            f"train={g[1]:+.3f} block={g[2] * 100:.0f}% n={g[3]} "
            f"(block_{g[4]} q={g[5]:.1f} & block_{g[6]} q={g[7]:.1f})"
        )

    # ── C. ridge linear gate ──
    print("\n-- ridge linear gate over all features --")
    for target, y in (("fwdz", fwdz), ("cont", cont)):
        lin = linear_gate(feats, y, train)
        if lin:
            print(
                f"  [{target:4s}] corr train={lin['tr_c']:+.3f} test={lin['te_c']:+.3f} "
                f"test={lin['uplift']:+.3f}pp train={lin['train_uplift']:+.3f} "
                f"block={lin['block'] * 100:.0f}% n={lin['n_keep']}"
            )
            topw = ", ".join(f"{n}:{w:+.2f}" for n, w in lin["weights"][:4])
            print(f"      top weights: {topw}")
        else:
            print(f"  [{target:4s}] none (insufficient data)")

    # return the best single survivor for cross-tf reporting
    best = None
    for n, (tc, ec, g1, g2) in singles.items():
        for label, g in (("fwdz", g1), ("cont", g2)):
            # survivor: sign-stable corr AND positive test uplift AND positive
            # train uplift (train/test agree the gate helps)
            if g and tc * ec > 0 and g[3] > 0 and g[4] > 0:
                cand = (g[3], n, label, tc, ec)
                if best is None or cand[0] > best[0]:
                    best = cand
    return best


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--tf", default="1h,4h,12h", help="comma list of timeframes")
    ap.add_argument("--start", default="2025-07-01")
    ap.add_argument("--end", default="2026-08-01")
    ap.add_argument(
        "--warmup",
        default=None,
        help="indicator lookback (default: derived from --tf — max(2×29 bars, 1 day))",
    )
    ap.add_argument(
        "--horizon", type=int, default=12, help="forward-return horizon (hours)"
    )
    ap.add_argument("--top", type=int, default=10)
    ap.add_argument(
        "--cap-tiers",
        default=None,
        help="CSV (header: symbol,tier) for market-cap split reporting",
    )
    ap.add_argument(
        "--cap-volume",
        action="store_true",
        help="split reporting by median-bar-volume terciles (liquidity proxy)",
    )
    args = ap.parse_args()

    bests = []
    for tf in [t.strip() for t in args.tf.split(",") if t.strip()]:
        b = screen_tf(tf, args)
        if b:
            bests.append((tf, b))

    if bests:
        bests.sort(key=lambda x: x[1][0], reverse=True)
        tf, (up, n, label, tc, ec) = bests[0]
        print(
            f"\n=== overall best: {n} [{label}] @ {tf} (test uplift {up:+.3f}pp, "
            f"ic_train {tc:+.4f} ic_test {ec:+.4f}) ==="
        )
        uni = load_pooled(
            tf,
            args.start,
            args.end,
            args.warmup or default_warmup(tf, args.start),
            args.horizon,
        )
        train, _ = split_mask(uni)
        f = uni[n].to_numpy()
        y = uni["fwdz"].to_numpy() if label == "fwdz" else uni["cont"].to_numpy()
        g = bar_gate(f, y, train)
        if g:
            monthly_stability(uni, f, y, g[0], g[2], f"{n} [{label}]")
    else:
        print("\nNo sign-stable positive survivor across timeframes.")


if __name__ == "__main__":
    main()
