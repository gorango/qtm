#!/usr/bin/env python3
"""Screen the quantamental strategy catalog against the shared validation
protocol (see README.md — "Screening against the validation protocol").

Runs each selected strategy's discrete signal (+1/-1/0) from the native Rust
package over the symbols.yaml 1h universe and reports the per-bar
cross-sectional Spearman rank IC vs timestamp-based forward returns at
6/12/24 hours — the protocol's first kill test (constraint 1: raw 1h
cross-sectional rank IC ceiling ~0.03-0.04; anything at or below the 10 bps
cost floor is dead on arrival).

This is a screen (necessary condition), not a promotion path.  A follow
survivor must still pass market-neutral long-short, overlap-free sim, the
walk-forward gate, and the shared-pool replay.

Usage (from research/traditional/):
    uv run python screen.py                     # curated default set
    uv run python screen.py --all               # all 97 strategies
    uv run python screen.py --category trend,volume
    uv run python screen.py --strategies ma-crossover,rsi
    uv run python screen.py --sweep rsi --trials 300   # optuna sweep
"""

from __future__ import annotations

import argparse
import os
import sys

import numpy as np
import polars as pl

_HERE = os.path.dirname(os.path.abspath(__file__))
_RESEARCH = os.path.abspath(os.path.join(_HERE, "..", ".."))
_VALIDATION = os.path.abspath(os.path.join(_HERE, ".."))
for _p in (_HERE, _RESEARCH, _VALIDATION):
    if _p not in sys.path:
        sys.path.insert(0, _p)

import quantamental as q  # noqa: E402

HOUR_MS = 3600000
# Reference symbol for pair/statistics strategies that need a second series.
REF_SYMBOL = "BTC/USDT:PERP"
# 2025-07-01+ is the known-complete window: continuous live collection since
# the orderbook era.  The older backfill (2024-01-01 back) is still seeding and
# may have partial months.
START, END = "2025-07-01", "2026-08-01"
HORIZONS = [6, 12, 24]


def _tf_ms(tf: str) -> int:
    """Bar length in ms for a tf string like 15m/1h/4h/12h/1d."""
    unit = tf[-1]
    n = int(tf[:-1])
    if unit == "m":
        return n * 60_000
    if unit == "h":
        return n * 3_600_000
    if unit == "d":
        return n * 86_400_000
    raise ValueError(f"unsupported tf: {tf}")


class NeedsTwoAssetsError(Exception):
    """Raised for strategies that require a second asset series (pair/statistics)."""


DEFAULT_STRATEGIES = [
    # trend
    "ma_crossover",
    "macd",
    "super_trend",
    "dmi",
    "parabolic_sar",
    "hma_trend",
    "vortex",
    "aroon",
    # momentum
    "rsi",
    "stochastic",
    "cci",
    "williams_r",
    "momentum",
    "roc",
    # volatility
    "bollinger_bands",
    "bollinger_bands_mean_reversion",
    "donchian_breakout",
    "donchian_reversion",
    "keltner_channel_breakout",
    "keltner_channel_reversion",
    "z_score_breakout",
    "z_score_reversion",
    # volume
    "obv",
    "chaikin_money_flow",
    "force_index",
    "money_flow_index",
    "volume_weighted_average_price",
    "vwap_reversion",
    # patterns
    "head_and_shoulders_reversal",
    "double_top_bottom_reversal",
    "triangle_breakout",
    # composite
    "ma_rsi_trend_following",
    "macd_rsi_momentum",
    "bb_rsi_breakout",
]


def rank_ic(frame: pl.DataFrame, sig_col: str, fwd_col: str) -> dict:
    """Per-bar cross-sectional Spearman rank IC of signal vs forward return.

    Vectorized: per-timestamp average-ranks → Pearson correlation via polars
    group_by (spearmanr per bar was the screen's bottleneck).  Bars where the
    signal or forward return is constant across symbols yield null corr and are
    skipped, as are bars with <20 finite symbols.
    """
    df = (
        frame.select(["timestamp", sig_col, fwd_col])
        .filter(pl.col(fwd_col).is_finite() & pl.col(fwd_col).is_not_null())
        .with_columns(
            pl.len().over("timestamp").alias("_n"),
            pl.col(sig_col).n_unique().over("timestamp").alias("_nsig"),
            pl.col(fwd_col).n_unique().over("timestamp").alias("_nfwd"),
        )
        # skip bars where the signal (or forward return) is constant across
        # symbols — pl.corr returns ~1e-16 noise instead of NaN for a
        # constant column, which would pass is_finite and pollute the ICs
        # with fake ~0 bars (huge t-stats on all-zero signals).
        .filter((pl.col("_n") >= 20) & (pl.col("_nsig") >= 2) & (pl.col("_nfwd") >= 2))
    )
    if df.is_empty():
        return {"mean": np.nan, "t": np.nan, "bars": 0}
    r = (
        df.with_columns(
            pl.col(sig_col).rank().over("timestamp").alias("_rs"),
            pl.col(fwd_col).rank().over("timestamp").alias("_rf"),
        )
        .group_by("timestamp")
        .agg(pl.corr("_rs", "_rf").alias("ic"))
        .filter(pl.col("ic").is_finite())
    )
    ics = r["ic"].to_numpy()
    v = np.asarray(ics, dtype=float)
    v = v[np.isfinite(v)]
    if v.size == 0:
        return {"mean": np.nan, "t": np.nan, "bars": 0}
    sem = v.std(ddof=1) / np.sqrt(v.size) if v.size > 1 else np.nan
    return {
        "mean": v.mean(),
        "t": v.mean() / sem if sem and np.isfinite(sem) else np.nan,
        "bars": v.size,
    }


def fwd_cols(sig: pl.DataFrame, horizons) -> dict[int, np.ndarray]:
    """Timestamp-based forward returns per symbol (gap-safe; never dense
    array indexing)."""
    ts = sig["timestamp"].to_numpy()
    close = sig["close"].to_numpy().astype(np.float64)
    sym = sig["fsym"].to_numpy()
    out = {h: np.full(len(sig), np.nan) for h in horizons}
    starts = np.flatnonzero(sym[1:] != sym[:-1]) + 1
    bounds = np.concatenate(([0], starts, [len(sig)]))
    for a, b in zip(bounds[:-1], bounds[1:]):
        t = ts[a:b]
        c = close[a:b]
        for h in horizons:
            idx = np.searchsorted(t, t + h * HOUR_MS, side="right") - 1
            fwd = np.where(idx >= 0, c[np.clip(idx, 0, len(c) - 1)], np.nan)
            out[h][a:b] = fwd / c - 1.0
    return out


def load_universe(tf: str, start: str, end: str) -> pl.DataFrame:
    from binance_loader import coverage_report, load_price_bars

    print(
        f"Loading {tf} price_bars {start}..{end} from binance...",
        file=sys.stderr,
        flush=True,
    )
    data = load_price_bars(start=start, end=end, tf=tf)
    coverage_report(data)
    frames = []
    for sym in sorted(data):
        frames.append(
            data[sym].with_columns(
                pl.Series("fsym", np.full(data[sym].height, sym, dtype=object))
            )
        )
    return pl.concat(frames, rechunk=False)


def strategy_signals(strategy_id: str, uni: pl.DataFrame) -> pl.DataFrame:
    """Run one strategy over the whole universe; return a signal-column frame
    (fsym, timestamp, signal) — row order preserved from ``uni``."""
    sig = signals_by_symbol(strategy_id, uni, None)
    ts = uni["timestamp"].to_numpy()
    sym = uni["fsym"].to_numpy()
    out = np.zeros(len(uni), dtype=np.int8)
    starts = np.flatnonzero(sym[1:] != sym[:-1]) + 1
    bounds = np.concatenate(([0], starts, [len(uni)]))
    for a, b in zip(bounds[:-1], bounds[1:]):
        out[a:b] = sig[sym[a]]
    return pl.DataFrame({"fsym": sym, "timestamp": ts, "signal": out})


def run_strategy_symbol(
    strategy_id: str,
    df: pl.DataFrame,
    ts: np.ndarray,
    ref_ts: np.ndarray,
    ref_close: np.ndarray,
    config: dict | None,
) -> np.ndarray:
    """Run one strategy over ONE symbol's bars; ``ts`` must match ``df`` row
    order.  Pair/statistics strategies get the reference symbol's closes as the
    second series (as-of aligned via ``ref_ts``/``ref_close``) — the py
    bindings only expose it through config.  Kept as a separate function so
    streaming paths (gate_separators) never materialize the concatenated
    universe."""
    inp = {
        "opens": df["open"].to_numpy(),
        "highs": df["high"].to_numpy(),
        "lows": df["low"].to_numpy(),
        "closes": df["close"].to_numpy(),
        "volumes": df["volume"].to_numpy(),
        "timestamps": ts,
    }
    try:
        return np.asarray(q.run_strategy(strategy_id, inp, config), dtype=np.int8)
    except ValueError as e:
        if "second" not in str(e) or ref_ts.size == 0:
            raise NeedsTwoAssetsError(
                f"{strategy_id} requires a second asset series ({str(e)})"
            ) from e
        # retry with the reference symbol's closes as the second series,
        # as-of aligned to this symbol's bars (gap-safe)
        idx = np.searchsorted(ref_ts, ts, side="right") - 1
        idx = np.clip(idx, 0, ref_close.size - 1)
        cfg2 = dict(config or {})
        cfg2["secondCloses"] = ref_close[idx]
        return np.asarray(q.run_strategy(strategy_id, inp, cfg2), dtype=np.int8)


def signals_by_symbol(
    strategy_id: str, uni: pl.DataFrame, config: dict | None
) -> dict[str, np.ndarray]:
    """Run one strategy over the universe with an optional config dict; return
    {symbol: int8 signal array} in the row order of ``uni``.  Slices the
    concatenated universe per symbol; the per-symbol work is in
    ``run_strategy_symbol``."""
    ts = uni["timestamp"].to_numpy()
    sym = uni["fsym"].to_numpy()
    close = uni["close"].to_numpy()
    starts = np.flatnonzero(sym[1:] != sym[:-1]) + 1
    bounds = np.concatenate(([0], starts, [len(uni)]))
    by_sym = [(sym[a], a, b) for a, b in zip(bounds[:-1], bounds[1:])]

    ref_mask = sym == REF_SYMBOL
    ref_ts = ts[ref_mask]
    ref_close = close[ref_mask]

    sigs: dict[str, np.ndarray] = {}
    for s, a, b in sorted(by_sym, key=lambda x: x[0]):
        sub = uni.slice(a, b - a)
        sigs[s] = run_strategy_symbol(
            strategy_id, sub, ts[a:b], ref_ts, ref_close, config
        )
    return sigs


# ── Optuna sweep ────────────────────────────────────────────────────────────


def precompute(uni: pl.DataFrame, horizon: int):
    """Per-symbol forward returns (timestamp-based), hold-horizon anchors, and
    timestamps — computed once, shared by every sweep trial."""
    ts = uni["timestamp"].to_numpy()
    sym = uni["fsym"].to_numpy()
    close = uni["close"].to_numpy().astype(np.float64)
    starts = np.flatnonzero(sym[1:] != sym[:-1]) + 1
    bounds = np.concatenate(([0], starts, [len(uni)]))
    fwd: dict[str, np.ndarray] = {}
    anchor: dict[str, np.ndarray] = {}
    ts_by: dict[str, np.ndarray] = {}
    for a, b in zip(bounds[:-1], bounds[1:]):
        s = sym[a]
        t = ts[a:b]
        c = close[a:b]
        idx = np.searchsorted(t, t + horizon * HOUR_MS, side="right") - 1
        f = np.where(idx >= 0, c[np.clip(idx, 0, len(c) - 1)], np.nan) / c - 1.0
        fwd[s] = f
        ts_by[s] = t
        anchor[s] = np.arange(b - a) % horizon == 0
    return fwd, anchor, ts_by


def month_ranges(ts_all: np.ndarray) -> list[tuple[tuple[int, int], int, int]]:
    """(year, month) → [month start ms, month end ms], sorted."""
    from datetime import datetime, timezone

    months = sorted(
        {
            (
                datetime.fromtimestamp(t / 1000, tz=timezone.utc).year,
                datetime.fromtimestamp(t / 1000, tz=timezone.utc).month,
            )
            for t in ts_all
        }
    )
    ranges = []
    for y, m in months:
        lo = int(datetime(y, m, 1, tzinfo=timezone.utc).timestamp() * 1000)
        hi = (
            int(
                datetime(
                    y + (1 if m == 12 else 0),
                    (1 if m == 12 else m + 1),
                    1,
                    tzinfo=timezone.utc,
                ).timestamp()
                * 1000
            )
            - 1
        )
        ranges.append(((y, m), lo, hi))
    return ranges


def trade_rets(fwd, anchor, ts_by, sigs, ranges) -> np.ndarray:
    """Per-trade returns of the both-sides long(+1)/short(-1) signal, overlap-free
    (every-horizon bar anchor).  Longs + flipped shorts, concatenated."""
    rets = []
    for s, sig in sigs.items():
        f = fwd[s]
        a = anchor[s]
        t = ts_by[s]
        fin = np.isfinite(f)
        for _, lo, hi in ranges:
            m = (t >= lo) & (t <= hi)
            rets.append(f[m & a & (sig > 0) & fin])
            rets.append(-f[m & a & (sig < 0) & fin])
    if not rets:
        return np.array([])
    return np.concatenate(rets)


def net_bps(fwd, anchor, ts_by, sigs, ranges, cost: float) -> tuple[float, int]:
    """Net bps after cost of the both-sides long(+1)/short(-1) signal, overlap-
    free (every-horizon bar anchor).  This is the protocol's market-neutral
    long-short in its discrete-signal form.  Returns (net_bps, n_trades)."""
    r = trade_rets(fwd, anchor, ts_by, sigs, ranges)
    if len(r) < 20:
        return 0.0, len(r)
    return float(np.mean(r)) * 10000 - cost, len(r)


def lottery_share(rets: np.ndarray) -> float:
    """Share of total PnL from the top-3 trades (constraint 1.4: lottery-alpha
    is the norm — a >100% share means the rest of the book loses)."""
    if len(rets) < 3:
        return 0.0
    total = float(rets.sum())
    if total <= 0:
        return 0.0
    top3 = float(np.sort(rets)[-3:].sum())
    return top3 / total


def net_bps_by_tier(
    fwd, anchor, ts_by, sigs, ranges, cost: float, tiers: dict[str, str]
) -> dict[str, tuple[float, int]]:
    """net_bps split by market-cap tier (symbol → tier from --cap-tiers CSV)."""
    out: dict[str, tuple[float, int]] = {}
    tier_syms: dict[str, list[str]] = {}
    for s in sigs:
        tier_syms.setdefault(tiers.get(s, "?"), []).append(s)
    for tier in sorted(tier_syms):
        sub = {s: sigs[s] for s in tier_syms[tier]}
        out[tier] = net_bps(fwd, anchor, ts_by, sub, ranges, cost)
    return out


def suggest_config(trial, base_params: dict, bounds: list) -> dict:
    """Sample one config from the strategy's optimization_bounds (min/max/step
    map 1:1 to Optuna suggestions); unspecified params keep the defaults."""
    cfg = dict(base_params)
    for b in bounds:
        name = b["param_name"]
        step = b["step"]
        if float(step).is_integer():
            cfg[name] = trial.suggest_int(
                name, int(b["min"]), int(b["max"]), step=int(step)
            )
        else:
            cfg[name] = trial.suggest_float(name, b["min"], b["max"], step=step)
    return cfg


def run_sweep(
    strategy_id: str,
    uni: pl.DataFrame,
    tf: str,
    start: str,
    end: str,
    args,
) -> None:
    import optuna

    optuna.logging.set_verbosity(optuna.logging.WARNING)
    defaults = q.get_strategy_defaults().get(strategy_id)
    if defaults is None:
        print(f"Unknown strategy: {strategy_id}", file=sys.stderr)
        return
    base_params = dict(defaults.get("params", {}))
    bounds = defaults.get("optimization_bounds", [])
    horizon = args.horizon
    cost = args.cost

    fwd, anchor, ts_by = precompute(uni, horizon)
    ts_all = uni["timestamp"].to_numpy()
    ranges = month_ranges(ts_all)
    train_ranges = ranges[: len(ranges) - args.forward_months]
    fwd_ranges = ranges[len(ranges) - args.forward_months :]
    tiers = load_tiers(args.cap_tiers) if args.cap_tiers else {}

    base_sigs = signals_by_symbol(strategy_id, uni, base_params)
    base_train, base_n = net_bps(fwd, anchor, ts_by, base_sigs, train_ranges, cost)
    base_fwd, base_nf = net_bps(fwd, anchor, ts_by, base_sigs, fwd_ranges, cost)
    print(
        f"\nSweep {strategy_id} — {start}..{end} {tf}, H={horizon}h, "
        f"cost={cost:.0f} bps round-trip, {args.trials} trials, "
        f"min-trades={args.min_trades}"
    )
    print(
        f"  months: {len(train_ranges)} train ({train_ranges[0][0]}..{train_ranges[-1][0]}) "
        f"| {len(fwd_ranges)} held-out forward ({fwd_ranges[0][0]}..{fwd_ranges[-1][0]})"
    )
    if not bounds:
        print("  (no optimization_bounds — every trial equals defaults)")
    print(
        f"  baseline (default params): train {base_train:+.2f} bps ({base_n} trades) "
        f"| forward {base_fwd:+.2f} bps ({base_nf})"
    )

    def objective(trial):
        cfg = suggest_config(trial, base_params, bounds)
        sigs = signals_by_symbol(strategy_id, uni, cfg)
        nb, n = net_bps(fwd, anchor, ts_by, sigs, train_ranges, cost)
        if n < args.min_trades:
            # smooth penalty: edge scaled by coverage, minus a floor discount so
            # a sparse/zero-trade config can never win by scoring ~0.0
            frac = n / args.min_trades
            nb = nb * frac - 20.0 * (1.0 - frac)
        return nb

    study_kwargs = dict(
        direction="maximize", sampler=optuna.samplers.TPESampler(seed=42)
    )
    if args.storage:
        study = optuna.create_study(
            storage=args.storage,
            study_name=f"sweep_{strategy_id}_{start}_{end}",
            load_if_exists=True,
            **study_kwargs,
        )
    else:
        study = optuna.create_study(**study_kwargs)

    checkpoint = os.path.join(_HERE, "_cache", f"sweep_{strategy_id}_best.json")

    def _cb(s, trial):
        if trial.number % max(1, args.trials // 10) == 0:
            print(
                f"  trial {trial.number}/{args.trials}: best train "
                f"{s.best_value:+.2f} bps",
                file=sys.stderr,
                flush=True,
            )
            if s.best_params:
                import json

                with open(checkpoint, "w") as fh:
                    json.dump(s.best_params, fh, indent=2)
                print(
                    f"    best params -> {checkpoint}",
                    file=sys.stderr,
                )

    print("  optimizing...", file=sys.stderr, flush=True)
    try:
        study.optimize(objective, n_trials=args.trials, callbacks=[_cb])
    except KeyboardInterrupt:
        print(
            "\n  interrupted — reporting best-so-far (params also on disk)",
            file=sys.stderr,
            flush=True,
        )

    def eval_cfg(cfg) -> tuple[float, int, float, int]:
        sigs = signals_by_symbol(strategy_id, uni, cfg)
        tr, nt = net_bps(fwd, anchor, ts_by, sigs, train_ranges, cost)
        fw, nf = net_bps(fwd, anchor, ts_by, sigs, fwd_ranges, cost)
        return tr, nt, fw, nf

    # best + top-5 by train (re-run signals for each)
    cands = []
    for trial in study.trials:
        if trial.state == optuna.trial.TrialState.COMPLETE:
            cands.append(trial)
    cands.sort(key=lambda t: t.value, reverse=True)
    if not cands:
        print("  no completed trials — nothing to report")
        return

    print(f"\n  Best {strategy_id} trial (by train net bps):")
    best = cands[0]
    tr, nt, fw, nf = eval_cfg(best.params)
    best_sigs = signals_by_symbol(strategy_id, uni, best.params)
    print(f"    params: {best.params}")
    print(f"    train {tr:+.2f} bps ({nt})  |  FORWARD {fw:+.2f} bps ({nf})")

    if tiers:
        train_tiers = net_bps_by_tier(
            fwd, anchor, ts_by, best_sigs, train_ranges, cost, tiers
        )
        fwd_tiers = net_bps_by_tier(
            fwd, anchor, ts_by, best_sigs, fwd_ranges, cost, tiers
        )
        print("    per-tier net bps (train | forward):")
        for tier in sorted(train_tiers):
            tn, nn = train_tiers[tier]
            fn, nfn = fwd_tiers.get(tier, (float("nan"), 0))
            print(f"      {tier:>5s}: {tn:+7.2f} ({nn:4d}) | {fn:+7.2f} ({nfn:4d})")

    print("\n  Top-5 by train (train | FORWARD):")
    for i, trial in enumerate(cands[:5], 1):
        tr, nt, fw, nf = eval_cfg(trial.params)
        print(
            f"    {i}. train {tr:+7.2f} bps ({nt:4d}) | FORWARD {fw:+7.2f} bps ({nf:4d})"
            f"   {trial.params}"
        )

    print("\n  Best-trial per-month net bps (both windows; >=6/9 positive = gate):")
    pos = neg = 0
    for label, rng in [("train", train_ranges), ("forward", fwd_ranges)]:
        for (y, m), lo, hi in rng:
            nb, n = net_bps(fwd, anchor, ts_by, best_sigs, [((y, m), lo, hi)], cost)
            mark = "+" if nb > 0 else " "
            if nb > 0:
                pos += 1
            else:
                neg += 1
            print(f"    {label[0]} {y}-{m:02d}: {nb:+.2f} bps ({n:3d}) {mark}")
    print(f"    positive months: {pos}, negative: {neg}")
    print(
        "\n  Sweep judgment: the held-out forward window is the arbiter.  If "
        "best forward bps is not positive after cost, the strategy is not "
        "promotable regardless of train performance."
    )


def load_tiers(path: str) -> dict[str, str]:
    """Load --cap-tiers CSV (header: symbol,tier) into {symbol: tier}."""
    import csv

    tiers: dict[str, str] = {}
    with open(path) as fh:
        for row in csv.DictReader(fh):
            tiers[row["symbol"]] = row["tier"]
    return tiers


def signals_column(
    strategy_id: str, uni: pl.DataFrame, config: dict | None
) -> np.ndarray:
    """Signal array for one strategy in ``uni`` row order (numpy int8)."""
    sigs = signals_by_symbol(strategy_id, uni, config)
    sym = uni["fsym"].to_numpy()
    out = np.zeros(len(uni), dtype=np.int8)
    starts = np.flatnonzero(sym[1:] != sym[:-1]) + 1
    bounds = np.concatenate(([0], starts, [len(uni)]))
    for a, b in zip(bounds[:-1], bounds[1:]):
        out[a:b] = sigs[sym[a]]
    return out


def run_stability(
    strategy_id: str, uni: pl.DataFrame, tf: str, start: str, end: str, args
) -> None:
    """Per-month net bps + trade counts for one strategy, plus the lottery-alpha
    share (constraint 1.4) — the protocol's stability gate before any optuna."""
    import json

    config = json.loads(args.config) if args.config else None
    horizon = args.horizon
    cost = args.cost

    fwd, anchor, ts_by = precompute(uni, horizon)
    ranges = month_ranges(uni["timestamp"].to_numpy())
    sigs = signals_by_symbol(strategy_id, uni, config)

    rets_all = trade_rets(fwd, anchor, ts_by, sigs, ranges)
    nb_all, n_all = net_bps(fwd, anchor, ts_by, sigs, ranges, cost)
    share = lottery_share(rets_all)

    print(
        f"\nStability {strategy_id} — {start}..{end} {tf}, H={horizon}h, "
        f"cost={cost:.0f} bps round-trip"
    )
    if config:
        print(f"  config: {config}")
    print(
        f"  whole window: {nb_all:+.2f} bps net ({n_all} trades), "
        f"top-3-trade PnL share {share * 100:.0f}%"
    )
    print("  per-month net bps (net of cost) + trades:")
    for (y, m), lo, hi in ranges:
        nb, n = net_bps(fwd, anchor, ts_by, sigs, [((y, m), lo, hi)], cost)
        mark = "+" if nb > 0 else " "
        print(f"    {y}-{m:02d}: {nb:+7.2f} bps ({n:5d}) {mark}")
    print(
        "\n  Gate reads: per-month spread = stability; >=6/9 positive months = "
        "walk-forward gate; a top-3 share near/exceeding 100% = lottery-alpha, "
        "not edge.  A survivor can then be swept (--sweep) and judged on its "
        "held-out forward window."
    )


def run_correlate(
    ids: list[str], uni: pl.DataFrame, tf: str, start: str, end: str
) -> None:
    """Pairwise correlation + entry-agreement between strategies' discrete
    signals — the 'how many independent bets is this really' question.

    corr = Pearson correlation of the -1/0/+1 series (all bars, both
    directions); agreement = share of bars where both fire the SAME direction.
    A family that looks like 8 strategies but correlates >0.8 is 1-2 bets.
    """
    print(
        f"\nSignal correlation — {start}..{end} {tf}, {len(ids)} strategies:",
        file=sys.stderr,
    )
    cols: dict[str, np.ndarray] = {}
    for sid in ids:
        cols[sid] = signals_column(sid, uni, None)
        print(f"  {sid}", file=sys.stderr)

    n = len(ids)
    corr = np.eye(n)
    agree = np.eye(n)
    for i in range(n):
        for j in range(i + 1, n):
            a, b = cols[ids[i]], cols[ids[j]]
            both = (a != 0) | (b != 0)
            if both.sum() == 0:
                continue
            corr[i, j] = corr[j, i] = float(np.corrcoef(a, b)[0, 1])
            agree[i, j] = agree[j, i] = float(((a != 0) & (a == b)).sum() / both.sum())

    w = max(len(x) for x in ids) + 2
    print(f"\n{'':>{w}} " + "".join(f"{x[:9]:>10}" for x in ids))
    for i, a in enumerate(ids):
        row = f"{a[: w - 2]:>{w}} "
        for j in range(n):
            if i == j:
                row += f"{'--':>10}"
            else:
                row += f"{corr[i, j]:>10.2f}"
        print(row)

    print("\nEntry-agreement (share of bars both fire the same direction):")
    print(f"{'':>{w}} " + "".join(f"{x[:9]:>10}" for x in ids))
    for i, a in enumerate(ids):
        row = f"{a[: w - 2]:>{w}} "
        for j in range(n):
            if i == j:
                row += f"{'--':>10}"
            else:
                row += f"{agree[i, j]:>10.2f}"
        print(row)
    print(
        "\n  Interpretation: high corr/agreement = same bet twice; keep one.  "
        "Low-correlation survivors are the candidates for the separator/gate "
        "test — the constraint-1.5 mechanism, not portfolio stacking."
    )


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--strategies", type=str, default=",".join(DEFAULT_STRATEGIES))
    ap.add_argument("--category", type=str, default=None)
    ap.add_argument("--all", action="store_true")
    ap.add_argument(
        "--tf", type=str, default="1h", help="bar timeframe (1h/4h/12h/15m)"
    )
    ap.add_argument("--start", type=str, default=START)
    ap.add_argument("--end", type=str, default=END)
    ap.add_argument(
        "--sweep",
        type=str,
        default=None,
        metavar="STRATEGY",
        help="optuna-optimize one strategy's params (see --trials/--horizon/...)",
    )
    ap.add_argument("--trials", type=int, default=200, help="sweep: optuna trials")
    ap.add_argument(
        "--horizon", type=int, default=12, help="sweep: forward-return horizon (h)"
    )
    ap.add_argument(
        "--cost", type=float, default=10.0, help="sweep: round-trip cost (bps)"
    )
    ap.add_argument(
        "--forward-months",
        type=int,
        default=6,
        help="sweep: last N months held out as the forward-window arbiter",
    )
    ap.add_argument(
        "--min-trades",
        type=int,
        default=100,
        help="sweep: trade-count floor; configs below it get a smooth penalty",
    )
    ap.add_argument(
        "--cap-tiers",
        type=str,
        default=None,
        metavar="CSV",
        help="sweep: CSV (symbol,tier) to break results out by market-cap tier",
    )
    ap.add_argument(
        "--storage",
        type=str,
        default=None,
        help="sweep: optuna storage URL (sqlite:///...) to persist/resume",
    )
    ap.add_argument(
        "--stability",
        type=str,
        default=None,
        metavar="STRATEGY",
        help="per-month net bps + lottery-alpha for one strategy (stability gate)",
    )
    ap.add_argument(
        "--correlate",
        action="store_true",
        help="pairwise signal correlation/agreement across --strategies",
    )
    ap.add_argument(
        "--config",
        type=str,
        default=None,
        metavar="JSON",
        help="stability: config dict (e.g. '{\"period\": 12}') instead of defaults",
    )
    args = ap.parse_args()

    global HOUR_MS
    HOUR_MS = _tf_ms(args.tf)
    start, end = args.start, args.end

    uni = load_universe(args.tf, start, end)
    print(f"  {len(uni)} rows, {uni['fsym'].n_unique()} symbols", file=sys.stderr)

    if args.all:
        ids = sorted(q.get_strategy_registry()["strategies"])
    elif args.category:
        cats = args.category.split(",")
        reg = q.get_strategy_registry()["strategies"]
        ids = sorted(sid for sid, m in reg.items() if m["category"] in cats)
    else:
        ids = [s.strip() for s in args.strategies.split(",") if s.strip()]

    if args.sweep:
        run_sweep(args.sweep, uni, args.tf, start, end, args)
        return
    if args.stability:
        run_stability(args.stability, uni, args.tf, start, end, args)
        return
    if args.correlate:
        run_correlate(ids, uni, args.tf, start, end)
        return

    fwd = fwd_cols(uni, HORIZONS)

    print(
        f"\nquantamental strategy screen — {start}..{end}, {args.tf}, "
        f"binance OHLCV, cost floor 10 bps round-trip"
    )
    print("Per-bar cross-sectional Spearman rank IC vs fwd ret (mean / t / bars):")
    header = f"{'strategy':30s} " + " ".join(f"{'h' + str(h):>20s}" for h in HORIZONS)
    print(header)
    print("-" * len(header))

    rows = []
    for sid in ids:
        try:
            sig = strategy_signals(sid, uni)
        except NeedsTwoAssetsError:
            print(f"{sid:30s} {'needs 2-asset input (skipped)':63s}")
            rows.append((sid, np.nan, 0))
            continue
        cells = []
        for h in HORIZONS:
            ic_frame = sig.select(["timestamp", "signal"]).with_columns(
                pl.Series("fwd", fwd[h])
            )
            r = rank_ic(ic_frame, "signal", "fwd")
            if r["bars"] == 0:
                cells.append((0.0, 0, "    --    --   0"))
            else:
                cells.append(
                    (
                        r["mean"],
                        r["bars"],
                        f"{r['mean']:7.4f} {r['t']:7.2f} {r['bars']:5d}",
                    )
                )
        print(f"{sid:30s} " + " ".join(c[2] for c in cells))
        rows.append((sid, cells[1][0], cells[1][1]))  # h12 IC, bars

    print("\nSorted by 12h rank IC:")
    for sid, ic12, bars in sorted(
        (r for r in rows if np.isfinite(r[1])), key=lambda x: -x[1]
    ):
        print(f"  {sid:30s} {ic12:+.4f}  ({bars} bars)")

    print(
        "\nKill-test note: mean IC at/under the 10 bps cost band is the norm "
        "for raw 1h cross-sectional signals (validation README constraint 1); "
        "positive survivors must still pass the long-short / overlap-free / "
        "walk-forward protocol before any pool work."
    )


if __name__ == "__main__":
    main()
