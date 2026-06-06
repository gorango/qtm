use napi_derive::napi;

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
) -> Vec<f64> {
	strategies_core::multi_factor_strategy(
		value_factors,
		quality_factors,
		momentum_factors,
		prices,
		config,
	)
}

#[napi]
pub fn risk_parity_strategy(closes: Vec<Vec<f64>>, config: Option<RiskParityConfig>) -> Vec<f64> {
	strategies_core::risk_parity_strategy(closes, config)
}

#[napi]
pub fn dual_momentum_strategy(
	closes: Vec<Vec<f64>>,
	config: Option<DualMomentumConfig>,
) -> Vec<f64> {
	strategies_core::dual_momentum_strategy(closes, config)
}

pub fn multi_factor_strategy_metadata() -> serde_json::Value {
	strategies_core::multi_factor_strategy_metadata()
}

pub fn risk_parity_strategy_metadata() -> serde_json::Value {
	strategies_core::risk_parity_strategy_metadata()
}

pub fn dual_momentum_strategy_metadata() -> serde_json::Value {
	strategies_core::dual_momentum_strategy_metadata()
}

pub fn multi_factor_strategy_defaults() -> serde_json::Value {
	strategies_core::multi_factor_strategy_defaults()
}

pub fn risk_parity_strategy_defaults() -> serde_json::Value {
	strategies_core::risk_parity_strategy_defaults()
}

pub fn dual_momentum_strategy_defaults() -> serde_json::Value {
	strategies_core::dual_momentum_strategy_defaults()
}
