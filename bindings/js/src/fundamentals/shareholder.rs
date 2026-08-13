use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::validation::validate_non_empty;
use factors_core::FundamentalPoint;
use strategies_core::DividendConfig;
use strategies_core::DividendGrowthConsistencyConfig;

#[napi]
pub fn dividend_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<DividendConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::dividend_strategy(points, config))
}

#[napi]
pub fn dividend_growth_consistency_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<DividendGrowthConsistencyConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::dividend_growth_consistency_strategy(
		points, config,
	))
}

#[napi]
pub fn dividend_strategy_metadata() -> serde_json::Value {
	strategies_core::dividend_strategy_metadata()
}

#[napi]
pub fn dividend_strategy_defaults() -> serde_json::Value {
	strategies_core::dividend_strategy_defaults()
}

#[napi]
pub fn dividend_growth_consistency_strategy_metadata() -> serde_json::Value {
	strategies_core::dividend_growth_consistency_strategy_metadata()
}

#[napi]
pub fn dividend_growth_consistency_strategy_defaults() -> serde_json::Value {
	strategies_core::dividend_growth_consistency_strategy_defaults()
}
