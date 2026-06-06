use napi_derive::napi;

use factors_core::FundamentalPoint;
use strategies_core::DividendConfig;
use strategies_core::DividendGrowthConsistencyConfig;

#[napi]
pub fn dividend_strategy(points: Vec<FundamentalPoint>, config: Option<DividendConfig>) -> Vec<i8> {
	strategies_core::dividend_strategy(points, config)
}

#[napi]
pub fn dividend_growth_consistency_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<DividendGrowthConsistencyConfig>,
) -> Vec<i8> {
	strategies_core::dividend_growth_consistency_strategy(points, config)
}

pub fn dividend_strategy_metadata() -> serde_json::Value {
	strategies_core::dividend_strategy_metadata()
}

pub fn dividend_strategy_defaults() -> serde_json::Value {
	strategies_core::dividend_strategy_defaults()
}

pub fn dividend_growth_consistency_strategy_metadata() -> serde_json::Value {
	strategies_core::dividend_growth_consistency_strategy_metadata()
}

pub fn dividend_growth_consistency_strategy_defaults() -> serde_json::Value {
	strategies_core::dividend_growth_consistency_strategy_defaults()
}
