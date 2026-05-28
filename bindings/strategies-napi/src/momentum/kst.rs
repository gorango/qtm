use napi_derive::napi;

use strategies_core::KSTConfig;

#[napi]
pub fn kst_strategy(closes: Vec<f64>, config: Option<KSTConfig>) -> napi::Result<Vec<i8>> {
	strategies_core::kst_strategy(&closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}

pub fn kst_strategy_metadata() -> serde_json::Value {
	strategies_core::kst_strategy_metadata()
}

pub fn kst_strategy_defaults() -> serde_json::Value {
	strategies_core::kst_strategy_defaults()
}

pub fn kst(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<KSTConfig>(c).unwrap_or_default());
	strategies_core::kst_strategy(&input.closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}
