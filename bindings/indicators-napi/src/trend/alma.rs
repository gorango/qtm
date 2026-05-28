use indicators_core::{alma as alma_core, ALMAConfig};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Alma
#[napi]
pub fn alma(values: Float64Array, config: Option<ALMAConfig>) -> Result<Vec<f64>> {
	alma_core(values.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}
