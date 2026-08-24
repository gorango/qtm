"""Shared continuous-indicator features for the standalone screens.

Consolidated from the original ``gate_combos.py`` (``default_warmup``,
``adx_at``) and ``gate_separators.py`` (``INDICATOR_FEATURES``) so the
standalone ``gate_combos.py`` / ``lgbm_stack.py`` have one source of truth
and no dependency on the pool-conditioned modules.  Every feature is a
lambda ``(high, low, close, volume) -> np.ndarray`` computed per symbol.
"""

from __future__ import annotations

import numpy as np
import quantamental as q

ADX_PERIODS = (7, 14, 28)
BAR_SECS = {"15m": 900, "30m": 1800, "1h": 3600, "4h": 14400, "12h": 43200, "1d": 86400}
HOUR_MS = 3_600_000


def _band_pos(bands, close):
    """(close - lower)/(upper - lower) — position within a channel."""
    up = np.asarray(bands["upper"], dtype=float)
    lo = np.asarray(bands["lower"], dtype=float)
    rng = up - lo
    with np.errstate(divide="ignore", invalid="ignore"):
        pos = (np.asarray(close, dtype=float) - lo) / rng
    return np.where(np.isfinite(rng) & (rng > 0), pos, np.nan)


def _band_width(bands):
    """(upper - lower)/close — relative channel width."""
    up = np.asarray(bands["upper"], dtype=float)
    lo = np.asarray(bands["lower"], dtype=float)
    c = np.asarray(bands["middle"], dtype=float)
    with np.errstate(divide="ignore", invalid="ignore"):
        w = (up - lo) / c
    return np.where(np.isfinite(w), w, np.nan)


# Continuous indicator features computed per (symbol, bar) from OHLCV arrays.
INDICATOR_FEATURES = {
    "rsi": lambda h, low, c, v: q.rsi(c),
    "z_score": lambda h, low, c, v: q.z_score(c),
    "bbpos": lambda h, low, c, v: _band_pos(q.bollinger_bands(c), c),
    "bbwidth": lambda h, low, c, v: _band_width(q.bollinger_bands(c)),
    "atrp": lambda h, low, c, v: (
        np.asarray(q.atr(h, low, c)["atr_line"], dtype=float)
        / np.asarray(c, dtype=float)
    ),
    "mfi": lambda h, low, c, v: q.money_flow_index(h, low, c, v),
    "cci": lambda h, low, c, v: q.cci(h, low, c),
    "stochk": lambda h, low, c, v: q.stochastic_oscillator(h, low, c)["k"],
    "stochd": lambda h, low, c, v: q.stochastic_oscillator(h, low, c)["d"],
    "adx": lambda h, low, c, v: q.adx(h, low, c)["adx"],
    "dispread": lambda h, low, c, v: (
        np.asarray(q.adx(h, low, c)["plus_di"], dtype=float)
        - np.asarray(q.adx(h, low, c)["minus_di"], dtype=float)
    ),
    "macdh": lambda h, low, c, v: q.macd(c)["histogram"],
    "keltpos": lambda h, low, c, v: _band_pos(q.keltner_channel(h, low, c), c),
}

DEFAULT_INDICATORS = ",".join(sorted(INDICATOR_FEATURES))
# all features computed per symbol from OHLCV
FEATURES = list(INDICATOR_FEATURES) + [f"adx{p}" for p in ADX_PERIODS]


def default_warmup(tf: str, start: str) -> str:
    """Minimal indicator lookback before the analysis window: 2× the longest
    indicator period (ADX-28 = 29 bars), floored at 1 day.  ~14h at 15m,
    ~29h at 30m, ~2.4d at 1h — nothing needs a months-long warmup."""
    from datetime import datetime, timedelta, timezone

    max_bars = (max(ADX_PERIODS) + 1) * 2  # 58 bars
    secs = max(max_bars * BAR_SECS.get(tf, 3600), 24 * 3600)
    start_dt = datetime.strptime(start, "%Y-%m-%d").replace(tzinfo=timezone.utc)
    return (start_dt - timedelta(seconds=secs)).strftime("%Y-%m-%d")


def adx_at(h, low, c, v, period: int):
    return np.asarray(q.adx(h, low, c, {"period": period})["adx"], dtype=float)
