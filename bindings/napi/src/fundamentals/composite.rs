use napi_derive::napi;

use factors_core::FactorPoint;
use factors_core::FundamentalPoint;
use strategies_core::AltmanZScoreConfig;
use strategies_core::JoelGreenblattConfig;
use strategies_core::MagicFormulaConfig;
use strategies_core::MultiFactorSuiteConfig;
use strategies_core::PiotroskiConfig;
use strategies_core::SuiteConfig;

#[napi]
pub fn altman_z_score_strategy(
	factors: Vec<FactorPoint>,
	config: Option<AltmanZScoreConfig>,
) -> Vec<i8> {
	strategies_core::altman_z_score_strategy(factors, config)
}

#[napi]
pub fn piotroski_strategy(factors: Vec<FactorPoint>, config: Option<PiotroskiConfig>) -> Vec<i8> {
	strategies_core::piotroski_strategy(factors, config)
}

#[napi]
pub fn magic_formula_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<MagicFormulaConfig>,
) -> Vec<i8> {
	strategies_core::magic_formula_strategy(points, config)
}

#[napi]
pub fn joel_greenblatt_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<JoelGreenblattConfig>,
) -> Vec<i8> {
	strategies_core::joel_greenblatt_strategy(points, config)
}

#[napi]
pub fn growth_investing_suite_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<SuiteConfig>,
) -> Vec<i8> {
	strategies_core::growth_investing_suite_strategy(points, config)
}

#[napi]
pub fn quality_investing_suite_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<SuiteConfig>,
) -> Vec<i8> {
	strategies_core::quality_investing_suite_strategy(points, config)
}

#[napi]
pub fn value_investing_suite_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<SuiteConfig>,
) -> Vec<i8> {
	strategies_core::value_investing_suite_strategy(points, config)
}

#[napi]
pub fn multi_factor_suite_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<MultiFactorSuiteConfig>,
) -> Vec<i8> {
	strategies_core::multi_factor_suite_strategy(points, config)
}

pub fn altman_z_score_strategy_metadata() -> serde_json::Value {
	strategies_core::altman_z_score_strategy_metadata()
}

pub fn altman_z_score_strategy_defaults() -> serde_json::Value {
	strategies_core::altman_z_score_strategy_defaults()
}

pub fn piotroski_strategy_metadata() -> serde_json::Value {
	strategies_core::piotroski_strategy_metadata()
}

pub fn piotroski_strategy_defaults() -> serde_json::Value {
	strategies_core::piotroski_strategy_defaults()
}

pub fn magic_formula_strategy_metadata() -> serde_json::Value {
	strategies_core::magic_formula_strategy_metadata()
}

pub fn magic_formula_strategy_defaults() -> serde_json::Value {
	strategies_core::magic_formula_strategy_defaults()
}

pub fn joel_greenblatt_strategy_metadata() -> serde_json::Value {
	strategies_core::joel_greenblatt_strategy_metadata()
}

pub fn joel_greenblatt_strategy_defaults() -> serde_json::Value {
	strategies_core::joel_greenblatt_strategy_defaults()
}

pub fn growth_investing_suite_strategy_metadata() -> serde_json::Value {
	strategies_core::growth_investing_suite_strategy_metadata()
}

pub fn growth_investing_suite_strategy_defaults() -> serde_json::Value {
	strategies_core::growth_investing_suite_strategy_defaults()
}

pub fn quality_investing_suite_strategy_metadata() -> serde_json::Value {
	strategies_core::quality_investing_suite_strategy_metadata()
}

pub fn quality_investing_suite_strategy_defaults() -> serde_json::Value {
	strategies_core::quality_investing_suite_strategy_defaults()
}

pub fn value_investing_suite_strategy_metadata() -> serde_json::Value {
	strategies_core::value_investing_suite_strategy_metadata()
}

pub fn value_investing_suite_strategy_defaults() -> serde_json::Value {
	strategies_core::value_investing_suite_strategy_defaults()
}

pub fn multi_factor_suite_strategy_metadata() -> serde_json::Value {
	strategies_core::multi_factor_suite_strategy_metadata()
}

pub fn multi_factor_suite_strategy_defaults() -> serde_json::Value {
	strategies_core::multi_factor_suite_strategy_defaults()
}
