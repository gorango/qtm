use napi_derive::napi;

use strategies_core::RSIConfig;

#[napi]
pub fn rsi_strategy(closes: Vec<f64>, config: Option<RSIConfig>) -> napi::Result<Vec<i8>> {
	strategies_core::rsi_strategy(&closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}

pub fn rsi_strategy_metadata() -> serde_json::Value {
	strategies_core::rsi_strategy_metadata()
}

pub fn rsi_strategy_defaults() -> serde_json::Value {
	strategies_core::rsi_strategy_defaults()
}

pub fn rsi(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<RSIConfig>(c).unwrap_or_default());
	strategies_core::rsi_strategy(&input.closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}
