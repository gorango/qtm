use crate::internal::ema::ema_internal;
use crate::internal::true_range::tr_internal;
use crate::momentum::rsi::{rsi, RSIConfig};
use crate::trend::rma::rma_internal;
use crate::IndicatorResult;
use serde::{Deserialize, Serialize};

/// Revin Momentum Oscillator (RMO) — open approximation.
///
/// Proprietary RMO measures 5 dimensions: Duration, Price Move, Separation,
/// Oscillator Level, Combined. This approximation reproduces the intent:
/// - **duration** — normalized bars since last midline cross (-100..100)
/// - **price_move** — (close - midline)/ATR scaled (-100..100)
/// - **separation** — (EMA_fast - EMA_slow)/ATR scaled (-100..100)
/// - **level** — RSI-derived oscillator level (-100..100)
/// - **combined** — mean of the four, the traded oscillator
#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct RMOResult {
	pub duration: Vec<f64>,
	pub price_move: Vec<f64>,
	pub separation: Vec<f64>,
	pub level: Vec<f64>,
	pub combined: Vec<f64>,
}

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RevinMomentumOscillatorConfig {
	/// Midline EMA period (default 20).
	pub period: Option<u32>,
	/// Fast EMA period for separation (default 10).
	pub fast_period: Option<u32>,
	/// Slow EMA period for separation (default 30).
	pub slow_period: Option<u32>,
	/// RSI period for oscillator level (default 14).
	pub rsi_period: Option<u32>,
}

impl Default for RevinMomentumOscillatorConfig {
	fn default() -> Self {
		Self {
			period: Some(20),
			fast_period: Some(10),
			slow_period: Some(30),
			rsi_period: Some(14),
		}
	}
}

fn clamp(v: f64, lo: f64, hi: f64) -> f64 {
	if v < lo {
		lo
	} else if v > hi {
		hi
	} else {
		v
	}
}

/// Open approximation. Returns 5 synchronized Vec<f64> in -100..100 (NaN until warmup).
pub fn revin_momentum_oscillator(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<RevinMomentumOscillatorConfig>,
) -> IndicatorResult<RMOResult> {
	crate::utils::validation::validate_multiple_arrays(&[highs, lows, closes])?;
	crate::utils::validation::validate_finite(&[highs, lows, closes])?;

	let len = closes.len();
	let cfg = config.unwrap_or_default();
	let period = cfg.period.unwrap_or(20) as usize;
	let fast = cfg.fast_period.unwrap_or(10) as usize;
	let slow = cfg.slow_period.unwrap_or(30) as usize;
	let rsi_period = cfg.rsi_period.unwrap_or(14);

	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_period(fast)?;
	crate::utils::validation::validate_period(slow)?;
	crate::utils::validation::validate_period(rsi_period as usize)?;

	let midline = ema_internal(closes, period);
	let ema_fast = ema_internal(closes, fast);
	let ema_slow = ema_internal(closes, slow);
	let tr_line = tr_internal(highs, lows, closes);
	let atr = rma_internal(&tr_line, period);
	let rsi_vals = rsi(
		closes,
		Some(RSIConfig {
			period: Some(rsi_period),
		}),
	);

	let mut duration = vec![f64::NAN; len];
	let mut price_move = vec![f64::NAN; len];
	let mut separation = vec![f64::NAN; len];
	let mut level = vec![f64::NAN; len];
	let mut combined = vec![f64::NAN; len];

	// Track last midline cross for duration
	let mut last_cross_idx: Option<usize> = None;
	let mut last_cross_sign: i8 = 0;

	for i in 0..len {
		let mid = midline[i];
		let c = closes[i];
		let a = atr[i];
		let ef = ema_fast[i];
		let es = ema_slow[i];
		let r = rsi_vals[i];

		if mid.is_nan() || c.is_nan() {
			continue;
		}

		// Detect midline cross to reset duration anchor
		if i > 0 {
			let prev_c = closes[i - 1];
			let prev_mid = midline[i - 1];
			if !prev_c.is_nan() && !prev_mid.is_nan() {
				let crossed_up = prev_c <= prev_mid && c > mid;
				let crossed_down = prev_c >= prev_mid && c < mid;
				if crossed_up {
					last_cross_idx = Some(i);
					last_cross_sign = 1;
				} else if crossed_down {
					last_cross_idx = Some(i);
					last_cross_sign = -1;
				}
			}
		}

		// Duration: normalized bars since last cross, signed by side of midline
		if let Some(anchor) = last_cross_idx {
			let bars_since = (i - anchor) as f64;
			// normalize to period window, cap at 2*period -> 100
			let norm = (bars_since / (period as f64 * 2.0) * 100.0).min(100.0);
			let sign = if c > mid {
				1.0
			} else if c < mid {
				-1.0
			} else {
				last_cross_sign as f64
			};
			duration[i] = sign * norm;
		} else {
			// no cross yet -> 0 until anchored
			duration[i] = 0.0;
		}

		// Price move: (close - midline)/ATR * 10, clamped -100..100
		if !a.is_nan() && a != 0.0 {
			price_move[i] = clamp((c - mid) / a * 10.0, -100.0, 100.0);
		} else {
			price_move[i] = 0.0;
		}

		// Separation: (EMA_fast - EMA_slow)/ATR * 10
		if !ef.is_nan() && !es.is_nan() && !a.is_nan() && a != 0.0 {
			separation[i] = clamp((ef - es) / a * 10.0, -100.0, 100.0);
		} else if !ef.is_nan() && !es.is_nan() {
			separation[i] = clamp(ef - es, -100.0, 100.0);
		} else {
			separation[i] = f64::NAN;
		}

		// Level: RSI centered at 50 -> -100..100
		if !r.is_nan() {
			level[i] = clamp((r - 50.0) * 2.0, -100.0, 100.0);
		} else {
			level[i] = f64::NAN;
		}

		// Combined: mean of available components
		let mut sum = 0.0;
		let mut cnt = 0usize;
		for v in [duration[i], price_move[i], separation[i], level[i]] {
			if !v.is_nan() {
				sum += v;
				cnt += 1;
			}
		}
		if cnt > 0 {
			combined[i] = sum / cnt as f64;
		}
	}

	Ok(RMOResult {
		duration,
		price_move,
		separation,
		level,
		combined,
	})
}

/// Alias `rmo`.
pub fn rmo(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<RevinMomentumOscillatorConfig>,
) -> IndicatorResult<RMOResult> {
	revin_momentum_oscillator(highs, lows, closes, config)
}
