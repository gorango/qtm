use indicators_core::rolling_moving_average as rma_core;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Rolling Moving Average
#[napi]
pub fn rolling_moving_average(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	rma_core(values.as_ref(), period).map_err(|e| napi::Error::from_reason(e.to_string()))
}
