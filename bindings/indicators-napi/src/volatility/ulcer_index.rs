use indicators_core::ulcer_index as ui_core;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Ulcer Index
#[napi]
pub fn ulcer_index(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	ui_core(values.as_ref(), period).map_err(|e| napi::Error::from_reason(e.to_string()))
}
