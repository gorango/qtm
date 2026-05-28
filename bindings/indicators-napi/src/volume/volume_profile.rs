use indicators_core::{volume_profile as vp_core, VolumeProfileResult};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Volume Profile
#[napi]
pub fn volume_profile(
	highs: Float64Array,
	lows: Float64Array,
	volumes: Float64Array,
	bins: Option<u32>,
) -> VolumeProfileResult {
	vp_core(highs.as_ref(), lows.as_ref(), volumes.as_ref(), bins)
}
