use napi_derive::napi;

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
pub fn peg_strategy(factors: Vec<FactorPoint>, config: Option<PegConfig>) -> Vec<i8> {
	strategies_core::peg_strategy(factors, config)
}

#[napi]
pub fn earnings_growth_vs_competition_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<GrowthVsCompetitionConfig>,
) -> Vec<i8> {
	strategies_core::earnings_growth_vs_competition_strategy(points, config)
}

#[napi]
pub fn revenue_growth_analysis_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<RevenueGrowthAnalysisConfig>,
) -> Vec<i8> {
	strategies_core::revenue_growth_analysis_strategy(points, config)
}

#[napi]
pub fn sustainable_growth_rate_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<SustainableGrowthRateConfig>,
) -> Vec<i8> {
	strategies_core::sustainable_growth_rate_strategy(points, config)
}

#[napi]
pub fn earnings_reinvestment_rate_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<EarningsReinvestmentRateConfig>,
) -> Vec<i8> {
	strategies_core::earnings_reinvestment_rate_strategy(points, config)
}

#[napi]
pub fn top_quartile_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<TopQuartileConfig>,
) -> Vec<i8> {
	strategies_core::top_quartile_strategy(points, config)
}

#[napi]
pub fn qoq_revenue_momentum_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<QoqRevenueMomentumConfig>,
) -> Vec<i8> {
	strategies_core::qoq_revenue_momentum_strategy(points, config)
}

#[napi]
pub fn revenue_growth_vs_competitors_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<RevenueGrowthVsCompetitorsConfig>,
) -> Vec<i8> {
	strategies_core::revenue_growth_vs_competitors_strategy(points, config)
}

#[napi]
pub fn revenue_growth_vs_competition_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<GrowthVsCompetitionConfig>,
) -> Vec<i8> {
	strategies_core::revenue_growth_vs_competition_strategy(points, config)
}

#[napi]
pub fn revenue_volatility_score_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<RevenueVolatilityScoreConfig>,
) -> Vec<i8> {
	strategies_core::revenue_volatility_score_strategy(points, config)
}

#[napi]
pub fn seasonality_index_revenue_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<SeasonalityIndexRevenueConfig>,
) -> Vec<i8> {
	strategies_core::seasonality_index_revenue_strategy(points, config)
}

#[napi]
pub fn management_earnings_call_tone_analysis_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<MgmtEarningsCallToneConfig>,
) -> Vec<i8> {
	strategies_core::management_earnings_call_tone_analysis_strategy(points, config)
}

#[napi]
pub fn earnings_call_revenue_analysis_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<EarningsCallRevenueConfig>,
) -> Vec<i8> {
	strategies_core::earnings_call_revenue_analysis_strategy(points, config)
}

pub fn peg_strategy_metadata() -> serde_json::Value {
	strategies_core::peg_strategy_metadata()
}

pub fn peg_strategy_defaults() -> serde_json::Value {
	strategies_core::peg_strategy_defaults()
}

pub fn earnings_growth_vs_competition_strategy_metadata() -> serde_json::Value {
	strategies_core::earnings_growth_vs_competition_strategy_metadata()
}

pub fn earnings_growth_vs_competition_strategy_defaults() -> serde_json::Value {
	strategies_core::earnings_growth_vs_competition_strategy_defaults()
}

pub fn revenue_growth_analysis_strategy_metadata() -> serde_json::Value {
	strategies_core::revenue_growth_analysis_strategy_metadata()
}

pub fn revenue_growth_analysis_strategy_defaults() -> serde_json::Value {
	strategies_core::revenue_growth_analysis_strategy_defaults()
}

pub fn sustainable_growth_rate_strategy_metadata() -> serde_json::Value {
	strategies_core::sustainable_growth_rate_strategy_metadata()
}

pub fn sustainable_growth_rate_strategy_defaults() -> serde_json::Value {
	strategies_core::sustainable_growth_rate_strategy_defaults()
}

pub fn earnings_reinvestment_rate_strategy_metadata() -> serde_json::Value {
	strategies_core::earnings_reinvestment_rate_strategy_metadata()
}

pub fn earnings_reinvestment_rate_strategy_defaults() -> serde_json::Value {
	strategies_core::earnings_reinvestment_rate_strategy_defaults()
}

pub fn top_quartile_strategy_metadata() -> serde_json::Value {
	strategies_core::top_quartile_strategy_metadata()
}

pub fn top_quartile_strategy_defaults() -> serde_json::Value {
	strategies_core::top_quartile_strategy_defaults()
}

pub fn qoq_revenue_momentum_strategy_metadata() -> serde_json::Value {
	strategies_core::qoq_revenue_momentum_strategy_metadata()
}

pub fn qoq_revenue_momentum_strategy_defaults() -> serde_json::Value {
	strategies_core::qoq_revenue_momentum_strategy_defaults()
}

pub fn revenue_growth_vs_competitors_strategy_metadata() -> serde_json::Value {
	strategies_core::revenue_growth_vs_competitors_strategy_metadata()
}

pub fn revenue_growth_vs_competitors_strategy_defaults() -> serde_json::Value {
	strategies_core::revenue_growth_vs_competitors_strategy_defaults()
}

pub fn revenue_growth_vs_competition_strategy_metadata() -> serde_json::Value {
	strategies_core::revenue_growth_vs_competition_strategy_metadata()
}

pub fn revenue_growth_vs_competition_strategy_defaults() -> serde_json::Value {
	strategies_core::revenue_growth_vs_competition_strategy_defaults()
}

pub fn revenue_volatility_score_strategy_metadata() -> serde_json::Value {
	strategies_core::revenue_volatility_score_strategy_metadata()
}

pub fn revenue_volatility_score_strategy_defaults() -> serde_json::Value {
	strategies_core::revenue_volatility_score_strategy_defaults()
}

pub fn seasonality_index_revenue_strategy_metadata() -> serde_json::Value {
	strategies_core::seasonality_index_revenue_strategy_metadata()
}

pub fn seasonality_index_revenue_strategy_defaults() -> serde_json::Value {
	strategies_core::seasonality_index_revenue_strategy_defaults()
}

pub fn management_earnings_call_tone_analysis_strategy_metadata() -> serde_json::Value {
	strategies_core::management_earnings_call_tone_analysis_strategy_metadata()
}

pub fn management_earnings_call_tone_analysis_strategy_defaults() -> serde_json::Value {
	strategies_core::management_earnings_call_tone_analysis_strategy_defaults()
}

pub fn earnings_call_revenue_analysis_strategy_metadata() -> serde_json::Value {
	strategies_core::earnings_call_revenue_analysis_strategy_metadata()
}

pub fn earnings_call_revenue_analysis_strategy_defaults() -> serde_json::Value {
	strategies_core::earnings_call_revenue_analysis_strategy_defaults()
}
