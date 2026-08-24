# FINDINGS — new chart-pattern batch (Aug-18 detectors + strategy wrappers)

Scope: the pattern-detector indicators and their 12 strategy wrappers added
in c8f019f/701a417 (through `6b1deca` plus the local detector fixes described
below), run through the standard pipeline: baseline XS rank-IC screen →
stability gate → optuna sweep with held-out forward arbiter → correlation.
Universe/window/cost as everywhere else in this dir: symbols.yaml universe,
1h bars, default window 2025-07-01..2026-08-01, H=12h, 10 bps round-trip.

Provenance: the campaign ran in the parent repo's validation tree (postgres
`price_bars`); every number below reproduces from this directory over Binance
OHLCV — `binance_loader.load_price_bars` implements the identical loader
contract, and the scripts listed here are the ported originals.

Scripts: `screen.py --strategies/--stability/--sweep` (canonical),
`candle_family_screen.py` (21-pattern family, one universe load),
`candle_flip_oos.py` (contrarian-flip OOS test),
`sweep_candle_flip.py` (flipped-objective sweep).

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
| wedge_breakout | untestable (8 bars) | detector defect, fixed (see below) | post-fix: −0.0015 (8093 bars) — dense-sample kill |
| cup_and_handle_breakout | untestable (3 bars) | detector defect, fixed (see below) | post-fix: −0.0141 (131 bars) — thin, negative-leaning |
| diamond_reversal | untestable (2 bars) | threshold-starved on noisy pivots, not fixed | untested-as-implemented |
| broadening_breakout | untestable (5 bars) | same | untested-as-implemented |
| island_reversal | untestable (3 bars) | instrument-inapplicable (no OHLC gaps in 24/7 perps at any tf) | closed |

Family structure (`--correlate`, 7 dense members): max pairwise corr 0.06,
entry-agreement ≤3% — fully orthogonal, ~7 independent bets (contrast the
reversion family's 8→2-3 collapse). Orthogonality was moot: no member had
edge to combine.

## Candlestick family (all 21 patterns, `candle_family_screen.py`)

Only hammer cleared the walk-forward bar (+6.20 net / 2152 trades / 27%
lottery / 8 of 13 months) and it died in the sweep. Positives-by-defaults:
three_outside_up +3.35, three_outside_down +2.53, inverted_hammer +2.25
(57% lottery — not promotable). Uniform bleeds with low lottery (real signal,
wrong sign): shooting_star −42.38 (1/12 mo), three_inside_up −25.07,
three_inside_down −19.15, tweezer_bottom −18.14. spinning_top is a
non-signal: −6.49 over 20,083 trades. abandoned_baby/piercing_line/
dark_cloud_cover/three_white_soldiers/three_black_crows fire <60 times —
no sample.

Construction audit of the bleed candidates found NO sign or anchoring bugs
(detectors are end-anchored on the completing bar, conventions match
bullish=+1/bearish=−1; harami omits its classical prior-trend precondition —
a selectivity gap, not a correctness bug).

## THE SURVIVOR: flipped shooting_star

The strongest uniform bleed reverses into the only candidate that ever passed
this pipeline's forward arbiter. Object: candlestick_reversal
pattern=shooting_star with signs negated ("go long after an upside-wick
rejection following an advance" — fade the fade). Mechanically consistent
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
concentration. Weak spots stated plainly: long-era month consistency 17/29
(59%) sits under the strict ≥⅔ bar; 2025-08/09 are losing months (−67.6,
−12.6); the usual small-cap cost-floor caveat applies; and the candidate was
selected as the extreme order statistic of ~40 sign combinations before
confirmation. Mitigation for the selection concern: all 8 bleed candidates
were pre-registered together and tested on the untouched era in one pass —
shooting_star was not chosen within that window, and the 2026-03..08 pass is
a second independent confirmation.

Flipped-objective sweep (`sweep_candle_flip.py`, 300 TPE trials): best train
+82.96 → forward only +16.33 with **89% lottery share and 2/6 months** — a
kill under §1.4. TPE collapsed onto one concentrated basin while the boring
defaults carry the actual (modest, robust) edge. Optimization made the
candidate worse — the mirror image of volume_profile_rsi and hammer, where
the optimizer couldn't rescue defaults either.

Status: **promotable candidate, not promoted.** Next stage: formalize into a
trading layer as an entry-gate/conditioning feature (constraint 1.5 — likely
worth more as a short-blocking/momentum-confirm feature than a standalone
both-sides signal), then run the pool-conditioned separator test in the
parent engine before any deployment claim.

## Detector defects found & fixed

Sparsity suspicion triggered source audit + fire-rate forensics; two of five
sparse detectors were genuinely broken:

- **wedges.rs — non-causal, end-of-series artifact.** Pivots were selected
  from the WHOLE array (`p > len*0.3`, last-N-of-series), lines fitted once,
  breakout scanned to series end → signals can only exist near each symbol's
  final bars and depend on future data extent. BTC 1h: exactly 1 event, at
  position 1.00. Rewritten as a causal rolling-window scan (trailing
  `lookback`, last-N pivots inside the window, scale-free normalized slopes,
  single-bar cross with prev-bar-inside check). Post-fix BTC 1h: 308 events,
  median at series midpoint; sane scaling at 4h (82) and 1d (6). Universe
  re-screen: 8,093 signal bars, IC −0.0015 — first real test, clean kill.
- **cup_and_handle.rs — unsatisfiable retracement gate.**
  `(handle_high−handle_low)/(rim−handle_low)` starts its window one bar after
  the local top, so the ratio → 1.0 for any realistic pullback: a
  stage-by-stage trace showed ALL 586 depth-passing cups rejected across
  every parameterization and both timeframes. Replaced with classical handle
  depth `(rim − handle_low)/(rim − bottom)`. Post-fix fires track relaxation
  monotonically (BTC depth .08→2, .04→6, .02+dur60→26); strict defaults
  remain strict-for-BTC (13 months, 15% cup within ±10 bars of an order-5
  trough). Re-screen: 131 signal bars, IC −0.014 — now measurable, still
  nothing.
- island_reversal: implementation correct (fires on gapped synthetic data);
  true OHLC gaps essentially never exist in continuous perp data — pattern
  structurally absent from this universe. No fix possible or needed.
- diamond.rs / broadening.rs: working but threshold-starved (zero fires at
  defaults on BTC at any tf; appear when tolerance→0). Root design weakness:
  OLS "trendlines" fitted through order-1 pivots (every 3-bar wiggle) — unit
  tests hide this via smooth synthetic fixtures. Left unfixed; treat any
  future use as needing pivot-quality work first.

Changed files (uncommitted working tree at time of writing):
`crates/indicators/src/patterns/{wedges,cup_and_handle}.rs` (+ regression
tests incl. rejects-deep-handle and non-wedge-flat negatives),
`crates/strategies/src/types/configs.rs` (WedgeConfig.lookback),
`crates/strategies/src/patterns/wedge.rs`,
`bindings/{js,py}/src/indicators.rs` (lookback arg). Full workspace tests +
clippy green; wheel rebuilt and revalidated against real data.

Methodological notes for the record:

1. A sign bug and a winning contrarian flip are observationally identical at
   aggregate level — audit construction BEFORE crediting a flip (here the
   audit came first and was clean).
2. Synthetic-fixture testing validates geometry but hides pivot-noise
   sensitivity; fire-rate-on-real-data should be a standard smoke test for
   rare-event detectors (a handful of fires may be broken, not rare — see
   broken-strategies.md's corollary).
3. Flip arithmetic (`flip_net == −orig_net − 2·cost`) held exactly in every
   evaluation — cheap invariant, worth asserting in any flip-based test.
