use napi_derive::napi;

use strategies_core::CorrelationPairConfig;

#[napi]
pub fn correlation_pair_strategy(
	closes: Vec<f64>,
	config: Option<CorrelationPairConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::correlation_pair_strategy(&closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}

pub fn correlation_pair_strategy_metadata() -> serde_json::Value {
	strategies_core::correlation_pair_strategy_metadata()
}

pub fn correlation_pair_strategy_defaults() -> serde_json::Value {
	strategies_core::correlation_pair_strategy_defaults()
}

pub fn correlation_pair(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<CorrelationPairConfig>(c).unwrap_or_default());
	strategies_core::correlation_pair_strategy(&input.closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}
