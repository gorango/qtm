use indicators_core::volume::obv::{obv as obv_core, on_balance_volume as obv_alias};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Obv
#[napi]
pub fn obv(closings: Float64Array, volumes: Float64Array) -> Vec<f64> {
	obv_core(closings.as_ref(), volumes.as_ref())
}

/// On Balance Volume
#[napi]
pub fn on_balance_volume(closings: Float64Array, volumes: Float64Array) -> Vec<f64> {
	obv_alias(closings.as_ref(), volumes.as_ref())
}
