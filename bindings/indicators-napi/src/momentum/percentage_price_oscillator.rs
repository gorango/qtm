use indicators_core::{
	percentage_price_oscillator as ppo_core, PercentagePriceOscillatorConfig,
	PercentagePriceOscillatorResult,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Percentage Price Oscillator
#[napi]
pub fn percentage_price_oscillator(
	prices: Float64Array,
	config: Option<PercentagePriceOscillatorConfig>,
) -> Result<PercentagePriceOscillatorResult> {
	ppo_core(prices.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}
