use indicators_core::volatility::chandelier_exit::{
	ce as ce_core, chandelier_exit as ce_alias, CEResult,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Ce
#[napi]
pub fn ce(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	period: Option<u32>,
) -> Result<CEResult> {
	ce_core(highs.as_ref(), lows.as_ref(), closings.as_ref(), period)
		.map_err(napi::Error::from_reason)
}

/// Chandelier Exit (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn chandelierExit(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	period: Option<u32>,
) -> Result<CEResult> {
	ce_alias(highs.as_ref(), lows.as_ref(), closings.as_ref(), period)
		.map_err(napi::Error::from_reason)
}
