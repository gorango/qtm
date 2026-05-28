use napi_derive::napi;

use strategies_core::PairsTradingConfig;

#[napi]
pub fn pairs_trading_strategy(
	closes: Vec<f64>,
	config: Option<PairsTradingConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::pairs_trading_strategy(&closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}

pub fn pairs_trading_strategy_metadata() -> serde_json::Value {
	strategies_core::pairs_trading_strategy_metadata()
}

pub fn pairs_trading_strategy_defaults() -> serde_json::Value {
	strategies_core::pairs_trading_strategy_defaults()
}

pub fn pairs_trading(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<PairsTradingConfig>(c).unwrap_or_default());
	strategies_core::pairs_trading_strategy(&input.closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}
