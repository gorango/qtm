use indicators_core::dema as dema_core;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Dema
#[napi]
pub fn dema(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	dema_core(values.as_ref(), period).map_err(|e| napi::Error::from_reason(e.to_string()))
}
