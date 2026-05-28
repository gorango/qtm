use indicators_core::volatility::projection_oscillator::{
	po as po_core, projection_oscillator as po_alias, POResult,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Po
#[napi]
pub fn po(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	period: Option<u32>,
	smooth: Option<u32>,
) -> Result<POResult> {
	po_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		period,
		smooth,
	)
	.map_err(napi::Error::from_reason)
}

/// Projection Oscillator (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn projectionOscillator(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	period: Option<u32>,
	smooth: Option<u32>,
) -> Result<POResult> {
	po_alias(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		period,
		smooth,
	)
	.map_err(napi::Error::from_reason)
}
