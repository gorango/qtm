use indicators_core::trend::moving_max::moving_max_internal;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Moving Max
#[napi]
pub fn moving_max(values: Float64Array, period: Option<u32>) -> Vec<f64> {
	let period = period.unwrap_or(4) as usize;
	moving_max_internal(values.as_ref(), period)
}
