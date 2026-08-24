# Broken / non-firing quantamental strategies — root-cause investigation

Date: 2026-08-15.  Updated: 2026-08-23 (Aug-18 pattern-detector batch — see
below).  Source: the PyPI `quantamental` wheel (snake_case registry, v0.1.0)
and this repository (HEAD `6b1deca` + working-tree fixes).  Verified on real
1h data (BTC, 2025-07..2026-08, full OHLCV) and on targeted synthetic patterns.

## Status (2026-08-15)

**All Class A and Class B bugs are FIXED in the submodule source** (commits
`d78342b` indicators, `54cbbbb` strategies) with regression tests
(`crates/indicators/tests/broken_fixes.rs`,
`crates/strategies/tests/broken_strategy_fixes.rs`).  Rebuilt wheel verified
on BTC 1h: super_trend 460 signals (was 0), donchian_breakout 1965, kst 593
(indicator no longer all-NaN), vwap_ema_rsi_trend 532,
double_top_stochastic_reversal 336, elliott_wave 14, triangles indicator 357.
`cup_and_handle` fires with a sensible `min_duration` config (its default of
20 only fits tiny intraday cups; unit-tested).  Class C needs no Rust fix —
the pair strategies take the second series through **config**
(`secondCloses`); the screen must pass it instead of skipping (screen change).

11 strategies never fired in the 97-strategy baseline.  Root causes, grouped
by class:

## Class A — hard logic bugs (arithmetically impossible to fire)

| strategy | root cause | evidence |
|---|---|---|
| `super_trend` | **Band-ratchet bug.** Both final bands ratchet against `super_trend_values[i-1]` (a single line) instead of their own previous values (`final_upper = basic_upper.min(prev_st)`, and prev_st is the *lower* band during an uptrend → upper collapses to lower). Direction freezes at +1; a flip needs a bar with `range > 6×ATR`, but the observed max over 13 months is `3.2×ATR`. | Canonical TradingView supertrend flips 431× on the same data; this one flips 0–1×. `close` is above the band on 100% of bars. |
| `donchian_breakout` | **Channel includes the current bar.** `rolling_max_growing`/`rolling_min_growing` window is `values[i+1-window_size..=i]`, so `upper[i]` contains `close[i]` → `close[i] > upper[i]` is mathematically impossible (same for the lower band). Classic turtle compares against the *prior* window (`upper[i-1]`). | `crossed_over_series(closes, upper, i)` can never be true. |
| `kst` | **NaN-poisoned `sma_internal`.** `sma_internal` uses a running accumulator (`sum += values[i]`), so the first NaN (ROC's warmup) makes `sum` NaN **forever** → KST and signal are all-NaN → no crossovers. Shared helper bug — latent for any indicator fed a NaN-warmup series (qstick, random_index, …). | `kst`/`signal` are NaN on 9418/9418 bars. CCI/stoch/BB survive because their inputs are finite from bar 0. |
| `vwap_ema_rsi_trend` | **Self-contradictory confluence.** Buy = EMA5×EMA20 cross-up **AND** close > VWAP **AND** RSI < 30. A bullish cross + price above VWAP means momentum turned up — RSI can't be oversold. RSI gate is directionally inverted vs the trend conditions. | 288 cross-ups, 277 above VWAP, **0** with RSI<30 (RSI at cross-up bars: median 55, p10–p90 = 51–60). Sell side mirrored. |
| `double_top_stochastic_reversal` | **Sign mismatch + dead confluence.** (a) `double_top` indicator emits **-1.0** for a top, but the strategy checks `double_top[i] == 1.0` — sell branch can never fire. (b) Even for the buy branch: `double_bottom == 1` fires 187× but never co-occurs with stoch K ≤ 20 (0/187). | 0 signals across the window despite 187 bottoms / 207 tops detected. |
| `elliott_wave_pattern` | **Structurally mis-modeled impulse.** The detector indexes **5 troughs per sequence** (t0→p1→t1→p2→t2→p3→t3→p4→t4 = 4 up-legs + 4 down-legs), but a real 5-wave impulse has only 3 troughs (w1-start, w2-low, w4-low). With the extra required "w3 low" and "w5 low" that don't exist, and retracements ~1.0 on any alternating structure vs the 0.618±10% gate, it can never match. | 0 on clean synthetics, noisy synthetics, and all real data. |

## Class B — end-anchored detection (only ever examines the most recent pattern)

These scan only the *tail* of the series, so over a full-history pass they
almost never fire (a pattern mid-history is invisible):

| strategy | root cause |
|---|---|
| `triangles` | Takes the **last 4 peaks** after an arbitrary `index > len*0.3` cutoff, regresses those (often contaminated by post-pattern breakout bars), then requires a breakout in the remaining bars after the last peak. |
| `cup_and_handle` | Examines only the **last 3 troughs** of the entire series (`troughs.iter().rev().take(3)`), then requires the breakout after them. A cup that completed earlier is never detected. |

Both would need a sliding-window scan (scan every pattern position, fire on the
breakout) to be usable as history-wide detectors.

## Class C — reachable via config, screen must pass the second series

| strategy | issue |
|---|---|
| `cointegration_pair_trading` | Need a **second asset series**, passed through **config** (`secondCloses`), not the input dict.  `StrategyInput` has no second-series field, but `config.second_closes` works — the py binding accepts `{'secondCloses': [...]}`.  The screen skips them; it should pass a reference symbol (e.g. BTC) as the second series. |
| `correlation_mean_reversion` | same |
| `correlation_pair_trading` | same |

(`pairs_trading` works — it is a self-contained z-score entry/exit on a single
asset, not a two-series strategy.)

## Not broken — just genuinely rare (fired a handful of times)

`chande_forecast_oscillator` (2 bars), `ma_rsi_trend_following` (4),
`vwap_stochastic_confirmation` (16),
`triangle_breakout` (18), `buy_and_hold` (31), `vwap_rsi_breakout` (53).
These emit, but their conditions are rare on 1h crypto; treat as noise rows,
not failures.  (`wedge_breakout` was listed here as "13 fires" on
2026-08-15 — reclassified on 2026-08-23: those were tail artifacts of the
non-causal fit bug, see the update section below.)

## Update 2026-08-23 — Aug-18 pattern-detector batch

The 2026-08-18 commits added ~30 pattern detectors (broadening, bump_and_run,
candlesticks ×21, channels, diamond, island_reversal, rectangle, rounding,
triple_top_bottom) and 12 grouped strategies.  Running them through the
protocol produced implausibly sparse samples for five formations — universe
totals over ~13 months × 110 symbols at 1h: diamond **2** bars, island **3**,
cup_and_handle **3**, broadening **5**, wedge **8** — against dense siblings
(rounding 7059, channels 4920, candlesticks 5581).  Source audit plus
stage-by-stage replication of each detector's gates on real BTC OHLCV (1h +
resampled 4h/1d) found two real bugs, one instrument mismatch, and two
over-thresholded-but-correct detectors.

### New Class A — `cup_and_handle`: unsatisfiable handle-retracement gate — FIXED

`retracement = (handle_high − handle_low) / (rim − handle_low)` compares the
handle window's own range against its drawdown-from-rim.  But the window
starts one bar after the right shoulder (the local top), so its high sits near
the rim by construction and the ratio → 1.0 for any realistic pullback;
passing ≤0.3 requires a degenerate flat drift under the rim — a handle with no
pullback in it.

Evidence: on BTC the depth gate passes up to 586 candidate cups per
parameterization and **every one dies at the retrace gate** — 0 fires at every
config tried, including `cupDepth=0.01, minDuration=120` at 1h, 4h and 1d.
Not threshold-tunable; the metric itself was wrong.

Fix: handle depth redefined classically as `(rim − handle_low) / (rim −
bottom)` — pullback as a fraction of the cup's advance.  Regression tests
added (`detects_cup_and_handle_breakout`, `rejects_deep_handle`).  Post-fix,
BTC fires scale monotonically with relaxation (depth .08→1, .04→6,
.02+dur60→26); universe screen goes 3 → 131 signal bars, IC −0.014 (t=−1.21)
— testable-but-thin, mildly negative.  Defaults (depth 0.15 within ±10 bars of
an order-5 trough) remain strict-for-intraday; that is now tuning, not defect.

This supersedes the 2026-08-15 status note "`cup_and_handle` fires with a
sensible `min_duration` config" — synthetic fixtures passed because
`ohlc_from_series` builds smooth series where handles are flat by
construction; every real-data cup failed the gate.

### New Class B — `wedges`: whole-series non-causal fit — FIXED

The detector computed pivots once against an arbitrary `len*0.3` cutoff, took
the last-N peaks/troughs **of the entire series**, fitted one line pair, then
scanned for a breakout only from the last pivot to series end.  Consequences:
(a) signals can only exist in each symbol's final bars — evidence: exactly 1
fire on BTC 9476×1h bars, at position **1.00** (the literal last bar); all 8
universe events were tail artifacts; (b) results depend on future data extent
(the same historical bar classifies differently as more data arrives);
(c) bonus: `slope_tolerance` compared unnormalized price-per-bar slopes, so
the threshold meant different things for BTC vs a $0.02 alt.

Fix: rewritten as a causal rolling-window scan mirroring `broadening`/
`diamond` — per bar, fit to the most recent `min_points` pivots inside a
trailing `lookback` window (new param, default 120; plumbed through
`WedgeConfig`, the strategy wrapper, and both pyo3/napi bindings); slopes
normalized by mean pivot price; single-bar cross with prev-bar-inside check.
Classification semantics unchanged.  Tests: rising/falling fixtures with
exact fire bars + flat-series negative (`detects_rising_wedge_breakdown`,
`detects_falling_wedge_breakout`, `no_signal_without_wedge_geometry`).

Post-fix: BTC 1h → 308 fires spread across the series (median position 0.54),
82 at 4h, 6 at 1d; universe screen 8 → **8093** bars, IC −0.0015 (t=−1.29).
The strategy is now properly testable — and cleanly dead — instead of
accidentally empty.

### Instrument-inapplicable — `island_reversal`

Requires true OHLC gaps (`highs[i] < lows[i−1]` / mirror).  Perps trade
continuously, so bars derived from 1m data never gap at any resolution —
incidence is structurally zero regardless of parameters.  Implementation
verified correct on gapped synthetics (fires as documented).  Not fixable by
tuning: the pattern does not exist in this universe.  Dropped from screening.

### Audited clean (no action)

Candlestick family — all 21 detectors: signs follow bullish=+1 / bearish=−1,
end-anchored on the completing bar (signal knowable at close, no lookahead),
and the strategy wrapper passes detector signs through untouched.  Same
conventions verified for diamond, broadening, rectangle, rounding,
triple_top_bottom, channels, bump_and_run, head_and_shoulders.  The sparse
members (diamond, broadening) are threshold-starved rather than broken:
order-1 pivots (strictly-greater than immediate neighbors — every 3-bar
wiggle) fill windows with noise, OLS lines through that cloud rarely satisfy
the ±divergence tolerances, and the exact-cross event compounds it.  Tunable
via config; deeper fix would be higher-order pivots, not attempted here.
Soft spot noted: harami omits its classical prior-trend precondition
(selectivity loss, not a correctness bug).

### Batch verdicts (for the record)

Baseline screen → stability gate → sweeps killed every natural-direction
path: bump_and_run_reversal (lottery: −7.86 bps net, top-3-trade share 596%);
candlestick_reversal/hammer (sweep kill: best train +36.11 bps → forward
**−65.12**; defaults +25.72 → −21.21 forward; TPE collapsed onto one basin);
wedge_breakout post-fix (dense sample, zero edge).  The seven dense detectors
are mutually orthogonal (max pairwise corr 0.06, agreement ≤3% — no
reversion-family duplication).  One contrarian line survived: shooting_star
flipped passed pre-registered out-of-window testing (+22.38 bps discovery /
+10.12 held-out 2023-01..2025-06 era, ~17% lottery both) and then the
flipped-objective sweep's own arbiter at DEFAULT params (+28.17 bps forward
2026-03..08, 778 trades, 17% lottery, 5/6 months) — three windows positive
after cost.  The sweep's optimized config itself died under constraint 1.4
(train +82.96 → forward +16.33 but 89% top-3 concentration, 2/6 months:
TPE collapsed onto a concentrated basin while defaults carry the real edge).
Full record: `findings-pattern-batch.md`.  Status: promotable candidate, pending FSM
formalization + separator test (constraint 1.5).

## Suggested upstream fixes (in priority order)

1. `super_trend`: ratchet each final band against its **own** previous value
   (`fu[i-1]`, `fl[i-1]`), and pick the line by prior regime (canonical form).
2. `donchian_breakout`: compare against the **prior** window (`upper[i-1]`) —
   shift the channel by one bar (classic turtle does this).
3. `sma_internal`: recompute the window sum per index (skip-NaN or slice sum)
   instead of a running accumulator, or clear `sum` when NaN enters/leaves.
4. `vwap_ema_rsi_trend`: make the RSI gate agree with the crossover direction
   (e.g. RSI > 50 on buys / < 50 on sells) or drop it.
5. `double_top_stochastic_reversal`: fix the sign (`double_top == -1`), and
   loosen the stoch gate so confluence is reachable.
6. `elliott_wave_pattern`: rebuild the impulse indexer on 3 troughs (w1-start,
   w2-low, w4-low) + 3 peaks; corrective on 2.
7. `triangles` / `cup_and_handle`: sliding-window scan instead of
   end-anchored take(3)/take(4).  (Done 2026-08-15 for cup_and_handle's scan;
   2026-08-23 found a second, independent cup bug — see Class A above — plus
   the same whole-series disease in `wedges`, both now fixed.)
8. py bindings: add `secondCloses` (and friends) to `StrategyInput` so the
   statistics trio is reachable.
9. `cup_and_handle` (2026-08-23, FIXED in working tree): handle-retracement
   formula → `(rim − handle_low) / (rim − bottom)`; regression tests.
10. `wedges` (2026-08-23, FIXED in working tree): causal rolling-window
    rewrite + normalized slopes + `lookback` param plumbed through config,
    wrapper, and pyo3/napi bindings; tests.

Until fixed, the screen reports these correctly: Class A/B show `--` (0 bars)
and Class C shows "needs 2-asset input (skipped)" — but treat `--` as
"strategy never fired", not "signal tested and weak".

Post-2026-08-23 corollary: the inverse failure mode is real too — a detector
that fires a handful of times may be broken rather than rare (`wedges`
"worked" for weeks while emitting only tail artifacts).  Before reading any
sparse screen row, check WHERE the fires land in each symbol's series
(tail-clustering = anchoring bug) and whether every candidate dies at one
specific gate (unsatisfiable predicate).
