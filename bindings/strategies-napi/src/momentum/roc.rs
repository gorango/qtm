use napi_derive::napi;

use strategies_core::RocConfig;

#[napi]
pub fn roc_strategy(closes: Vec<f64>, config: Option<RocConfig>) -> napi::Result<Vec<i8>> {
	strategies_core::roc_strategy(&closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}

pub fn roc_strategy_metadata() -> serde_json::Value {
	strategies_core::roc_strategy_metadata()
}

pub fn roc_strategy_defaults() -> serde_json::Value {
	strategies_core::roc_strategy_defaults()
}

pub fn roc(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<RocConfig>(c).unwrap_or_default());
	strategies_core::roc_strategy(&input.closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}
