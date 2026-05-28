use indicators_core::{qstick as qstick_core, QstickConfig};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Qstick
#[napi]
pub fn qstick(opens: Float64Array, closes: Float64Array, config: Option<QstickConfig>) -> Vec<f64> {
	qstick_core(opens.as_ref(), closes.as_ref(), config)
}
