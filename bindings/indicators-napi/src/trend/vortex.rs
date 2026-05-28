use indicators_core::{vortex as vortex_core, VortexResult};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Vortex
#[napi]
pub fn vortex(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	period: Option<u32>,
) -> Result<VortexResult> {
	vortex_core(highs.as_ref(), lows.as_ref(), closings.as_ref(), period)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}
