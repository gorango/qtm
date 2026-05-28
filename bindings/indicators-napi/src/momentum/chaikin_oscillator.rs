use indicators_core::{
	chaikin_oscillator as co_core, cmo as cmo_core, ChaikinOscillatorConfig,
	ChaikinOscillatorResult,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Chaikin Oscillator
#[napi]
pub fn chaikin_oscillator(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	volumes: Float64Array,
	config: Option<ChaikinOscillatorConfig>,
) -> ChaikinOscillatorResult {
	co_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		volumes.as_ref(),
		config,
	)
}

/// Cmo
#[napi]
pub fn cmo(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	volumes: Float64Array,
	config: Option<ChaikinOscillatorConfig>,
) -> ChaikinOscillatorResult {
	cmo_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		volumes.as_ref(),
		config,
	)
}
