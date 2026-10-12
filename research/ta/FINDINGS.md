# FINDINGS — catalog defect audit & the Aug-18 pattern campaign

Two lines of work in this directory share one record:

1. **Catalog defect audit** (Part 1) — root-cause investigation of the
   strategies that never fired, and the Aug-18 chart-pattern detectors that
   fired implausibly rarely.
2. **Aug-18 pattern campaign** (Part 2) — the ~30 new pattern detectors and
   their 12 strategy wrappers run through the standard pipeline: baseline XS
   rank-IC screen → stability gate → optuna sweep with held-out forward
   arbiter → correlation.

Universe/window/cost as everywhere else in this dir: `config/symbols.yaml`
universe, 1h bars, default window 2025-07-01..2026-08-01, H=12h, 10 bps
round-trip.  Provenance: the campaign ran in the parent repo's validation tree
(postgres `price_bars`); every number reproduces from this directory over
Binance OHLCV — `binance_loader.load_price_bars` implements the identical
loader contract, and the scripts listed here are the ported originals.

Scripts: `screen.py --strategies/--stability/--sweep` (canonical),
`candle_family_screen.py` (21-pattern family, one universe load),
`candle_flip_oos.py` (contrarian-flip OOS test),
`sweep_candle_flip.py` (flipped-objective sweep).

> **Status: all defects below are FIXED in the current source tree.**  The
> per-bug tables are a root-cause record, not an open bug list.  In a screen,
> `--`/zero now genuinely means "tested and weak", not "broken".

---

# Part 1 — Catalog defect audit

Date: 2026-08-15.  Updated: 2026-08-23 (Aug-18 pattern-detector batch — see
below).  Source: the PyPI `quantamental` wheel (snake_case registry, v0.1.0)
and this repository.  Verified on real 1h data (BTC, 2025-07..2026-08, full
OHLCV) and on targeted synthetic patterns.

11 strategies never fired in the 97-strategy baseline.  Root causes, grouped
by class.  Fixes are in the source tree (verified by inspection + real-data
re-screen); the cup/wedge regression tests live inline in their source files
(`crates/indicators/src/patterns/{cup_and_handle,wedges}.rs`, `#[cfg(test)]`).
Rebuilt wheel verified on BTC 1h: super_trend 460 signals (was 0),
donchian_breakout 1965, kst 593 (indicator no longer all-NaN),
vwap_ema_rsi_trend 532, double_top_stochastic_reversal 336, elliott_wave 14,
triangles indicator 357.  Class C needs no Rust fix — the pair strategies take
the second series through **config** (`secondCloses`); the screen must pass it
instead of skipping (screen change).

## Class A — hard logic bugs (arithmetically impossible to fire)

| strategy | root cause | evidence |
|---|---|---|
| `super_trend` | **Band-ratchet bug.** Both final bands ratchet against `super_trend_values[i-1]` (a single line) instead of their own previous values (`final_upper = basic_upper.min(prev_st)`, and prev_st is the *lower* band during an uptrend → upper collapses to lower). Direction freezes at +1; a flip needs a bar with `range > 6×ATR`, but the observed max over 13 months is `3.2×ATR`. | Canonical TradingView supertrend flips 431× on the same data; this one flips 0–1×. `close` is above the band on 100% of bars. |
| `donchian_breakout` | **Channel includes the current bar.** `rolling_max_growing`/`rolling_min_growing` window is `values[i+1-window_size..=i]`, so `upper[i]` contains `close[i]` → `close[i] > upper[i]` is mathematically impossible (same for the lower band). Classic turtle compares against the *prior* window (`upper[i-1]`). | `crossed_over_series(closes, upper, i)` can never be true. |
| `kst` | **NaN-poisoned `sma_internal`.** `sma_internal` used a running accumulator (`sum += values[i]`), so the first NaN (ROC's warmup) made `sum` NaN **forever** → KST and signal all-NaN → no crossovers. Shared helper bug — latent for any indicator fed a NaN-warmup series (qstick, random_index, …). | `kst`/`signal` are NaN on 9418/9418 bars. CCI/stoch/BB survive because their inputs are finite from bar 0. |
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
breakout) to be usable as history-wide detectors.  (Done 2026-08-15 for
`cup_and_handle`'s scan; 2026-08-23 found a second, independent cup bug — see
the Aug-18 batch below — plus the same whole-series disease in `wedges`.)

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
non-causal fit bug, see the Aug-18 batch below.)

## Aug-18 batch — pattern-detector defects

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
bottom)` — pullback as a fraction of the cup's advance.  Current source:
`crates/indicators/src/patterns/cup_and_handle.rs`.  Regression tests added
(`detects_cup_and_handle_breakout`, `rejects_deep_handle`).  Post-fix, BTC
fires scale monotonically with relaxation (depth .08→1, .04→6, .02+dur60→26);
universe screen goes 3 → 131 signal bars, IC −0.014 (t=−1.21) —
testable-but-thin, mildly negative.  Defaults (depth 0.15 within ±10 bars of
an order-5 trough) remain strict-for-intraday; that is now tuning, not defect.

This supersedes the original 2026-08-15 status note "`cup_and_handle` fires
with a sensible `min_duration` config" — synthetic fixtures passed because
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
Classification semantics unchanged.  Current source:
`crates/indicators/src/patterns/wedges.rs`.  Tests: rising/falling fixtures
with exact fire bars + flat-series negative (`detects_rising_wedge_breakdown`,
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

### Threshold-starved, not broken — `diamond` / `broadening`

Working but threshold-starved (zero fires at defaults on BTC at any tf; appear
when tolerance→0).  Root design weakness: OLS "trendlines" fitted through
order-1 pivots (strictly-greater than immediate neighbors — every 3-bar
wiggle) fill windows with noise, and OLS lines through that cloud rarely
satisfy the ±divergence tolerances; the exact-cross event compounds it.  Unit
tests hide this via smooth synthetic fixtures.  Tunable via config; the deeper
fix would be higher-order pivots, not attempted.  Treat any future use as
needing pivot-quality work first.

### Audited clean (no action)

Candlestick family — all 21 detectors: signs follow bullish=+1 / bearish=−1,
end-anchored on the completing bar (signal knowable at close, no lookahead),
and the strategy wrapper passes detector signs through untouched.  Same
conventions verified for diamond, broadening, rectangle, rounding,
triple_top_bottom, channels, bump_and_run, head_and_shoulders.  Soft spot
noted: harami omits its classical prior-trend precondition (selectivity loss,
not a correctness bug).

## Fixes landed

1. `super_trend`: each final band now ratchets against its **own** previous
   value (`fu[i-1]`, `fl[i-1]`), line picked by prior regime (canonical form).
2. `donchian_breakout`: compares against the **prior** window (`upper[i-1]`) —
   channel shifted by one bar (classic turtle).
3. `sma_internal`: window sum no longer a running accumulator — it
   tracks `nan_in_window` and skips/removes NaNs, so a NaN warmup no longer
   poisons the sum.
4. `vwap_ema_rsi_trend`: RSI gate realigned with the crossover direction.
5. `double_top_stochastic_reversal`: sign fixed (`double_top == -1`) and the
   stoch gate loosened so confluence is reachable.
6. `elliott_wave_pattern`: impulse indexer rebuilt on 3 troughs (w1-start,
   w2-low, w4-low) + 3 peaks; corrective on 2.
7. `triangles` / `cup_and_handle`: sliding-window scan instead of
   end-anchored take(3)/take(4).  (Done 2026-08-15 for cup_and_handle's scan;
   2026-08-23 found a second, independent cup bug — see the Aug-18 batch — plus
   the same whole-series disease in `wedges`, both fixed.)
8. py bindings: `secondCloses` (and friends) accepted so the statistics trio is
   reachable.  Remaining work is the **screen change** — pass a reference
   symbol as the second series instead of skipping (Class C).
9. `cup_and_handle`: handle-retracement formula → `(rim − handle_low) /
   (rim − bottom)`; inline regression tests.
10. `wedges`: causal rolling-window rewrite + normalized slopes + `lookback`
    param plumbed through config, wrapper, and pyo3/napi bindings; inline tests.

Residual: Class C still needs the screen to pass the second series; the screen
currently reports Class A/B as `--` (0 bars) and Class C as "needs 2-asset
input (skipped)".

**Post-2026-08-23 corollary:** the inverse failure mode is real too — a
detector that fires a handful of times may be broken rather than rare
(`wedges` "worked" for weeks while emitting only tail artifacts).  Before
reading any sparse screen row, check WHERE the fires land in each symbol's
series (tail-clustering = anchoring bug) and whether every candidate dies at
one specific gate (unsatisfiable predicate).

---

# Part 2 — Aug-18 pattern campaign results

Scope: the pattern-detector indicators and their 12 strategy wrappers added in
c8f019f/701a417, run through the standard pipeline (baseline screen →
stability gate → sweep with held-out forward arbiter → correlation).  Detector
defects found during the run are in Part 1; this part is the campaign outcome.

## Verdict table

| strategy | screen h12 IC (bars) | gate | final |
|---|---|---|---|
| bump_and_run_reversal | +0.0046 (1806) | **killed** — net −7.86, top-3 = 596% (§1.4) | dead |
| candlestick_reversal (hammer) | +0.0025 (5581) | passed (+6.20 bps, 27% lot, 8/13 mo) | **sweep kill**: best train +36.11 → fwd −65.12; defaults also died fwd (+25.7→−21.2) |
| channels_breakout | −0.0047 (4920) | n/a — sub-cost anti-predictor | dead |
| rounding_reversal | −0.0032 (7059) | n/a | dead |
| triple_top_bottom_reversal | −0.0053 (3472) | n/a | dead |
| rectangle_breakout | −0.0019 (1356) | n/a | dead |
| flags_pennants_continuation | −0.0100 (470) | n/a | dead |
| wedge_breakout | untestable (8 bars) | detector defect, fixed (Part 1) | post-fix: −0.0015 (8093 bars) — dense-sample kill |
| cup_and_handle_breakout | untestable (3 bars) | detector defect, fixed (Part 1) | post-fix: −0.0141 (131 bars) — thin, negative-leaning |
| diamond_reversal | untestable (2 bars) | threshold-starved on noisy pivots, not fixed | untested-as-implemented |
| broadening_breakout | untestable (5 bars) | same | untested-as-implemented |
| island_reversal | untestable (3 bars) | instrument-inapplicable (no OHLC gaps in 24/7 perps at any tf) | closed |

Family structure (`--correlate`, 7 dense members): max pairwise corr 0.06,
entry-agreement ≤3% — fully orthogonal, ~7 independent bets (contrast the
reversion family's 8→2-3 collapse).  Orthogonality was moot: no member had
edge to combine.

## Candlestick family (all 21 patterns, `candle_family_screen.py`)

Only hammer cleared the walk-forward bar (+6.20 net / 2152 trades / 27%
lottery / 8 of 13 months) and it died in the sweep.  Positives-by-defaults:
three_outside_up +3.35, three_outside_down +2.53, inverted_hammer +2.25
(57% lottery — not promotable).  Uniform bleeds with low lottery (real signal,
wrong sign): shooting_star −42.38 (1/12 mo), three_inside_up −25.07,
three_inside_down −19.15, tweezer_bottom −18.14.  spinning_top is a
non-signal: −6.49 over 20,083 trades.  abandoned_baby/piercing_line/
dark_cloud_cover/three_white_soldiers/three_black_crows fire <60 times —
no sample.

Construction audit of the bleed candidates found NO sign or anchoring bugs
(detectors are end-anchored on the completing bar, conventions match
bullish=+1/bearish=−1; harami omits its classical prior-trend precondition —
a selectivity gap, not a correctness bug).

## THE SURVIVOR: flipped shooting_star

The strongest uniform bleed reverses into the only candidate that ever passed
this pipeline's forward arbiter.  Object: candlestick_reversal
pattern=shooting_star with signs negated ("go long after an upside-wick
rejection following an advance" — fade the fade).  Mechanically consistent
with perp microstructure: the rejected-breakdown wick marks absorbed selling,
not exhaustion.

Evidence at DEFAULT params (identity flip_net == −orig_net − 2×cost verified
exactly on every evaluation):

| window | status | net bps | trades | lottery | months+ |
|---|---|---|---|---|---|
| discovery 2025-07..2026-08 | in-sample | +22.38 | 1,872 | 15% | 11/13 |
| 2023-01..2025-06 (`candle_flip_oos.py`) | untouched #1 | +10.12 | 2,926 | 17% | 17/29 |
| 2026-03..08 (flip-sweep arbiter) | untouched #2 | **+28.17** | 778 | **17%** | **5/6** |

Three windows positive after cost, ~5,700 trades, no window carried by
concentration.  Weak spots stated plainly: long-era month consistency 17/29
(59%) sits under the strict ≥⅔ bar; 2025-08/09 are losing months (−67.6,
−12.6); the usual small-cap cost-floor caveat applies; and the candidate was
selected as the extreme order statistic of ~40 sign combinations before
confirmation.  Mitigation for the selection concern: all 8 bleed candidates
were pre-registered together and tested on the untouched era in one pass —
shooting_star was not chosen within that window, and the 2026-03..08 pass is
a second independent confirmation.

Flipped-objective sweep (`sweep_candle_flip.py`, 300 TPE trials): best train
+82.96 → forward only +16.33 with **89% lottery share and 2/6 months** — a
kill under §1.4.  TPE collapsed onto one concentrated basin while the boring
defaults carry the actual (modest, robust) edge.  Optimization made the
candidate worse — the mirror image of volume_profile_rsi and hammer, where
the optimizer couldn't rescue defaults either.

Status: **promotable candidate, not promoted.**  Next stage: formalize into a
trading layer as an entry-gate/conditioning feature (constraint 1.5 — likely
worth more as a short-blocking/momentum-confirm feature than a standalone
both-sides signal), then run the pool-conditioned separator test in the
parent engine before any deployment claim.

## Detector fixes — changed files

At time of writing (2026-08-23) the fixes touched:
`crates/indicators/src/patterns/{wedges,cup_and_handle}.rs` (+ inline
regression tests incl. rejects-deep-handle and non-wedge-flat negatives),
`crates/strategies/src/types/configs.rs` (WedgeConfig.lookback),
`crates/strategies/src/patterns/wedge.rs`,
`bindings/{js,py}/src/indicators.rs` (lookback arg).  Full workspace tests +
clippy green; wheel rebuilt and revalidated against real data.

## Methodological notes for the record

1. A sign bug and a winning contrarian flip are observationally identical at
   aggregate level — audit construction BEFORE crediting a flip (here the
   audit came first and was clean).
2. Synthetic-fixture testing validates geometry but hides pivot-noise
   sensitivity; fire-rate-on-real-data should be a standard smoke test for
   rare-event detectors (a handful of fires may be broken, not rare — see
   Part 1's corollary).
3. Flip arithmetic (`flip_net == −orig_net − 2·cost`) held exactly in every
   evaluation — cheap invariant, worth asserting in any flip-based test.
