use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::validation::validate_non_empty;
use factors_core::FundamentalPoint;
use strategies_core::HighYieldReitConfig;

#[napi]
pub fn high_yield_reit_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<HighYieldReitConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::high_yield_reit_strategy(points, config))
}

#[napi]
pub fn high_yield_reit_strategy_metadata() -> serde_json::Value {
	strategies_core::high_yield_reit_strategy_metadata()
}

#[napi]
pub fn high_yield_reit_strategy_defaults() -> serde_json::Value {
	strategies_core::high_yield_reit_strategy_defaults()
}
