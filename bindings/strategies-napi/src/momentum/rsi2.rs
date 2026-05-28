use napi_derive::napi;

use strategies_core::Rsi2Config;

#[napi]
pub fn rsi2_strategy(closes: Vec<f64>, config: Option<Rsi2Config>) -> napi::Result<Vec<i8>> {
	strategies_core::rsi2_strategy(&closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}

pub fn rsi2_strategy_metadata() -> serde_json::Value {
	strategies_core::rsi2_strategy_metadata()
}

pub fn rsi2_strategy_defaults() -> serde_json::Value {
	strategies_core::rsi2_strategy_defaults()
}

pub fn rsi2(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<Rsi2Config>(c).unwrap_or_default());
	strategies_core::rsi2_strategy(&input.closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}
