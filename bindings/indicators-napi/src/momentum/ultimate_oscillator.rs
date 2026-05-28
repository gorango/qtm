use indicators_core::{ultimate_oscillator as uo_core, uo as uo_alias, UltimateOscillatorConfig};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Ultimate Oscillator
#[napi]
pub fn ultimate_oscillator(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	config: Option<UltimateOscillatorConfig>,
) -> Vec<f64> {
	uo_core(highs.as_ref(), lows.as_ref(), closings.as_ref(), config)
}

/// Uo
#[napi]
pub fn uo(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	config: Option<UltimateOscillatorConfig>,
) -> Vec<f64> {
	uo_alias(highs.as_ref(), lows.as_ref(), closings.as_ref(), config)
}
