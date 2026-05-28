use indicators_core::market::mcclellan_oscillator::mcclellan_oscillator as mo_core;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// McClellan Oscillator
#[napi]
pub fn mcclellan_oscillator(advances: Float64Array, declines: Float64Array) -> Result<Vec<f64>> {
	mo_core(advances.as_ref(), declines.as_ref()).map_err(napi::Error::from_reason)
}
