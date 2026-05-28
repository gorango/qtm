use napi_derive::napi;

use strategies_core::StandardDeviationConfig;

#[napi]
pub fn standard_deviation_strategy(
	closes: Vec<f64>,
	config: Option<StandardDeviationConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::standard_deviation_strategy(&closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}

pub fn standard_deviation_strategy_metadata() -> serde_json::Value {
	strategies_core::standard_deviation_strategy_metadata()
}

pub fn standard_deviation_strategy_defaults() -> serde_json::Value {
	strategies_core::standard_deviation_strategy_defaults()
}

pub fn standard_deviation(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<StandardDeviationConfig>(c).unwrap_or_default());
	strategies_core::standard_deviation_strategy(&input.closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}
