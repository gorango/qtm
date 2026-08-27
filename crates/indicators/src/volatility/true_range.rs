use crate::internal::true_range::tr_internal;
use crate::IndicatorResult;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct TrueRangeResult {
	pub tr_line: Vec<f64>,
}

/// True Range — `tr` short alias. `max(high-low, |high-prev_close|, |low-prev_close|)` per bar.
pub fn tr(highs: &[f64], lows: &[f64], closings: &[f64]) -> IndicatorResult<TrueRangeResult> {
	crate::utils::validation::validate_multiple_arrays(&[highs, lows, closings])?;
	crate::utils::validation::validate_finite(&[highs, lows, closings])?;

	Ok(TrueRangeResult {
		tr_line: tr_internal(highs, lows, closings),
	})
}

/// True Range — Wilder's range accounting for gaps. Full-name alias.
/// `TR = max(high-low, |high - prev_close|, |low - prev_close|)`. No warmup beyond first bar.
pub fn true_range(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
) -> IndicatorResult<TrueRangeResult> {
	tr(highs, lows, closings)
}
