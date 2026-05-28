use indicators_core::tema as tema_core;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Tema
#[napi]
pub fn tema(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	tema_core(values.as_ref(), period).map_err(napi::Error::from_reason)
}
