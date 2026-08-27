use crate::volatility::revin_ribbons::{revin_ribbons, RevinRibbonsConfig};
use crate::IndicatorResult;
use serde::{Deserialize, Serialize};

/// Revin Width Percentile (RWP) — open approximation.
///
/// Volatility regime: where current ribbon width sits in its historical range.
/// Low percentile = compression (expansion imminent), high = expansion/exhaustion.
/// Width = (R1 - S1) / midline * 100 (normalized band width). Percentile is
/// rank of current width vs `lookback` prior widths, scaled to 0..100.
#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RevinWidthPercentileConfig {
	/// Midline EMA period (default 20).
	pub period: Option<u32>,
	/// ATR period (default 14).
	pub atr_period: Option<u32>,
	/// S1/R1 multiplier used to compute width (default 1.5).
	pub s1_mult: Option<f64>,
	/// Lookback window for percentile rank (default 100).
	pub lookback: Option<u32>,
}

impl Default for RevinWidthPercentileConfig {
	fn default() -> Self {
		Self {
			period: Some(20),
			atr_period: Some(14),
			s1_mult: Some(1.5),
			lookback: Some(100),
		}
	}
}

/// Returns Vec<f64> in [0, 100] (NaN where insufficient history).
pub fn revin_width_percentile(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<RevinWidthPercentileConfig>,
) -> IndicatorResult<Vec<f64>> {
	crate::utils::validation::validate_multiple_arrays(&[highs, lows, closes])?;
	crate::utils::validation::validate_finite(&[highs, lows, closes])?;

	let cfg = config.unwrap_or_default();
	let period = cfg.period.unwrap_or(20);
	let atr_period = cfg.atr_period.unwrap_or(14);
	let s1_mult = cfg.s1_mult.unwrap_or(1.5);
	let lookback = cfg.lookback.unwrap_or(100) as usize;

	crate::utils::validation::validate_period(period as usize)?;
	crate::utils::validation::validate_period(atr_period as usize)?;
	crate::utils::validation::validate_period(lookback)?;
	if !s1_mult.is_finite() || s1_mult <= 0.0 {
		return Err(crate::IndicatorError::Custom(
			"RevinWidthPercentile s1_mult must be finite and > 0".into(),
		));
	}

	let ribbons = revin_ribbons(
		highs,
		lows,
		closes,
		Some(RevinRibbonsConfig {
			period: Some(period),
			atr_period: Some(atr_period),
			s1_mult: Some(s1_mult),
			s2_mult: Some(2.5),
			s3_mult: Some(3.5),
		}),
	)?;

	let len = closes.len();
	let mut width = vec![f64::NAN; len];
	for (i, w) in width.iter_mut().enumerate() {
		let mid = ribbons.midline[i];
		let r1 = ribbons.r1[i];
		let s1 = ribbons.s1[i];
		if mid.is_nan() || r1.is_nan() || s1.is_nan() || mid == 0.0 {
			continue;
		}
		*w = (r1 - s1) / mid.abs() * 100.0;
	}

	let mut result = vec![f64::NAN; len];
	// percentile rank vs trailing lookback window
	for (i, cur) in width.iter().enumerate() {
		if cur.is_nan() {
			continue;
		}
		if i < lookback {
			continue;
		}
		let cur = *cur;
		let mut count = 0usize;
		let mut valid = 0usize;
		for &v in &width[(i - lookback)..i] {
			if v.is_nan() {
				continue;
			}
			valid += 1;
			if v <= cur {
				count += 1;
			}
		}
		if valid == 0 {
			continue;
		}
		result[i] = (count as f64 / valid as f64) * 100.0;
	}

	Ok(result)
}

/// Alias `rwp`.
pub fn rwp(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<RevinWidthPercentileConfig>,
) -> IndicatorResult<Vec<f64>> {
	revin_width_percentile(highs, lows, closes, config)
}
