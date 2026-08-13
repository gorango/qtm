use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::validation::validate_non_empty;
use factors_core::Bar;
use factors_core::FactorPoint;
use strategies_core::DualMomentumConfig;
use strategies_core::MultiFactorConfig;
use strategies_core::RiskParityConfig;

#[napi]
pub fn multi_factor_strategy(
	value_factors: Vec<FactorPoint>,
	quality_factors: Vec<FactorPoint>,
	momentum_factors: Vec<FactorPoint>,
	prices: Vec<Bar>,
	config: Option<MultiFactorConfig>,
) -> Result<Vec<f64>> {
	validate_non_empty(&value_factors, "value_factors")?;
	validate_non_empty(&quality_factors, "quality_factors")?;
	validate_non_empty(&momentum_factors, "momentum_factors")?;
	validate_non_empty(&prices, "prices")?;
	Ok(strategies_core::multi_factor_strategy(
		value_factors,
		quality_factors,
		momentum_factors,
		prices,
		config,
	))
}

#[napi]
pub fn risk_parity_strategy(
	closes: Vec<Vec<f64>>,
	config: Option<RiskParityConfig>,
) -> Result<Vec<f64>> {
	validate_non_empty(&closes, "closes")?;
	Ok(strategies_core::risk_parity_strategy(closes, config))
}

#[napi]
pub fn dual_momentum_strategy(
	closes: Vec<Vec<f64>>,
	config: Option<DualMomentumConfig>,
) -> Result<Vec<f64>> {
	validate_non_empty(&closes, "closes")?;
	Ok(strategies_core::dual_momentum_strategy(closes, config))
}

#[napi]
pub fn multi_factor_strategy_metadata() -> serde_json::Value {
	strategies_core::multi_factor_strategy_metadata()
}

#[napi]
pub fn risk_parity_strategy_metadata() -> serde_json::Value {
	strategies_core::risk_parity_strategy_metadata()
}

#[napi]
pub fn dual_momentum_strategy_metadata() -> serde_json::Value {
	strategies_core::dual_momentum_strategy_metadata()
}

#[napi]
pub fn multi_factor_strategy_defaults() -> serde_json::Value {
	strategies_core::multi_factor_strategy_defaults()
}

#[napi]
pub fn risk_parity_strategy_defaults() -> serde_json::Value {
	strategies_core::risk_parity_strategy_defaults()
}

#[napi]
pub fn dual_momentum_strategy_defaults() -> serde_json::Value {
	strategies_core::dual_momentum_strategy_defaults()
}
