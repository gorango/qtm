use indicators_core::{super_trend as st_core, SuperTrendResult};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Super Trend
#[napi]
pub fn super_trend(
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
	period: Option<u32>,
	multiplier: Option<f64>,
) -> Result<SuperTrendResult> {
	st_core(
		highs.as_ref(),
		lows.as_ref(),
		closes.as_ref(),
		period,
		multiplier,
	)
	.map_err(napi::Error::from_reason)
}
