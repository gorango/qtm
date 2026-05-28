use indicators_core::trix as trix_core;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Trix
#[napi]
pub fn trix(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	trix_core(values.as_ref(), period).map_err(|e| napi::Error::from_reason(e.to_string()))
}
