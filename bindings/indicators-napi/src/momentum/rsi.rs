use indicators_core::{rsi as rsi_core, RSIConfig};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Rsi
#[napi]
pub fn rsi(closings: Float64Array, config: Option<RSIConfig>) -> Vec<f64> {
	rsi_core(closings.as_ref(), config)
}
