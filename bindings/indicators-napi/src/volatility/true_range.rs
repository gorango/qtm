use indicators_core::volatility::true_range::{
	tr as tr_core, true_range as tr_alias, TrueRangeResult,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Tr
#[napi]
pub fn tr(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
) -> Result<TrueRangeResult> {
	tr_core(highs.as_ref(), lows.as_ref(), closings.as_ref())
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// True Range (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn trueRange(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
) -> Result<TrueRangeResult> {
	tr_alias(highs.as_ref(), lows.as_ref(), closings.as_ref())
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}
