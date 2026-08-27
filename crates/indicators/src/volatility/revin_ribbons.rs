use crate::internal::ema::ema_internal;
use crate::internal::true_range::tr_internal;
use crate::trend::rma::rma_internal;
use crate::IndicatorResult;
use serde::{Deserialize, Serialize};

/// Revin Ribbons — open approximation of the proprietary Revin Ribbons volatility-trend-momentum channel.
///
/// Proprietary formula is closed-source. This is a documented open approximation that
/// reproduces the described structure: dynamic midline + tiered ATR-based ribbons on price.
///
/// Structure:
/// - `midline` — adaptive equilibrium (EMA of closes)
/// - `S1/R1` — Master Bands (primary support/resistance, `s1_mult * ATR`)
/// - `S2/R2` — Extended Confluence (secondary, `s2_mult * ATR`)
/// - `S3/R3` — Extended Confluence (tertiary, `s3_mult * ATR`)
/// - `mid_upper/mid_lower` — dotted midlines (midpoints between midline and R1/S1)
#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct RevinRibbonsResult {
	pub midline: Vec<f64>,
	pub s1: Vec<f64>,
	pub r1: Vec<f64>,
	pub s2: Vec<f64>,
	pub r2: Vec<f64>,
	pub s3: Vec<f64>,
	pub r3: Vec<f64>,
	/// Dotted midline between midline and R1.
	pub mid_upper: Vec<f64>,
	/// Dotted midline between midline and S1.
	pub mid_lower: Vec<f64>,
}

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RevinRibbonsConfig {
	/// Midline EMA period (default 20).
	pub period: Option<u32>,
	/// ATR (RMA of true range) period (default 14).
	pub atr_period: Option<u32>,
	/// ATR multiplier for S1/R1 Master Bands (default 1.5).
	pub s1_mult: Option<f64>,
	/// ATR multiplier for S2/R2 Confluence Bands (default 2.5).
	pub s2_mult: Option<f64>,
	/// ATR multiplier for S3/R3 Confluence Bands (default 3.5).
	pub s3_mult: Option<f64>,
}

impl Default for RevinRibbonsConfig {
	fn default() -> Self {
		Self {
			period: Some(20),
			atr_period: Some(14),
			s1_mult: Some(1.5),
			s2_mult: Some(2.5),
			s3_mult: Some(3.5),
		}
	}
}

/// Open approximation of Revin Ribbons.
/// `s1_mult < s2_mult < s3_mult` is not enforced but expected.
pub fn revin_ribbons(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<RevinRibbonsConfig>,
) -> IndicatorResult<RevinRibbonsResult> {
	crate::utils::validation::validate_multiple_arrays(&[highs, lows, closes])?;
	crate::utils::validation::validate_finite(&[highs, lows, closes])?;

	let len = closes.len();
	let cfg = config.unwrap_or_default();
	let period = cfg.period.unwrap_or(20) as usize;
	let atr_period = cfg.atr_period.unwrap_or(14) as usize;
	let s1 = cfg.s1_mult.unwrap_or(1.5);
	let s2 = cfg.s2_mult.unwrap_or(2.5);
	let s3 = cfg.s3_mult.unwrap_or(3.5);

	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_period(atr_period)?;
	if !s1.is_finite() || !s2.is_finite() || !s3.is_finite() {
		return Err(crate::IndicatorError::Custom(
			"RevinRibbons multipliers must be finite".into(),
		));
	}
	if s1 <= 0.0 || s2 <= 0.0 || s3 <= 0.0 {
		return Err(crate::IndicatorError::Custom(
			"RevinRibbons multipliers must be > 0".into(),
		));
	}

	let tr_line = tr_internal(highs, lows, closes);
	let atr_line = rma_internal(&tr_line, atr_period);
	let midline = ema_internal(closes, period);

	let mut s1_arr = vec![f64::NAN; len];
	let mut r1_arr = vec![f64::NAN; len];
	let mut s2_arr = vec![f64::NAN; len];
	let mut r2_arr = vec![f64::NAN; len];
	let mut s3_arr = vec![f64::NAN; len];
	let mut r3_arr = vec![f64::NAN; len];
	let mut mid_upper = vec![f64::NAN; len];
	let mut mid_lower = vec![f64::NAN; len];

	for i in 0..len {
		let mid = midline[i];
		let atr = atr_line[i];
		if mid.is_nan() || atr.is_nan() {
			continue;
		}
		r1_arr[i] = mid + atr * s1;
		s1_arr[i] = mid - atr * s1;
		r2_arr[i] = mid + atr * s2;
		s2_arr[i] = mid - atr * s2;
		r3_arr[i] = mid + atr * s3;
		s3_arr[i] = mid - atr * s3;
		// dotted midlines — secondary internal midpoints
		mid_upper[i] = (mid + r1_arr[i]) / 2.0;
		mid_lower[i] = (mid + s1_arr[i]) / 2.0;
	}

	Ok(RevinRibbonsResult {
		midline,
		s1: s1_arr,
		r1: r1_arr,
		s2: s2_arr,
		r2: r2_arr,
		s3: s3_arr,
		r3: r3_arr,
		mid_upper,
		mid_lower,
	})
}

/// Alias matching the TradingView suite naming.
pub fn revin_ribbons_alias(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<RevinRibbonsConfig>,
) -> IndicatorResult<RevinRibbonsResult> {
	revin_ribbons(highs, lows, closes, config)
}
