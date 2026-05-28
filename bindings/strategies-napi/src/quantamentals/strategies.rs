use napi_derive::napi;

use factors_core::Bar;
use factors_core::FundamentalPoint;
use factors_core::OnChainDataPoint;
use factors_core::PredictionMarketPoint;
use strategies_core::AlternativeDataConfig;
use strategies_core::CompositeValueMomentumConfig;
use strategies_core::EventDrivenConfig;
use strategies_core::GrowthQualityConfig;
use strategies_core::MultiFactorValueConfig;
use strategies_core::OnChainConfirmationConfig;
use strategies_core::QarpConfig;
use strategies_core::QuantamentalValueMomentumConfig;
use strategies_core::ValueMomentumPatternConfig;

#[napi]
pub fn qarp_strategy(
	fundamentals: Vec<FundamentalPoint>,
	prices: Vec<Bar>,
	config: Option<QarpConfig>,
) -> Vec<i8> {
	strategies_core::qarp_strategy(fundamentals, prices, config)
}

#[napi]
pub fn multi_factor_value_strategy(
	fundamentals: Vec<FundamentalPoint>,
	prices: Vec<Bar>,
	config: Option<MultiFactorValueConfig>,
) -> Vec<i8> {
	strategies_core::multi_factor_value_strategy(fundamentals, prices, config)
}

#[napi]
pub fn alternative_data_strategy(
	on_chain_data: Vec<OnChainDataPoint>,
	prediction_data: Vec<PredictionMarketPoint>,
	config: Option<AlternativeDataConfig>,
) -> Vec<i8> {
	strategies_core::alternative_data_strategy(on_chain_data, prediction_data, config)
}

#[napi]
pub fn event_driven_strategy(
	prediction_data: Vec<PredictionMarketPoint>,
	prices: Vec<Bar>,
	config: Option<EventDrivenConfig>,
) -> Vec<i8> {
	strategies_core::event_driven_strategy(prediction_data, prices, config)
}

#[napi]
pub fn on_chain_confirmation_strategy(
	on_chain_data: Vec<OnChainDataPoint>,
	prices: Vec<Bar>,
	config: Option<OnChainConfirmationConfig>,
) -> Vec<i8> {
	strategies_core::on_chain_confirmation_strategy(on_chain_data, prices, config)
}

#[napi]
pub fn value_momentum_pattern_strategy(
	fundamentals: Vec<FundamentalPoint>,
	prices: Vec<Bar>,
	config: Option<ValueMomentumPatternConfig>,
) -> Vec<i8> {
	strategies_core::value_momentum_pattern_strategy(fundamentals, prices, config)
}

#[napi]
pub fn growth_quality_strategy(
	fundamentals: Vec<FundamentalPoint>,
	prices: Vec<Bar>,
	config: Option<GrowthQualityConfig>,
) -> Vec<i8> {
	strategies_core::growth_quality_strategy(fundamentals, prices, config)
}

#[napi]
pub fn composite_value_momentum_strategy(
	fundamentals: Vec<FundamentalPoint>,
	prices: Vec<Bar>,
	config: Option<CompositeValueMomentumConfig>,
) -> Vec<i8> {
	strategies_core::composite_value_momentum_strategy(fundamentals, prices, config)
}

#[napi]
pub fn quantamental_value_momentum_strategy(
	fundamentals: Vec<FundamentalPoint>,
	prices: Vec<Bar>,
	config: Option<QuantamentalValueMomentumConfig>,
) -> Vec<i8> {
	strategies_core::quantamental_value_momentum_strategy(fundamentals, prices, config)
}

pub fn qarp_strategy_metadata() -> serde_json::Value {
	strategies_core::qarp_strategy_metadata()
}

pub fn multi_factor_value_strategy_metadata() -> serde_json::Value {
	strategies_core::multi_factor_value_strategy_metadata()
}

pub fn alternative_data_strategy_metadata() -> serde_json::Value {
	strategies_core::alternative_data_strategy_metadata()
}

pub fn event_driven_strategy_metadata() -> serde_json::Value {
	strategies_core::event_driven_strategy_metadata()
}

pub fn on_chain_confirmation_strategy_metadata() -> serde_json::Value {
	strategies_core::on_chain_confirmation_strategy_metadata()
}

pub fn value_momentum_pattern_strategy_metadata() -> serde_json::Value {
	strategies_core::value_momentum_pattern_strategy_metadata()
}

pub fn growth_quality_strategy_metadata() -> serde_json::Value {
	strategies_core::growth_quality_strategy_metadata()
}

pub fn composite_value_momentum_strategy_metadata() -> serde_json::Value {
	strategies_core::composite_value_momentum_strategy_metadata()
}

pub fn quantamental_value_momentum_strategy_metadata() -> serde_json::Value {
	strategies_core::quantamental_value_momentum_strategy_metadata()
}

pub fn qarp_strategy_defaults() -> serde_json::Value {
	strategies_core::qarp_strategy_defaults()
}

pub fn multi_factor_value_strategy_defaults() -> serde_json::Value {
	strategies_core::multi_factor_value_strategy_defaults()
}

pub fn alternative_data_strategy_defaults() -> serde_json::Value {
	strategies_core::alternative_data_strategy_defaults()
}

pub fn event_driven_strategy_defaults() -> serde_json::Value {
	strategies_core::event_driven_strategy_defaults()
}

pub fn on_chain_confirmation_strategy_defaults() -> serde_json::Value {
	strategies_core::on_chain_confirmation_strategy_defaults()
}

pub fn value_momentum_pattern_strategy_defaults() -> serde_json::Value {
	strategies_core::value_momentum_pattern_strategy_defaults()
}

pub fn growth_quality_strategy_defaults() -> serde_json::Value {
	strategies_core::growth_quality_strategy_defaults()
}

pub fn composite_value_momentum_strategy_defaults() -> serde_json::Value {
	strategies_core::composite_value_momentum_strategy_defaults()
}

pub fn quantamental_value_momentum_strategy_defaults() -> serde_json::Value {
	strategies_core::quantamental_value_momentum_strategy_defaults()
}
