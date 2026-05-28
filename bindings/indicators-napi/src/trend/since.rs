use indicators_core::trend::since::since_internal;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Since
#[napi]
pub fn since(values: Float64Array) -> Vec<f64> {
	since_internal(values.as_ref())
}
