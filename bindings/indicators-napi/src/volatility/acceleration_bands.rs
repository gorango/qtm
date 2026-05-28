use indicators_core::volatility::acceleration_bands::{
	ab as ab_core, acceleration_bands as ab_alias, ABResult,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Ab
#[napi]
pub fn ab(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	period: Option<u32>,
	multiplier: Option<f64>,
) -> Result<ABResult> {
	ab_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		period,
		multiplier,
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Acceleration Bands (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn accelerationBands(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	period: Option<u32>,
	multiplier: Option<f64>,
) -> Result<ABResult> {
	ab_alias(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		period,
		multiplier,
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}
