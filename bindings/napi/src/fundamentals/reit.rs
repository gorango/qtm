use napi_derive::napi;

use factors_core::FundamentalPoint;
use strategies_core::HighYieldReitConfig;

#[napi]
pub fn high_yield_reit_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<HighYieldReitConfig>,
) -> Vec<i8> {
	strategies_core::high_yield_reit_strategy(points, config)
}

pub fn high_yield_reit_strategy_metadata() -> serde_json::Value {
	strategies_core::high_yield_reit_strategy_metadata()
}

pub fn high_yield_reit_strategy_defaults() -> serde_json::Value {
	strategies_core::high_yield_reit_strategy_defaults()
}
