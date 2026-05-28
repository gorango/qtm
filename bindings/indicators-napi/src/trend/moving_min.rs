use indicators_core::trend::moving_min::moving_min_internal;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Moving Min
#[napi]
pub fn moving_min(values: Float64Array, period: Option<u32>) -> Vec<f64> {
	let period = period.unwrap_or(4) as usize;
	moving_min_internal(values.as_ref(), period)
}
