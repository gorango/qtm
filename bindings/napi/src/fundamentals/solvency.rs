use napi_derive::napi;

use factors_core::FundamentalPoint;
use strategies_core::SolvencyConfig;

#[napi]
pub fn solvency_strategy(points: Vec<FundamentalPoint>, config: Option<SolvencyConfig>) -> Vec<i8> {
	strategies_core::solvency_strategy(points, config)
}

pub fn solvency_strategy_metadata() -> serde_json::Value {
	strategies_core::solvency_strategy_metadata()
}

pub fn solvency_strategy_defaults() -> serde_json::Value {
	strategies_core::solvency_strategy_defaults()
}
