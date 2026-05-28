use indicators_core::{awesome_oscillator as ao_core, AwesomeOscillatorConfig};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Awesome Oscillator
#[napi]
pub fn awesome_oscillator(
	highs: Float64Array,
	lows: Float64Array,
	config: Option<AwesomeOscillatorConfig>,
) -> Result<Vec<f64>> {
	ao_core(highs.as_ref(), lows.as_ref(), config)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}
