use indicators_core::volatility::donchian_channel::{
	dc as dc_core, donchian_channel as dc_alias, DCResult,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Dc
#[napi]
pub fn dc(closings: Float64Array, period: Option<u32>) -> Result<DCResult> {
	dc_core(closings.as_ref(), period).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Donchian Channel (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn donchianChannel(closings: Float64Array, period: Option<u32>) -> Result<DCResult> {
	dc_alias(closings.as_ref(), period).map_err(|e| napi::Error::from_reason(e.to_string()))
}
