use napi_derive::napi;

use strategies_core::CointegrationConfig;

#[napi]
pub fn cointegration_strategy(
	closes: Vec<f64>,
	config: Option<CointegrationConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::cointegration_strategy(&closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}

pub fn cointegration_strategy_metadata() -> serde_json::Value {
	strategies_core::cointegration_strategy_metadata()
}

pub fn cointegration_strategy_defaults() -> serde_json::Value {
	strategies_core::cointegration_strategy_defaults()
}

pub fn cointegration(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<CointegrationConfig>(c).unwrap_or_default());
	strategies_core::cointegration_strategy(&input.closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}
