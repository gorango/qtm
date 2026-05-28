use indicators_core::{volume_surge as vs_core, vs as vs_alias, VolumeSurgeConfig};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Volume Surge
#[napi]
pub fn volume_surge(volumes: Float64Array, config: Option<VolumeSurgeConfig>) -> Vec<bool> {
	vs_core(volumes.as_ref(), config)
}

/// Vs
#[napi]
pub fn vs(volumes: Float64Array, config: Option<VolumeSurgeConfig>) -> Vec<bool> {
	vs_alias(volumes.as_ref(), config)
}
