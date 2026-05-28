use napi_derive::napi;

use strategies_core::VarianceStopConfig;

#[napi]
pub fn variance_stop_strategy(
	closes: Vec<f64>,
	config: Option<VarianceStopConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::variance_stop_strategy(&closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}

pub fn variance_stop_strategy_metadata() -> serde_json::Value {
	strategies_core::variance_stop_strategy_metadata()
}

pub fn variance_stop_strategy_defaults() -> serde_json::Value {
	strategies_core::variance_stop_strategy_defaults()
}

pub fn variance_stop(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<VarianceStopConfig>(c).unwrap_or_default());
	strategies_core::variance_stop_strategy(&input.closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}
