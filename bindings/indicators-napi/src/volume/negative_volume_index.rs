use indicators_core::volume::negative_volume_index::{
	negative_volume_index as nvi_core, nvi as nvi_alias,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Negative Volume Index
#[napi]
pub fn negative_volume_index(
	closings: Float64Array,
	volumes: Float64Array,
	start: Option<f64>,
) -> Vec<f64> {
	nvi_core(closings.as_ref(), volumes.as_ref(), start)
}

/// Nvi
#[napi]
pub fn nvi(closings: Float64Array, volumes: Float64Array, start: Option<f64>) -> Vec<f64> {
	nvi_alias(closings.as_ref(), volumes.as_ref(), start)
}
