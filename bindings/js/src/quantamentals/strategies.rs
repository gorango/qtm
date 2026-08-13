use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::validation::validate_non_empty;
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
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	validate_non_empty(&prices, "prices")?;
	Ok(strategies_core::qarp_strategy(fundamentals, prices, config))
}

#[napi]
pub fn multi_factor_value_strategy(
	fundamentals: Vec<FundamentalPoint>,
	prices: Vec<Bar>,
	config: Option<MultiFactorValueConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	validate_non_empty(&prices, "prices")?;
	Ok(strategies_core::multi_factor_value_strategy(
		fundamentals,
		prices,
		config,
	))
}

#[napi]
pub fn alternative_data_strategy(
	on_chain_data: Vec<OnChainDataPoint>,
	prediction_data: Vec<PredictionMarketPoint>,
	config: Option<AlternativeDataConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&on_chain_data, "on_chain_data")?;
	validate_non_empty(&prediction_data, "prediction_data")?;
	Ok(strategies_core::alternative_data_strategy(
		on_chain_data,
		prediction_data,
		config,
	))
}

#[napi]
pub fn event_driven_strategy(
	prediction_data: Vec<PredictionMarketPoint>,
	prices: Vec<Bar>,
	config: Option<EventDrivenConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&prediction_data, "prediction_data")?;
	validate_non_empty(&prices, "prices")?;
	Ok(strategies_core::event_driven_strategy(
		prediction_data,
		prices,
		config,
	))
}

#[napi]
pub fn on_chain_confirmation_strategy(
	on_chain_data: Vec<OnChainDataPoint>,
	prices: Vec<Bar>,
	config: Option<OnChainConfirmationConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&on_chain_data, "on_chain_data")?;
	validate_non_empty(&prices, "prices")?;
	Ok(strategies_core::on_chain_confirmation_strategy(
		on_chain_data,
		prices,
		config,
	))
}

#[napi]
pub fn value_momentum_pattern_strategy(
	fundamentals: Vec<FundamentalPoint>,
	prices: Vec<Bar>,
	config: Option<ValueMomentumPatternConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	validate_non_empty(&prices, "prices")?;
	Ok(strategies_core::value_momentum_pattern_strategy(
		fundamentals,
		prices,
		config,
	))
}

#[napi]
pub fn growth_quality_strategy(
	fundamentals: Vec<FundamentalPoint>,
	prices: Vec<Bar>,
	config: Option<GrowthQualityConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	validate_non_empty(&prices, "prices")?;
	Ok(strategies_core::growth_quality_strategy(
		fundamentals,
		prices,
		config,
	))
}

#[napi]
pub fn composite_value_momentum_strategy(
	fundamentals: Vec<FundamentalPoint>,
	prices: Vec<Bar>,
	config: Option<CompositeValueMomentumConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	validate_non_empty(&prices, "prices")?;
	Ok(strategies_core::composite_value_momentum_strategy(
		fundamentals,
		prices,
		config,
	))
}

#[napi]
pub fn quantamental_value_momentum_strategy(
	fundamentals: Vec<FundamentalPoint>,
	prices: Vec<Bar>,
	config: Option<QuantamentalValueMomentumConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	validate_non_empty(&prices, "prices")?;
	Ok(strategies_core::quantamental_value_momentum_strategy(
		fundamentals,
		prices,
		config,
	))
}

#[napi]
pub fn qarp_strategy_metadata() -> serde_json::Value {
	strategies_core::qarp_strategy_metadata()
}

#[napi]
pub fn multi_factor_value_strategy_metadata() -> serde_json::Value {
	strategies_core::multi_factor_value_strategy_metadata()
}

#[napi]
pub fn alternative_data_strategy_metadata() -> serde_json::Value {
	strategies_core::alternative_data_strategy_metadata()
}

#[napi]
pub fn event_driven_strategy_metadata() -> serde_json::Value {
	strategies_core::event_driven_strategy_metadata()
}

#[napi]
pub fn on_chain_confirmation_strategy_metadata() -> serde_json::Value {
	strategies_core::on_chain_confirmation_strategy_metadata()
}

#[napi]
pub fn value_momentum_pattern_strategy_metadata() -> serde_json::Value {
	strategies_core::value_momentum_pattern_strategy_metadata()
}

#[napi]
pub fn growth_quality_strategy_metadata() -> serde_json::Value {
	strategies_core::growth_quality_strategy_metadata()
}

#[napi]
pub fn composite_value_momentum_strategy_metadata() -> serde_json::Value {
	strategies_core::composite_value_momentum_strategy_metadata()
}

#[napi]
pub fn quantamental_value_momentum_strategy_metadata() -> serde_json::Value {
	strategies_core::quantamental_value_momentum_strategy_metadata()
}

#[napi]
pub fn qarp_strategy_defaults() -> serde_json::Value {
	strategies_core::qarp_strategy_defaults()
}

#[napi]
pub fn multi_factor_value_strategy_defaults() -> serde_json::Value {
	strategies_core::multi_factor_value_strategy_defaults()
}

#[napi]
pub fn alternative_data_strategy_defaults() -> serde_json::Value {
	strategies_core::alternative_data_strategy_defaults()
}

#[napi]
pub fn event_driven_strategy_defaults() -> serde_json::Value {
	strategies_core::event_driven_strategy_defaults()
}

#[napi]
pub fn on_chain_confirmation_strategy_defaults() -> serde_json::Value {
	strategies_core::on_chain_confirmation_strategy_defaults()
}

#[napi]
pub fn value_momentum_pattern_strategy_defaults() -> serde_json::Value {
	strategies_core::value_momentum_pattern_strategy_defaults()
}

#[napi]
pub fn growth_quality_strategy_defaults() -> serde_json::Value {
	strategies_core::growth_quality_strategy_defaults()
}

#[napi]
pub fn composite_value_momentum_strategy_defaults() -> serde_json::Value {
	strategies_core::composite_value_momentum_strategy_defaults()
}

#[napi]
pub fn quantamental_value_momentum_strategy_defaults() -> serde_json::Value {
	strategies_core::quantamental_value_momentum_strategy_defaults()
}
