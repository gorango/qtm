use indicators_core::{adx as adx_core, ADXConfig, ADXResult};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Adx
#[napi]
pub fn adx(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	config: Option<ADXConfig>,
) -> Result<ADXResult> {
	adx_core(highs.as_ref(), lows.as_ref(), closings.as_ref(), config)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}
