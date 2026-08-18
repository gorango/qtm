# Traditional Indicators & Strategies — standalone (quantamental)

Screen the catalog from the native Rust `quantamental` PyPI package (pyo3
bindings) against the shared validation protocol, **with zero external
infra**: OHLCV comes from Binance (monthly `data.binance.vision` backfill +
REST tail) instead of a postgres `price_bars` table.  This is the same
"borrowed ranker" pattern as the parent validation work but over the whole
catalog of classic TA strategies, now engine-free and runable anywhere the
`quantamental` wheel builds.

Layout:

    binance_loader.py     OHLCV pipeline — universe, hydrate, resample, load_price_bars()
    config/symbols-sync.py  (re)generate config/symbols.yaml (CoinGecko × Binance Futures)
    config/symbols.yaml   the hydration universe (canonical {BASE}/USDT:PERP)
    screen.py             per-strategy rank-IC screen + stability gate + optuna sweep
    gate_combos.py        bar-level discovery screen (constraint 1.5, engine-free)
    lgbm_stack.py         2x2 LightGBM stack over the catalog
    indicators.py         shared continuous-indicator features (consolidated from the screen modules)
    BROKEN-STRATEGIES.md  upstream catalog bugs + fixes (historical)

## Setup

    just -f research/justfile setup   # venv + pip installs + the quantamental wheel (maturin build)

Verify the wheel: `./.venv/bin/python -c "import quantamental as q; print(len(q.get_strategy_registry()['strategies']), 'strategies')"`.

Data is hydrated independently of this (see below) and cached under
`data/` (gitignored).

## Data pipeline

The universe lives in `config/symbols.yaml` — top Binance-Futures USDT
perpetuals by CoinGecko market cap, canonical `{BASE}/USDT:PERP` spelling.
Regenerate it when the top-200 shifts:

    ./.venv/bin/python config/symbols-sync.py

Hydrate OHLCV (1m resolution, one parquet per symbol under `data/`):

    ./.venv/bin/python binance_loader.py --sync              # vision monthly backfill + REST tail
    ./.venv/bin/python binance_loader.py --sync --start 2023-01-01
    ./.venv/bin/python binance_loader.py --update            # REST-only daily tail refresh

Then load any resampled window with the pg_loader-compatible contract:

    from binance_loader import load_price_bars, coverage_report
    data = load_price_bars("2025-07-01", "2026-08-01", tf="1h")   # {canonical: pl.DataFrame}
    coverage_report(data)                                          # per-month symbol counts

`load_price_bars` returns per-symbol DataFrames with `timestamp` (ms
epoch Int64) / open / high / low / close / volume (Float32), resampled
epoch-aligned (open=first, high=max, low=min, close=last, volume=sum;
partial boundary buckets and symbols with <50 bars dropped).  Any symbol
can be overridden/limited via `syms` (a file path or explicit universe).

**Coverage note:** rank IC needs ≥20 symbols per bar, so periods before
most of the current universe listed drop out automatically; check the
per-month coverage report and pick windows whose months are complete.

## Screen the catalog

    ./.venv/bin/python screen.py                                # curated default set
    ./.venv/bin/python screen.py --all                          # all 97 strategies
    ./.venv/bin/python screen.py --category trend,volume
    ./.venv/bin/python screen.py --strategies ma-crossover,rsi
    ./.venv/bin/python screen.py --tf 4h --start 2023-01-01     # other timeframe / deeper window

Per-bar cross-sectional Spearman rank IC of each strategy's discrete
signal vs timestamp-based forward returns at 6/12/24h (the protocol's
first kill test — constraint 1: raw 1h rank-IC ceiling ~0.03-0.04;
anything at/below the 10 bps cost floor is dead on arrival).

### Stability gate (before optuna)

Per-month net bps + trade counts for one strategy, plus the lottery-alpha
share (constraint 1.4), over the window:

    ./.venv/bin/python screen.py --stability z_score_reversion --start 2025-07-01 --end 2026-08-01
    ./.venv/bin/python screen.py --stability rsi --config '{"period": 3}'

Month-to-month spread = stability; ≥6/9 positive months = the walk-forward
gate; a top-3-trade PnL share near/exceeding 100% = lottery-alpha, not edge.

### Correlation (independent-bet check)

    ./.venv/bin/python screen.py --correlate --strategies z_score_reversion,bollinger_bands_mean_reversion,donchian_reversion

High corr/agreement = the same bet twice.  Low-correlation survivors are
the candidates for the separator/gate test (constraint 1.5), not
screen-level portfolio stacking.

### Optuna sweep

Optimize one strategy's params against the protocol objective — search
space = the strategy's own `optimization_bounds`; objective = overlap-free
net bps after `--cost` (default 10 bps round-trip), **train months only**;
the last `--forward-months` (default 6) are held out entirely and are the
arbiter (a strategy is not promotable if best forward net bps ≤ 0):

    ./.venv/bin/python screen.py --sweep z_score_reversion --trials 300 --start 2025-07-01 --end 2026-08-01
    ./.venv/bin/python screen.py --sweep rsi --tf 4h --trials 300
    ./.venv/bin/python screen.py --sweep volume_profile_rsi --min-trades 100 --storage sqlite:///sweep.db --cap-tiers caps.csv

`--min-trades` floors sparse "lucky" configs; `--storage` persists/resumes
the study; `--cap-tiers symbol,tier.csv` breaks the best trial out by
market-cap tier (train + forward) — read the split as **the high-cap
bucket is the deployable number**; small-cap edge is the least trustworthy
for sizing.

## Bar-level discovery screen (constraint 1.5) — gate_combos.py

Discovery-stage separator test — pure OHLCV, no engine, no pool replay.
The sample is every (symbol, bar) with a finite forward return:

    ./.venv/bin/python gate_combos.py --tf 1h,4h,12h --horizon 12    # default
    ./.venv/bin/python gate_combos.py --tf 15m,30m,1d --cap-volume

16 continuous indicators (incl. ADX at periods 7/14/28) scored two ways:
**directional** (cross-sectional rank IC + train-selected tail gate on
vol-normalized returns) and **trend-continuation** (gate on
`sign(prior-move) × forward return`).  Plus pairwise AND-gates and a ridge
linear gate.  A survivor must clear: train-selected gate with POSITIVE
test uplift AND sign-stable corr.  `--cap-tiers caps.csv` or `--cap-volume`
split the reporting by size tier.  `--warmup` defaults to a derived
indicator lookback (2× the longest period, floored at 1 day).

## LGBM stack over the whole catalog — lgbm_stack.py

Does a non-linear combination of the catalog extract what individual
screens could not?  2x2 grid {discrete signals, continuous indicators} ×
{lambdarank, regression}, 2025-01..2026-08 1h, last 6 months held out:

    ./.venv/bin/python lgbm_stack.py                                     # full grid
    ./.venv/bin/python lgbm_stack.py --features signals --objectives lambdarank
    ./.venv/bin/python lgbm_stack.py --strategies rsi,ma_crossover \
        --features indicators --objectives regression --max-rounds 5  # smoke test

Judged the protocol's way: per-bar rank IC (train|val|forward), net bps
after `--cost` (top/bottom `--k`), lottery share, per-month forward net
bps, feature gains, prediction correlation across runs.

## Findings (2026-08)

Catalog verdict after the baseline (97-strategy screen on 2024-01..2025-08
and 2025-07..2026-08), the stability gate, the sweep, the separator test,
and the bar-level discovery screen:

- **Bar-level discovery screen (engine-free): no survivor.**  Over 1h/4h/12h
  with a 12h forward horizon, all 16 classic indicators (incl. ADX 7/14/28)
  have sign-stable **negative** directional rank IC (rsi −0.032/−0.026,
  bbwidth −0.039/−0.034, atrp −0.059/−0.036, …) — they *anti*-predict
  direction in this universe, consistent with the standing sub-cost finding.
  ADX is direction-agnostic by construction (IC ~0, sign-flips train/test) and
  shows **no trend-continuation edge** at bar level: every
  `sign(prior-move)×fwd` gate is ~0.000pp across features and TFs.  Pairwise
  AND and ridge-linear combos top out at +0.016pp with train/test disagreeing —
  noise.  **No cap tier** (high/mid/low) carries an edge; the high-cap bucket
  (the deployable number) is negative or ~0 everywhere.  Nothing survives to
  formalize into the FSM.
- **No standalone survivor.** Best ICs ~0.016-0.020 (z_score_reversion,
  keltner_volatility_breakout, mad_reversion) are sub-cost: converting to net bps
  after the 10 bps round-trip floor gives **negative** per-trade net
  (z_score_reversion +0.016 IC → −17.6 bps net).  The last hope —
  `volume_profile_rsi` (default params net +8.1/+25.4 bps, low lottery, both
  windows) — **died in its optuna sweep**: best train +60.40 bps →
  held-out forward **−30.04 bps**, edge concentrated in two train months
  (2025-10, 2026-02), forward months +63/0/−25/+36/−220/0, 6/13 positive.
  TPE collapsed onto that one overfit basin (top-5 all identical).
  Protocol-7 in action: the train/forward split caught the overfit.
- **The reversion "family" is ~2-3 independent bets**, not 8: `mad_reversion`
  = `z_score_reversion` (corr 1.00, byte-identical at 30m too); bb/donchian/
  keltner-reversion cluster 0.31-0.54; `z_score_reversion` is orthogonal to
  the band family.
- **Multi-TF (4h) screen: kill.**  The 1h reversion edge decays with horizon
  as the validation README's decay table predicts (z_score_reversion
  +0.0173→+0.0074, bollinger_bands_mean_reversion +0.0095→+0.0020,
  donchian_reversion +0.0060→−0.0024).  The only dense positive at 4h is
  `fibonacci_retracement` (+0.0163) — a rejected family, and unstable across
  TFs (−0.0009 at 1h).  No TF resurrects the catalog.
- **30m screen: no TF resurrects the catalog.**  Full 97-strategy screen on
  30m (2025-07..2026-08): the best 12h ICs are the same sub-cost band as 1h
  (keltner_volatility_breakout +0.0189, z_score_reversion +0.0187,
  mad_reversion +0.0187, projection_oscillator +0.0181), and the stability
  gate kills all four — whole-window net bps after cost −2.6/−2.5/−2.5/−9.6,
  positive months 5/14, 3/14, 3/14, 1/14, low lottery (8–38%) = uniform
  bleed, not concentration.  `projection_oscillator` (horizon-stable) fires
  ~115k trades at −9.6 bps — dead.  Strongest anti-predictors:
  opening_range_breakout −0.028, donchian_breakout −0.020,
  atr_volatility_threshold −0.019.  (The local source build fires all 97
  strategies, incl. the logic-bug + pair fixes; every newly-firing one is
  sub-cost or negative.)
- **LGBM stack over the whole catalog: kill.**  Every cell's forward rank IC
  ≤ +0.008 and forward net bps after cost ≤ 0 (signals×lambda +0.0288 train →
  +0.0083 forward, −1.91 bps, 1/6 months; signals×reg −19.9 bps, 0/6;
  indicators×reg +0.0332 train → −0.0016 forward, −12.1 bps, 1/6;
  indicators×lambda forward IC −0.0015 with +7.61 bps net = a beta tilt, not
  rank skill).  The stack shows constraint-1 is NOT a linear-model artifact:
  train IC reaches the 0.03–0.04 ceiling (+0.033) and transfers nothing.
  Highest-gain features are all anti-predictors (atrp −0.061, dispread
  −0.040, bbwidth −0.038).  Indicator objectives' prediction corr is 0.29 on
  forward rows — no convergent bet.
- Consistent with the standing parent-pool finding: raw cross-sectional
  trend/momentum/reversion in this universe is sub-cost.  The catalog's
  realistic role is gate/size *features* via the constraint-1.5 separator
  mechanism — and even that failed month-stability at bar level here.
  (Pool-conditioned follow-ups — `gate_separators.py`, `adx_*` — live in
  the parent repo's validation tree, not here.)

## API notes (learned while wiring this up)

- `run_strategy(id, input, config=None)` — `input` is a dict with `closes`
  (required) plus optional `opens/highs/lows/volumes/timestamps`; numpy arrays
  or lists both accepted.  Returns a numpy array of discrete signals:
  `+1` (buy) / `-1` (sell) / `0` (flat).  Warmup bars emit `0`.
  Config keys are snake_case and normalized to the serde camelCase fields.
- Simple indicators return a single `np.ndarray` (NaN warmup); composite
  indicators return a `dict` of named series (`macd` → `histogram/macd/signal`,
  `adx` → `adx/+di/-di`, `bollinger_bands` → `upper/middle/lower`, …).
  Coerce with `np.asarray(v)` / handle the dict.
- **Defaults**: `get_strategy_defaults()` returns every strategy's `params` +
  `optimization_bounds` in one call (97 entries, keyed by id) — pass `params`
  as the `config` dict to `run_strategy`.  (`strategy_defaults(name)` /
  `strategy_metadata(name)` are a *separate* API for the hand-written
  fundamental strategies like `value_strategy`, not the TA catalog.)
- Quirk (PyPI 0.1.0 wheel — now stale): 11 strategies never fired — 6 hard
  logic bugs (`super_trend` band ratchet, `donchian_breakout` current-bar
  channel, `kst` NaN-poisoned SMA, the `vwap_ema_rsi_trend` self-
  contradiction, the `double_top_stochastic_reversal` sign mismatch,
  `elliott_wave_pattern`'s mis-modeled impulse), 2 end-anchored pattern
  detectors (`triangles`, `cup_and_handle`), and the 3 statistics pair
  strategies (the wheel's bindings had no `secondCloses`).  Root causes +
  upstream fixes: `BROKEN-STRATEGIES.md`.  **The local source build fixes
  these — all 97 strategies fire**, so an all-zero signal column now
  genuinely means "tested and weak", not "broken".

## Screening against the validation protocol

The standing constraints this catalog runs into (from the parent validation
protocol this standalone inherits):

1. **Rank-IC ceiling ~0.03–0.04** on raw 1h cross-sectional features — a
   discrete -1/0/+1 signal's per-bar rank IC will be far below that, and it
   has to clear **10 bps** round-trip (5 bps one-way).  Trend/momentum
   follow signals here have already lost.
2. **Short-horizon churn kills fades** — hold-to-horizon is the validated
   exit; per-trade short exits on fade strategies destroyed the edge.
3. **Lottery-alpha is the norm** — check PnL concentration before believing
   any winner.
4. **The one proven extraction mechanism** is sub-cost features inside a
   multivariate conditional gate (constraint 1.5), not standalone alpha.

So: every screen here is a *killer* (rules strategies out), not a promoter.
A positive screen number is a necessary, not sufficient, condition.