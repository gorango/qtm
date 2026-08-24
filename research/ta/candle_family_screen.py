#!/usr/bin/env python3
"""Stability-gate every candlestick_reversal pattern in one pass.

Each `screen.py --stability --config '{"pattern": ...}'` invocation reloads
the full universe; this loads ONCE and runs all 21 patterns through the same
protocol numbers (H=12h, 10 bps round-trip, whole-window net bps +
lottery share + positive-month count).

Usage (from research/ta/):
    uv run python candle_family_screen.py \
        [--start 2025-07-01] [--end 2026-08-01]
"""

from __future__ import annotations

import argparse
import os
import sys

_HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, _HERE)

import screen as S

PATTERNS = [
    "hammer",
    "inverted_hammer",
    "hanging_man",
    "shooting_star",
    "spinning_top",
    "long_legged_doji",
    "dragonfly_doji",
    "gravestone_doji",
    "bullish_harami",
    "bearish_harami",
    "piercing_line",
    "dark_cloud_cover",
    "tweezer_bottom",
    "tweezer_top",
    "three_white_soldiers",
    "three_black_crows",
    "three_inside_up",
    "three_inside_down",
    "three_outside_up",
    "three_outside_down",
    "abandoned_baby",
]

HORIZON, COST = 12, 10.0


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--start", default="2025-07-01")
    ap.add_argument("--end", default="2026-08-01")
    args = ap.parse_args()

    uni = S.load_universe("1h", args.start, args.end)
    fwd, anchor, ts_by = S.precompute(uni, HORIZON)
    ranges = S.month_ranges(uni["timestamp"].to_numpy())

    print(
        f"\ncandlestick_reversal family — {args.start}..{args.end} 1h, "
        f"H={HORIZON}h, cost={COST:.0f} bps"
    )
    print(
        f"{'pattern':22s} {'net bps':>9s} {'trades':>7s} {'lottery%':>9s} "
        f"{'pos/neg':>8s}"
    )
    rows = []
    for p in PATTERNS:
        sigs = S.signals_by_symbol("candlestick_reversal", uni, {"pattern": p})
        nb, n = S.net_bps(fwd, anchor, ts_by, sigs, ranges, COST)
        rets = S.trade_rets(fwd, anchor, ts_by, sigs, ranges)
        share = S.lottery_share(rets)
        pos = neg = 0
        for (_y, _m), lo, hi in ranges:
            m_nb, _ = S.net_bps(
                fwd, anchor, ts_by, sigs, [((_y, _m), lo, hi)], COST
            )
            if m_nb > 0:
                pos += 1
            elif m_nb < 0:
                neg += 1
        print(
            f"{p:22s} {nb:+9.2f} {n:7d} {share * 100:8.0f}% {pos:>3d}/{neg:<3d}",
            flush=True,
        )
        rows.append((p, nb, n, share))

    print("\nSorted by net bps:")
    for p, nb, n, share in sorted(rows, key=lambda r: -r[1]):
        flag = ""
        if n < 100:
            flag = "  (<100 trades)"
        elif share > 0.9:
            flag = "  (lottery)"
        print(f"  {p:22s} {nb:+8.2f} ({n:5d}){flag}")


if __name__ == "__main__":
    main()
