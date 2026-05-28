use indicators_core::smoothed_moving_average as smma_core;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Smoothed Moving Average
#[napi]
pub fn smoothed_moving_average(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	smma_core(values.as_ref(), period).map_err(|e| napi::Error::from_reason(e.to_string()))
}
