#!/usr/bin/env python3
"""Flipped-objective optuna sweep for a candlestick_reversal pattern.

The catalog's --sweep maximizes the strategy's own signs. For a contrarian
candidate established by candle_flip_oos.py, the object of interest is the
NEGATED signal, so this sweep maximizes net bps of the FLIPPED signal over
the train months only, holds out the last 6 months as the arbiter, and
applies screen.py's exact min-trades penalty. Everything else (universe,
horizon, cost, TPE seed) matches `screen.py --sweep`.

NOTE on evaluation: the `pattern` selector is NOT part of optimization_bounds,
so it must be pinned explicitly in EVERY config — including baselines and any
re-evaluation of trial.params (trial.params carries only the suggested numeric
keys; dropping the pin silently evaluates pattern=hammer instead).

Usage (from research/ta/):
    uv run python sweep_candle_flip.py \
        --pattern shooting_star [--trials 300] [--start ... --end ...]
"""

from __future__ import annotations

import argparse
import os
import sys

_HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, _HERE)

import numpy as np

import screen as S

STRATEGY = "candlestick_reversal"
TF = "1h"
HORIZON, COST = 12, 10.0
MIN_TRADES = 100
FORWARD_MONTHS = 6


def flip(sigs: dict[str, np.ndarray]) -> dict[str, np.ndarray]:
    return {s: (-v).astype(np.int8) for s, v in sigs.items()}


def main() -> None:
    import optuna

    ap = argparse.ArgumentParser()
    ap.add_argument("--pattern", default="shooting_star")
    ap.add_argument("--trials", type=int, default=300)
    ap.add_argument("--start", default="2025-07-01")
    ap.add_argument("--end", default="2026-08-01")
    args = ap.parse_args()
    pattern = args.pattern

    optuna.logging.set_verbosity(optuna.logging.WARNING)
    defaults = S.q.get_strategy_defaults()[STRATEGY]
    base_params = dict(defaults.get("params", {}))
    bounds = defaults.get("optimization_bounds", [])

    uni = S.load_universe(TF, args.start, args.end)
    fwd, anchor, ts_by = S.precompute(uni, HORIZON)
    ranges = S.month_ranges(uni["timestamp"].to_numpy())
    train_ranges = ranges[: len(ranges) - FORWARD_MONTHS]
    fwd_ranges = ranges[len(ranges) - FORWARD_MONTHS :]

    def eval_cfg(cfg):
        cfg = {**cfg, "pattern": pattern}  # pin: trial.params omits it
        sigs = flip(S.signals_by_symbol(STRATEGY, uni, cfg))
        tr, nt = S.net_bps(fwd, anchor, ts_by, sigs, train_ranges, COST)
        fw, nf = S.net_bps(fwd, anchor, ts_by, sigs, fwd_ranges, COST)
        return tr, nt, fw, nf, sigs

    btr, bnt, bfw, bnf, _ = eval_cfg(base_params)
    print(
        f"\nSweep {STRATEGY}[{pattern}] FLIPPED — {args.start}..{args.end} {TF}, "
        f"H={HORIZON}h, cost={COST:.0f} bps, {args.trials} trials, "
        f"min-trades={MIN_TRADES}"
    )
    print(
        f"  months: {len(train_ranges)} train ({train_ranges[0][0]}..{train_ranges[-1][0]}) "
        f"| {len(fwd_ranges)} held-out forward ({fwd_ranges[0][0]}..{fwd_ranges[-1][0]})"
    )
    print(
        f"  baseline (default params, flipped): train {btr:+.2f} bps ({bnt}) "
        f"| FORWARD {bfw:+.2f} bps ({bnf})"
    )

    def objective(trial):
        cfg = S.suggest_config(trial, base_params, bounds)
        cfg["pattern"] = pattern
        sigs = flip(S.signals_by_symbol(STRATEGY, uni, cfg))
        nb, n = S.net_bps(fwd, anchor, ts_by, sigs, train_ranges, COST)
        if n < MIN_TRADES:
            frac = n / MIN_TRADES
            nb = nb * frac - 20.0 * (1.0 - frac)
        return nb

    cache_dir = os.path.join(_HERE, "_cache")
    os.makedirs(cache_dir, exist_ok=True)
    storage = f"sqlite:///{cache_dir}/sweep_{pattern}_flip.db"
    study_kwargs = {
        "direction": "maximize",
        "sampler": optuna.samplers.TPESampler(seed=42),
    }
    study = optuna.create_study(
        storage=storage,
        study_name=f"flip_{pattern}",
        load_if_exists=True,
        **study_kwargs,
    )

    def _cb(s, trial):
        if trial.number % max(1, args.trials // 10) == 0:
            print(
                f"  trial {trial.number}/{args.trials}: best train {s.best_value:+.2f} bps",
                file=sys.stderr,
                flush=True,
            )

    try:
        study.optimize(objective, n_trials=args.trials, callbacks=[_cb])
    except KeyboardInterrupt:
        print("\n  interrupted — reporting best-so-far", file=sys.stderr)

    # report DISTINCT configs by objective value; re-eval with pattern pinned
    cands = [t for t in study.trials if t.state == optuna.trial.TrialState.COMPLETE]
    seen, uniq = set(), []
    for t in sorted(cands, key=lambda t: t.value or -1e9, reverse=True):
        key = tuple(sorted(t.params.items()))
        if key not in seen:
            seen.add(key)
            uniq.append(t)

    print(f"\n  Top distinct flipped-{pattern} trials:")
    best_sigs = None
    for rank, trial in enumerate(uniq[:5], 1):
        tr, nt, fw, nf, sigs = eval_cfg(trial.params)
        print(
            f"  #{rank} (obj {trial.value:+7.2f}): train {tr:+7.2f} ({nt:4d}) "
            f"| FORWARD {fw:+7.2f} ({nf:4d})   {trial.params}"
        )
        if rank == 1:
            best_sigs = sigs
            rets_f = S.trade_rets(fwd, anchor, ts_by, sigs, fwd_ranges)
            pos = sum(
                1
                for (_y, _m), lo, hi in fwd_ranges
                if S.net_bps(fwd, anchor, ts_by, sigs, [((_y, _m), lo, hi)], COST)[0] > 0
            )
            print(
                f"      forward lottery {S.lottery_share(rets_f) * 100:.0f}% "
                f"| positive forward months {pos}/{len(fwd_ranges)}"
            )

    print("\n  Best-trial per-month net bps (flipped; >=6/9 positive = gate):")
    pos = neg = 0
    for label, rng in [("train", train_ranges), ("forward", fwd_ranges)]:
        for (y, m), lo, hi in rng:
            nb, n = S.net_bps(fwd, anchor, ts_by, best_sigs, [((y, m), lo, hi)], COST)
            mark = "+" if nb > 0 else " "
            if nb > 0:
                pos += 1
            else:
                neg += 1
            print(f"    {label[0]} {y}-{m:02d}: {nb:+8.2f} bps ({n:4d}) {mark}")
    print(f"    positive months: {pos}, negative: {neg}")

    # flip-arithmetic identity check on the best config
    cfg = {**uniq[0].params, "pattern": pattern}
    tr_o, _ = S.net_bps(
        fwd,
        anchor,
        ts_by,
        S.signals_by_symbol(STRATEGY, uni, cfg),
        train_ranges,
        COST,
    )
    tr_f, _ = S.net_bps(fwd, anchor, ts_by, best_sigs, train_ranges, COST)
    print(
        f"\n  identity check: flipped train {tr_f:+.2f} vs -orig-2c {-tr_o - 2 * COST:+.2f}"
    )
    print(
        "\n  Judgment: the held-out forward window is the arbiter. A positive "
        "forward number with high lottery share or few positive months is a "
        "kill under constraint 1.4, not a pass."
    )


if __name__ == "__main__":
    main()
