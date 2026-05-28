use napi_derive::napi;

use strategies_core::HmaTrendConfig;

#[napi]
pub fn hma_trend_strategy(
	closes: Vec<f64>,
	config: Option<HmaTrendConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::hma_trend_strategy(&closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}

pub fn hma_trend_strategy_metadata() -> serde_json::Value {
	strategies_core::hma_trend_strategy_metadata()
}

pub fn hma_trend_strategy_defaults() -> serde_json::Value {
	strategies_core::hma_trend_strategy_defaults()
}

pub fn hma_trend(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<HmaTrendConfig>(c).unwrap_or_default());
	strategies_core::hma_trend_strategy(&input.closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}
