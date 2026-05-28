use indicators_core::hma as hma_core;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Hma
#[napi]
pub fn hma(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	hma_core(values.as_ref(), period).map_err(napi::Error::from_reason)
}
