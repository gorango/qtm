use napi_derive::napi;

use strategies_core::ChandeForecastOscillatorConfig;

#[napi]
pub fn chande_forecast_oscillator_strategy(
	closes: Vec<f64>,
	config: Option<ChandeForecastOscillatorConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::chande_forecast_oscillator_strategy(&closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}

pub fn chande_forecast_oscillator_strategy_metadata() -> serde_json::Value {
	strategies_core::chande_forecast_oscillator_strategy_metadata()
}

pub fn chande_forecast_oscillator_strategy_defaults() -> serde_json::Value {
	strategies_core::chande_forecast_oscillator_strategy_defaults()
}

pub fn chande_forecast_oscillator(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config = config
		.map(|c| serde_json::from_value::<ChandeForecastOscillatorConfig>(c).unwrap_or_default());
	strategies_core::chande_forecast_oscillator_strategy(&input.closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}
