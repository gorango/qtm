use napi_derive::napi;

use strategies_core::MaCrossoverConfig;

#[napi]
pub fn ma_crossover_strategy(
	closes: Vec<f64>,
	config: Option<MaCrossoverConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::ma_crossover_strategy(&closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}

pub fn ma_crossover_strategy_metadata() -> serde_json::Value {
	strategies_core::ma_crossover_strategy_metadata()
}

pub fn ma_crossover_strategy_defaults() -> serde_json::Value {
	strategies_core::ma_crossover_strategy_defaults()
}

pub fn ma_crossover(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<MaCrossoverConfig>(c).unwrap_or_default());
	strategies_core::ma_crossover_strategy(&input.closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}
