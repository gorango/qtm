use crate::internal::ema::ema_internal;
use crate::volatility::bollinger_bands::BBResult;
use crate::{IndicatorError, IndicatorResult};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
/// Bollinger Bandwidth result — normalized width and its EMA.
pub struct BBWResult {
	/// Normalized width `(upper-lower)/middle`.
	pub width: Vec<f64>,
	/// EMA of width over the given period (default 90).
	pub width_ema: Vec<f64>,
}

/// Bollinger Bandwidth — `bb` short alias for `bollinger_bands_width`.
/// `(upper - lower) / middle`. Dimensionless squeeze/expansion gauge.
pub fn bbw(bb: BBResult, period: Option<u32>) -> IndicatorResult<BBWResult> {
	let len = bb.upper.len();

	if len == 0 {
		return Err(IndicatorError::Custom(
			"Bollinger bands result cannot be empty".into(),
		));
	}

	if bb.upper.len() != bb.middle.len() || bb.upper.len() != bb.lower.len() {
		return Err(IndicatorError::Custom(
			"Bollinger bands arrays must have the same length".into(),
		));
	}

	let period = period.unwrap_or(90) as usize;
	crate::utils::validation::validate_period(period)?;

	let mut width = vec![f64::NAN; len];

	for (i, width_val) in width.iter_mut().enumerate().take(len) {
		let upper = bb.upper[i];
		let lower = bb.lower[i];
		let middle = bb.middle[i];

		if !upper.is_nan() && !lower.is_nan() && !middle.is_nan() {
			*width_val = if middle != 0.0 {
				(upper - lower) / middle
			} else {
				0.0
			};
		}
	}

	let width_ema = ema_internal(&width, period);

	Ok(BBWResult { width, width_ema })
}

/// Bollinger Bandwidth — `(BB.upper - BB.lower) / BB.middle`.
///
/// Normalized band width. Low = squeeze, high = expansion. Dimensionless.
/// `NaN` where BB middle is 0 or `NaN`.
pub fn bollinger_bands_width(bb: BBResult, period: Option<u32>) -> IndicatorResult<BBWResult> {
	bbw(bb, period)
}
