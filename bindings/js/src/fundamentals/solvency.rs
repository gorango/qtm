use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::validation::validate_non_empty;
use factors_core::FundamentalPoint;
use strategies_core::SolvencyConfig;

#[napi]
pub fn solvency_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<SolvencyConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::solvency_strategy(points, config))
}

#[napi]
pub fn solvency_strategy_metadata() -> serde_json::Value {
	strategies_core::solvency_strategy_metadata()
}

#[napi]
pub fn solvency_strategy_defaults() -> serde_json::Value {
	strategies_core::solvency_strategy_defaults()
}
