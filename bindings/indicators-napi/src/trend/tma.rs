use indicators_core::tma as tma_core;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Tma
#[napi]
pub fn tma(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	tma_core(values.as_ref(), period).map_err(napi::Error::from_reason)
}
