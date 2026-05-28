use napi_derive::napi;

use strategies_core::FibonacciRetracementConfig;

#[napi]
pub fn fibonacci_retracement_strategy(
	closes: Vec<f64>,
	config: Option<FibonacciRetracementConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::fibonacci_retracement_strategy(&closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}

pub fn fibonacci_retracement_strategy_metadata() -> serde_json::Value {
	strategies_core::fibonacci_retracement_strategy_metadata()
}

pub fn fibonacci_retracement_strategy_defaults() -> serde_json::Value {
	strategies_core::fibonacci_retracement_strategy_defaults()
}

pub fn fibonacci_retracement(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<FibonacciRetracementConfig>(c).unwrap_or_default());
	strategies_core::fibonacci_retracement_strategy(&input.closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}
