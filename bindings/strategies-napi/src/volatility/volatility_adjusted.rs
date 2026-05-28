use napi_derive::napi;

use strategies_core::VolatilityAdjustedConfig;

#[napi]
pub fn volatility_adjusted_strategy(
	closes: Vec<f64>,
	config: Option<VolatilityAdjustedConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::volatility_adjusted_strategy(&closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}

pub fn volatility_adjusted_strategy_metadata() -> serde_json::Value {
	strategies_core::volatility_adjusted_strategy_metadata()
}

pub fn volatility_adjusted_strategy_defaults() -> serde_json::Value {
	strategies_core::volatility_adjusted_strategy_defaults()
}

pub fn volatility_adjusted(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<VolatilityAdjustedConfig>(c).unwrap_or_default());
	strategies_core::volatility_adjusted_strategy(&input.closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}
