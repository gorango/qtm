use napi_derive::napi;

use strategies_core::PercentRankConfig;

#[napi]
pub fn percent_rank_strategy(
	closes: Vec<f64>,
	config: Option<PercentRankConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::percent_rank_strategy(&closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}

pub fn percent_rank_strategy_metadata() -> serde_json::Value {
	strategies_core::percent_rank_strategy_metadata()
}

pub fn percent_rank_strategy_defaults() -> serde_json::Value {
	strategies_core::percent_rank_strategy_defaults()
}

pub fn percent_rank(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<PercentRankConfig>(c).unwrap_or_default());
	strategies_core::percent_rank_strategy(&input.closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}
