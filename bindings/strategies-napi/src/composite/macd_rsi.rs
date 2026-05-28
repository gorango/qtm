use napi_derive::napi;

use strategies_core::MACDConfig;
use strategies_core::RSIConfig;

#[napi]
pub fn macd_rsi_strategy(
	closes: Vec<f64>,
	macd_config: Option<MACDConfig>,
	rsi_config: Option<RSIConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::macd_rsi_strategy(&closes, macd_config, rsi_config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}

pub fn macd_rsi_strategy_metadata() -> serde_json::Value {
	strategies_core::macd_rsi_strategy_metadata()
}

pub fn macd_rsi_strategy_defaults() -> serde_json::Value {
	strategies_core::macd_rsi_strategy_defaults()
}

pub fn macd_rsi(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<MACDConfig>(c).unwrap_or_default());
	strategies_core::macd_rsi_strategy(&input.closes, config, None)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}
