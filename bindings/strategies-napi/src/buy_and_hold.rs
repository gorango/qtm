use napi_derive::napi;

use strategies_core::BuyAndHoldConfig;

#[napi]
pub fn buy_and_hold_strategy(
	closes: Vec<f64>,
	config: Option<BuyAndHoldConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::buy_and_hold_strategy(&closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}

pub fn buy_and_hold_strategy_metadata() -> serde_json::Value {
	strategies_core::buy_and_hold_strategy_metadata()
}

pub fn buy_and_hold_strategy_defaults() -> serde_json::Value {
	strategies_core::buy_and_hold_strategy_defaults()
}

pub fn buy_and_hold(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<BuyAndHoldConfig>(c).unwrap_or_default());
	strategies_core::buy_and_hold_strategy(&input.closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}
