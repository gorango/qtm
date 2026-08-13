use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::validation::validate_non_empty;
use factors_core::FactorPoint;
use factors_core::FundamentalPoint;
use strategies_core::EarningsCallRevenueConfig;
use strategies_core::EarningsReinvestmentRateConfig;
use strategies_core::GrowthVsCompetitionConfig;
use strategies_core::MgmtEarningsCallToneConfig;
use strategies_core::PegConfig;
use strategies_core::QoqRevenueMomentumConfig;
use strategies_core::RevenueGrowthAnalysisConfig;
use strategies_core::RevenueGrowthVsCompetitorsConfig;
use strategies_core::RevenueVolatilityScoreConfig;
use strategies_core::SeasonalityIndexRevenueConfig;
use strategies_core::SustainableGrowthRateConfig;
use strategies_core::TopQuartileConfig;

#[napi]
pub fn peg_strategy(factors: Vec<FactorPoint>, config: Option<PegConfig>) -> Result<Vec<i8>> {
	validate_non_empty(&factors, "factors")?;
	Ok(strategies_core::peg_strategy(factors, config))
}

#[napi]
pub fn earnings_growth_vs_competition_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<GrowthVsCompetitionConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::earnings_growth_vs_competition_strategy(
		points, config,
	))
}

#[napi]
pub fn revenue_growth_analysis_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<RevenueGrowthAnalysisConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::revenue_growth_analysis_strategy(
		points, config,
	))
}

#[napi]
pub fn sustainable_growth_rate_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<SustainableGrowthRateConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::sustainable_growth_rate_strategy(
		points, config,
	))
}

#[napi]
pub fn earnings_reinvestment_rate_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<EarningsReinvestmentRateConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::earnings_reinvestment_rate_strategy(
		points, config,
	))
}

#[napi]
pub fn top_quartile_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<TopQuartileConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::top_quartile_strategy(points, config))
}

#[napi]
pub fn qoq_revenue_momentum_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<QoqRevenueMomentumConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::qoq_revenue_momentum_strategy(
		points, config,
	))
}

#[napi]
pub fn revenue_growth_vs_competitors_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<RevenueGrowthVsCompetitorsConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::revenue_growth_vs_competitors_strategy(
		points, config,
	))
}

#[napi]
pub fn revenue_growth_vs_competition_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<GrowthVsCompetitionConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::revenue_growth_vs_competition_strategy(
		points, config,
	))
}

#[napi]
pub fn revenue_volatility_score_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<RevenueVolatilityScoreConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::revenue_volatility_score_strategy(
		points, config,
	))
}

#[napi]
pub fn seasonality_index_revenue_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<SeasonalityIndexRevenueConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::seasonality_index_revenue_strategy(
		points, config,
	))
}

#[napi]
pub fn management_earnings_call_tone_analysis_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<MgmtEarningsCallToneConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::management_earnings_call_tone_analysis_strategy(points, config))
}

#[napi]
pub fn earnings_call_revenue_analysis_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<EarningsCallRevenueConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::earnings_call_revenue_analysis_strategy(
		points, config,
	))
}

#[napi]
pub fn peg_strategy_metadata() -> serde_json::Value {
	strategies_core::peg_strategy_metadata()
}

#[napi]
pub fn peg_strategy_defaults() -> serde_json::Value {
	strategies_core::peg_strategy_defaults()
}

#[napi]
pub fn earnings_growth_vs_competition_strategy_metadata() -> serde_json::Value {
	strategies_core::earnings_growth_vs_competition_strategy_metadata()
}

#[napi]
pub fn earnings_growth_vs_competition_strategy_defaults() -> serde_json::Value {
	strategies_core::earnings_growth_vs_competition_strategy_defaults()
}

#[napi]
pub fn revenue_growth_analysis_strategy_metadata() -> serde_json::Value {
	strategies_core::revenue_growth_analysis_strategy_metadata()
}

#[napi]
pub fn revenue_growth_analysis_strategy_defaults() -> serde_json::Value {
	strategies_core::revenue_growth_analysis_strategy_defaults()
}

#[napi]
pub fn sustainable_growth_rate_strategy_metadata() -> serde_json::Value {
	strategies_core::sustainable_growth_rate_strategy_metadata()
}

#[napi]
pub fn sustainable_growth_rate_strategy_defaults() -> serde_json::Value {
	strategies_core::sustainable_growth_rate_strategy_defaults()
}

#[napi]
pub fn earnings_reinvestment_rate_strategy_metadata() -> serde_json::Value {
	strategies_core::earnings_reinvestment_rate_strategy_metadata()
}

#[napi]
pub fn earnings_reinvestment_rate_strategy_defaults() -> serde_json::Value {
	strategies_core::earnings_reinvestment_rate_strategy_defaults()
}

#[napi]
pub fn top_quartile_strategy_metadata() -> serde_json::Value {
	strategies_core::top_quartile_strategy_metadata()
}

#[napi]
pub fn top_quartile_strategy_defaults() -> serde_json::Value {
	strategies_core::top_quartile_strategy_defaults()
}

#[napi]
pub fn qoq_revenue_momentum_strategy_metadata() -> serde_json::Value {
	strategies_core::qoq_revenue_momentum_strategy_metadata()
}

#[napi]
pub fn qoq_revenue_momentum_strategy_defaults() -> serde_json::Value {
	strategies_core::qoq_revenue_momentum_strategy_defaults()
}

#[napi]
pub fn revenue_growth_vs_competitors_strategy_metadata() -> serde_json::Value {
	strategies_core::revenue_growth_vs_competitors_strategy_metadata()
}

#[napi]
pub fn revenue_growth_vs_competitors_strategy_defaults() -> serde_json::Value {
	strategies_core::revenue_growth_vs_competitors_strategy_defaults()
}

#[napi]
pub fn revenue_growth_vs_competition_strategy_metadata() -> serde_json::Value {
	strategies_core::revenue_growth_vs_competition_strategy_metadata()
}

#[napi]
pub fn revenue_growth_vs_competition_strategy_defaults() -> serde_json::Value {
	strategies_core::revenue_growth_vs_competition_strategy_defaults()
}

#[napi]
pub fn revenue_volatility_score_strategy_metadata() -> serde_json::Value {
	strategies_core::revenue_volatility_score_strategy_metadata()
}

#[napi]
pub fn revenue_volatility_score_strategy_defaults() -> serde_json::Value {
	strategies_core::revenue_volatility_score_strategy_defaults()
}

#[napi]
pub fn seasonality_index_revenue_strategy_metadata() -> serde_json::Value {
	strategies_core::seasonality_index_revenue_strategy_metadata()
}

#[napi]
pub fn seasonality_index_revenue_strategy_defaults() -> serde_json::Value {
	strategies_core::seasonality_index_revenue_strategy_defaults()
}

#[napi]
pub fn management_earnings_call_tone_analysis_strategy_metadata() -> serde_json::Value {
	strategies_core::management_earnings_call_tone_analysis_strategy_metadata()
}

#[napi]
pub fn management_earnings_call_tone_analysis_strategy_defaults() -> serde_json::Value {
	strategies_core::management_earnings_call_tone_analysis_strategy_defaults()
}

#[napi]
pub fn earnings_call_revenue_analysis_strategy_metadata() -> serde_json::Value {
	strategies_core::earnings_call_revenue_analysis_strategy_metadata()
}

#[napi]
pub fn earnings_call_revenue_analysis_strategy_defaults() -> serde_json::Value {
	strategies_core::earnings_call_revenue_analysis_strategy_defaults()
}
