use indicators_core::volume::chaikin_money_flow::{
	chaikin_money_flow as cmf_core, cmf as cmf_alias,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Chaikin Money Flow
#[napi]
pub fn chaikin_money_flow(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	volumes: Float64Array,
	period: Option<u32>,
) -> Vec<f64> {
	let period = period.unwrap_or(20);
	cmf_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		volumes.as_ref(),
		period,
	)
}

/// Cmf
#[napi]
pub fn cmf(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	volumes: Float64Array,
	period: Option<u32>,
) -> Vec<f64> {
	let period = period.unwrap_or(20);
	cmf_alias(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		volumes.as_ref(),
		period,
	)
}
