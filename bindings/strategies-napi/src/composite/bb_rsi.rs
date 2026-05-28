use napi_derive::napi;

use strategies_core::BbRsiConfig;

#[napi]
pub fn bb_rsi_strategy(closes: Vec<f64>, config: Option<BbRsiConfig>) -> napi::Result<Vec<i8>> {
	strategies_core::bb_rsi_strategy(&closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}

pub fn bb_rsi_strategy_metadata() -> serde_json::Value {
	strategies_core::bb_rsi_strategy_metadata()
}

pub fn bb_rsi_strategy_defaults() -> serde_json::Value {
	strategies_core::bb_rsi_strategy_defaults()
}

pub fn bb_rsi(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<BbRsiConfig>(c).unwrap_or_default());
	strategies_core::bb_rsi_strategy(&input.closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}
