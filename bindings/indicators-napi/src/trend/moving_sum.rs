use indicators_core::internal::moving_sum::moving_sum_internal;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Moving Sum
#[napi]
pub fn moving_sum(values: Float64Array, period: Option<u32>) -> Vec<f64> {
	let period = period.unwrap_or(4) as usize;
	moving_sum_internal(values.as_ref(), period)
}
