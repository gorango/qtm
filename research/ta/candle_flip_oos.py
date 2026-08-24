#!/usr/bin/env python3
"""Contrarian-flip OOS test for uniformly-bleeding candlestick patterns.

The stability screen finds several candlestick_reversal patterns losing
consistently on the discovery window. A sign-flip of a losing signal is the
SAME observation viewed from the other side, so any flip alpha must be
confirmed out-of-sample. This evaluates each pre-registered candidate BOTH
ways (orig / flipped) on a held-out era no prior screen touched, and
sanity-checks the flip arithmetic against the identity
flip_net == -orig_net - 2*cost.

Usage (from research/ta/):
    uv run python candle_flip_oos.py \
        [--discovery-start 2025-07-01 --discovery-end 2026-08-01] \
        [--oos-start 2023-01-01 --oos-end 2025-06-30]
"""

from __future__ import annotations

import argparse
import os
import sys

_HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, _HERE)

import numpy as np

import screen as S

# Candidates pre-registered by their discovery-era bleeds (orig net bps,
# 2025-07..2026-08 defaults screen — see findings-pattern-batch.md).
CANDIDATES = [
    ("shooting_star", -42.38),
    ("three_inside_up", -25.07),
    ("three_inside_down", -19.15),
    ("tweezer_bottom", -18.14),
    ("tweezer_top", -7.70),
    ("spinning_top", -6.49),
    ("bullish_harami", -6.19),
    ("bearish_harami", -6.49),
]

HORIZON, COST = 12, 10.0


def eval_window(uni, pattern: str):
    fwd, anchor, ts_by = S.precompute(uni, HORIZON)
    ranges = S.month_ranges(uni["timestamp"].to_numpy())
    sigs = S.signals_by_symbol("candlestick_reversal", uni, {"pattern": pattern})
    out = {}
    for mode in ("orig", "flip"):
        use = (
            {s: (-v).astype(np.int8) for s, v in sigs.items()}
            if mode == "flip"
            else sigs
        )
        nb, n = S.net_bps(fwd, anchor, ts_by, use, ranges, COST)
        rets = S.trade_rets(fwd, anchor, ts_by, use, ranges)
        share = S.lottery_share(rets)
        pos = sum(
            1
            for (_y, _m), lo, hi in ranges
            if S.net_bps(fwd, anchor, ts_by, use, [((_y, _m), lo, hi)], COST)[0] > 0
        )
        out[mode] = (nb, n, share, pos, len(ranges))
    return out


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--discovery-start", default="2025-07-01")
    ap.add_argument("--discovery-end", default="2026-08-01")
    ap.add_argument("--oos-start", default="2023-01-01")
    ap.add_argument("--oos-end", default="2025-06-30")
    args = ap.parse_args()

    print(
        f"Loading discovery {args.discovery_start}..{args.discovery_end}...",
        file=sys.stderr,
        flush=True,
    )
    uni_d = S.load_universe("1h", args.discovery_start, args.discovery_end)
    print(f"Loading OOS {args.oos_start}..{args.oos_end}...", file=sys.stderr, flush=True)
    uni_o = S.load_universe("1h", args.oos_start, args.oos_end)

    print(
        f"\n{'pattern':20s} {'window':10s} {'mode':6s} {'net bps':>9s} "
        f"{'trades':>7s} {'lottery':>8s} {'months+':>8s}"
    )
    for pat, _disc_nb in CANDIDATES:
        for label, uni in (("discovery", uni_d), ("OOS", uni_o)):
            r = eval_window(uni, pat)
            for mode in ("orig", "flip"):
                nb, n, share, pos, nm = r[mode]
                ident = ""
                if mode == "flip":
                    expect = -r["orig"][0] - 2 * COST
                    ident = (
                        "  [== -o-2c]"
                        if abs(nb - expect) < 0.05 or r["orig"][1] == 0
                        else f"  [MISMATCH exp {expect:+.2f}]"
                    )
                mark = " *" if nb > 0 else ""
                print(
                    f"{pat:20s} {label:10s} {mode:6s} {nb:+9.2f} {n:7d} "
                    f"{share * 100:7.0f}% {pos:>3d}/{nm - 1:<3d}{ident}{mark}",
                    flush=True,
                )


if __name__ == "__main__":
    main()
