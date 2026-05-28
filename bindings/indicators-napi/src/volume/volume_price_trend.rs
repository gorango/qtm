use indicators_core::volume::volume_price_trend::{
	volume_price_trend as vpt_core, vpt as vpt_alias,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Volume Price Trend
#[napi]
pub fn volume_price_trend(closings: Float64Array, volumes: Float64Array) -> Vec<f64> {
	vpt_core(closings.as_ref(), volumes.as_ref())
}

/// Vpt
#[napi]
pub fn vpt(closings: Float64Array, volumes: Float64Array) -> Vec<f64> {
	vpt_alias(closings.as_ref(), volumes.as_ref())
}
