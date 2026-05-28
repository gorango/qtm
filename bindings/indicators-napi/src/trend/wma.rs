use indicators_core::wma as wma_core;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Wma
#[napi]
pub fn wma(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	wma_core(values.as_ref(), period).map_err(napi::Error::from_reason)
}
