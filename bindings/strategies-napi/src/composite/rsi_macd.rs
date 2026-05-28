use napi_derive::napi;

use strategies_core::RsiMacdConfig;

#[napi]
pub fn rsi_macd_strategy(closes: Vec<f64>, config: Option<RsiMacdConfig>) -> napi::Result<Vec<i8>> {
	strategies_core::rsi_macd_strategy(&closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}

pub fn rsi_macd_strategy_metadata() -> serde_json::Value {
	strategies_core::rsi_macd_strategy_metadata()
}

pub fn rsi_macd_strategy_defaults() -> serde_json::Value {
	strategies_core::rsi_macd_strategy_defaults()
}

pub fn rsi_macd(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<RsiMacdConfig>(c).unwrap_or_default());
	strategies_core::rsi_macd_strategy(&input.closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}
