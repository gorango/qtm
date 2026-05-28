use indicators_core::vwma as vwma_core;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Vwma
#[napi]
pub fn vwma(closes: Float64Array, volumes: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	vwma_core(closes.as_ref(), volumes.as_ref(), period).map_err(napi::Error::from_reason)
}
