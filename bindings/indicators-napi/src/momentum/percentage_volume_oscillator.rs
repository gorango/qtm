use indicators_core::{
	percentage_volume_oscillator as pvo_core, pvo as pvo_alias, PercentageVolumeOscillatorConfig,
	PercentageVolumeOscillatorResult,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Percentage Volume Oscillator
#[napi]
pub fn percentage_volume_oscillator(
	volumes: Float64Array,
	config: Option<PercentageVolumeOscillatorConfig>,
) -> PercentageVolumeOscillatorResult {
	pvo_core(volumes.as_ref(), config)
}

/// Pvo
#[napi]
pub fn pvo(
	volumes: Float64Array,
	config: Option<PercentageVolumeOscillatorConfig>,
) -> PercentageVolumeOscillatorResult {
	pvo_alias(volumes.as_ref(), config)
}
