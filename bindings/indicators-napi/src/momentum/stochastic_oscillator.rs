use indicators_core::{stochastic_oscillator as stoch_core, StochConfig, StochResult};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Stochastic Oscillator
#[napi]
pub fn stochastic_oscillator(
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
	config: Option<StochConfig>,
) -> StochResult {
	stoch_core(highs.as_ref(), lows.as_ref(), closes.as_ref(), config)
}
