use napi_derive::napi;

use strategies_core::WmaMomentumConfig;

#[napi]
pub fn wma_momentum_strategy(
	closes: Vec<f64>,
	config: Option<WmaMomentumConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::wma_momentum_strategy(&closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}

pub fn wma_momentum_strategy_metadata() -> serde_json::Value {
	strategies_core::wma_momentum_strategy_metadata()
}

pub fn wma_momentum_strategy_defaults() -> serde_json::Value {
	strategies_core::wma_momentum_strategy_defaults()
}

pub fn wma_momentum(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<WmaMomentumConfig>(c).unwrap_or_default());
	strategies_core::wma_momentum_strategy(&input.closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}
