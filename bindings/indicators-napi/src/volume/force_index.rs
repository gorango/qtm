use indicators_core::{fi as fi_alias, force_index as fi_core, FIConfig};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Force Index
#[napi]
pub fn force_index(
	closings: Float64Array,
	volumes: Float64Array,
	config: Option<FIConfig>,
) -> Vec<f64> {
	fi_core(closings.as_ref(), volumes.as_ref(), config)
}

/// Fi
#[napi]
pub fn fi(closings: Float64Array, volumes: Float64Array, config: Option<FIConfig>) -> Vec<f64> {
	fi_alias(closings.as_ref(), volumes.as_ref(), config)
}
