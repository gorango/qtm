use crate::internal::true_range::tr_internal;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct TrueRangeResult {
	pub tr_line: Vec<f64>,
}

pub fn tr(highs: &[f64], lows: &[f64], closings: &[f64]) -> Result<TrueRangeResult, String> {
	crate::utils::validation::validate_multiple_arrays(&[highs, lows, closings])?;

	Ok(TrueRangeResult {
		tr_line: tr_internal(highs, lows, closings),
	})
}

pub fn true_range(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
) -> Result<TrueRangeResult, String> {
	tr(highs, lows, closings)
}
