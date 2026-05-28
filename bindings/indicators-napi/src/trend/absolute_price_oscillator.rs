use indicators_core::absolute_price_oscillator as apo_core;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Absolute Price Oscillator
#[napi]
pub fn absolute_price_oscillator(
	closes: Float64Array,
	fast_period: Option<u32>,
	slow_period: Option<u32>,
) -> Result<Vec<f64>> {
	apo_core(closes.as_ref(), fast_period, slow_period)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}
