use napi_derive::napi;

use strategies_core::MacdCrossoverConfig;

#[napi]
pub fn macd_crossover_strategy(
	closes: Vec<f64>,
	config: Option<MacdCrossoverConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::macd_crossover_strategy(&closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}

pub fn macd_crossover_strategy_metadata() -> serde_json::Value {
	strategies_core::macd_crossover_strategy_metadata()
}

pub fn macd_crossover_strategy_defaults() -> serde_json::Value {
	strategies_core::macd_crossover_strategy_defaults()
}

pub fn macd_crossover(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<MacdCrossoverConfig>(c).unwrap_or_default());
	strategies_core::macd_crossover_strategy(&input.closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}
