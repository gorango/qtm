use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::validation::validate_non_empty;
use factors_core::FactorPoint;
use factors_core::FundamentalPoint;
use strategies_core::CapexDisciplineConfig;
use strategies_core::CccAnalysisConfig;
use strategies_core::CccCheckConfig;
use strategies_core::DupontRoeConfig;
use strategies_core::EarningsQualityConfig;
use strategies_core::EbitdaGrowthVsCompetitionConfig;
use strategies_core::EbitdaMarginConfig;
use strategies_core::EpsVsFcfDivergenceConfig;
use strategies_core::ExpenseSurpriseConfig;
use strategies_core::GrossProfitConfig;
use strategies_core::MarginChecklistConfig;
use strategies_core::MarginExpansionConfig;
use strategies_core::OcfAnalysisConfig;
use strategies_core::OperatingLeverageConfig;
use strategies_core::QualityChecklistConfig;
use strategies_core::QualityConfig;
use strategies_core::ReturnOnCapitalConfig;
use strategies_core::RevenueAssetsEfficiencyConfig;
use strategies_core::RevenueDiversificationConfig;
use strategies_core::RevenuePerEmployeeConfig;
use strategies_core::RndIntensityConfig;
use strategies_core::RoicDurabilityConfig;
use strategies_core::WorkingCapitalEfficiencyConfig;

#[napi]
pub fn quality_strategy(
	factors: Vec<FactorPoint>,
	config: Option<QualityConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&factors, "factors")?;
	Ok(strategies_core::quality_strategy(factors, config))
}

#[napi]
pub fn dupont_roe_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<DupontRoeConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::dupont_roe_strategy(points, config))
}

#[napi]
pub fn charlie_munger_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<QualityChecklistConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::charlie_munger_strategy(points, config))
}

#[napi]
pub fn philip_fisher_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<QualityChecklistConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::philip_fisher_strategy(points, config))
}

#[napi]
pub fn return_on_capital_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<ReturnOnCapitalConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::return_on_capital_strategy(points, config))
}

#[napi]
pub fn operating_margin_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<MarginChecklistConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::operating_margin_strategy(points, config))
}

#[napi]
pub fn earnings_quality_analysis_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<EarningsQualityConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::earnings_quality_analysis_strategy(
		points, config,
	))
}

#[napi]
pub fn capex_discipline_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<CapexDisciplineConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::capex_discipline_strategy(points, config))
}

#[napi]
pub fn ebitda_margin_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<EbitdaMarginConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::ebitda_margin_strategy(points, config))
}

#[napi]
pub fn gross_profit_analysis_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<GrossProfitConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::gross_profit_analysis_strategy(
		points, config,
	))
}

#[napi]
pub fn operating_cashflow_analysis_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<OcfAnalysisConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::operating_cashflow_analysis_strategy(
		points, config,
	))
}

#[napi]
pub fn operating_leverage_trend_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<OperatingLeverageConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::operating_leverage_trend_strategy(
		points, config,
	))
}

#[napi]
pub fn cash_conversion_cycle_check_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<CccCheckConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::cash_conversion_cycle_check_strategy(
		points, config,
	))
}

#[napi]
pub fn cash_conversion_cycle_analysis_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<CccAnalysisConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::cash_conversion_cycle_analysis_strategy(
		points, config,
	))
}

#[napi]
pub fn five_year_margin_expansion_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<MarginExpansionConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::five_year_margin_expansion_strategy(
		points, config,
	))
}

#[napi]
pub fn ebitda_growth_vs_competition_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<EbitdaGrowthVsCompetitionConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::ebitda_growth_vs_competition_strategy(
		points, config,
	))
}

#[napi]
pub fn eps_vs_fcf_divergence_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<EpsVsFcfDivergenceConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::eps_vs_fcf_divergence_strategy(
		points, config,
	))
}

#[napi]
pub fn expense_surprise_detector_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<ExpenseSurpriseConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::expense_surprise_detector_strategy(
		points, config,
	))
}

#[napi]
pub fn revenue_assets_efficiency_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<RevenueAssetsEfficiencyConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::revenue_assets_efficiency_strategy(
		points, config,
	))
}

#[napi]
pub fn revenue_diversification_proxy_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<RevenueDiversificationConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::revenue_diversification_proxy_strategy(
		points, config,
	))
}

#[napi]
pub fn revenue_per_employee_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<RevenuePerEmployeeConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::revenue_per_employee_strategy(
		points, config,
	))
}

#[napi]
pub fn rnd_intensity_tracker_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<RndIntensityConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::rnd_intensity_tracker_strategy(
		points, config,
	))
}

#[napi]
pub fn roic_durability_sweep_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<RoicDurabilityConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::roic_durability_sweep_strategy(
		points, config,
	))
}

#[napi]
pub fn working_capital_efficiency_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<WorkingCapitalEfficiencyConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&points, "points")?;
	Ok(strategies_core::working_capital_efficiency_strategy(
		points, config,
	))
}

#[napi]
pub fn quality_strategy_metadata() -> serde_json::Value {
	strategies_core::quality_strategy_metadata()
}

#[napi]
pub fn quality_strategy_defaults() -> serde_json::Value {
	strategies_core::quality_strategy_defaults()
}

#[napi]
pub fn dupont_roe_strategy_metadata() -> serde_json::Value {
	strategies_core::dupont_roe_strategy_metadata()
}

#[napi]
pub fn dupont_roe_strategy_defaults() -> serde_json::Value {
	strategies_core::dupont_roe_strategy_defaults()
}

#[napi]
pub fn charlie_munger_strategy_metadata() -> serde_json::Value {
	strategies_core::charlie_munger_strategy_metadata()
}

#[napi]
pub fn charlie_munger_strategy_defaults() -> serde_json::Value {
	strategies_core::charlie_munger_strategy_defaults()
}

#[napi]
pub fn philip_fisher_strategy_metadata() -> serde_json::Value {
	strategies_core::philip_fisher_strategy_metadata()
}

#[napi]
pub fn philip_fisher_strategy_defaults() -> serde_json::Value {
	strategies_core::philip_fisher_strategy_defaults()
}

#[napi]
pub fn return_on_capital_strategy_metadata() -> serde_json::Value {
	strategies_core::return_on_capital_strategy_metadata()
}

#[napi]
pub fn return_on_capital_strategy_defaults() -> serde_json::Value {
	strategies_core::return_on_capital_strategy_defaults()
}

#[napi]
pub fn operating_margin_strategy_metadata() -> serde_json::Value {
	strategies_core::operating_margin_strategy_metadata()
}

#[napi]
pub fn operating_margin_strategy_defaults() -> serde_json::Value {
	strategies_core::operating_margin_strategy_defaults()
}

#[napi]
pub fn earnings_quality_analysis_strategy_metadata() -> serde_json::Value {
	strategies_core::earnings_quality_analysis_strategy_metadata()
}

#[napi]
pub fn earnings_quality_analysis_strategy_defaults() -> serde_json::Value {
	strategies_core::earnings_quality_analysis_strategy_defaults()
}

#[napi]
pub fn capex_discipline_strategy_metadata() -> serde_json::Value {
	strategies_core::capex_discipline_strategy_metadata()
}

#[napi]
pub fn capex_discipline_strategy_defaults() -> serde_json::Value {
	strategies_core::capex_discipline_strategy_defaults()
}

#[napi]
pub fn ebitda_margin_strategy_metadata() -> serde_json::Value {
	strategies_core::ebitda_margin_strategy_metadata()
}

#[napi]
pub fn ebitda_margin_strategy_defaults() -> serde_json::Value {
	strategies_core::ebitda_margin_strategy_defaults()
}

#[napi]
pub fn gross_profit_analysis_strategy_metadata() -> serde_json::Value {
	strategies_core::gross_profit_analysis_strategy_metadata()
}

#[napi]
pub fn gross_profit_analysis_strategy_defaults() -> serde_json::Value {
	strategies_core::gross_profit_analysis_strategy_defaults()
}

#[napi]
pub fn operating_cashflow_analysis_strategy_metadata() -> serde_json::Value {
	strategies_core::operating_cashflow_analysis_strategy_metadata()
}

#[napi]
pub fn operating_cashflow_analysis_strategy_defaults() -> serde_json::Value {
	strategies_core::operating_cashflow_analysis_strategy_defaults()
}

#[napi]
pub fn operating_leverage_trend_strategy_metadata() -> serde_json::Value {
	strategies_core::operating_leverage_trend_strategy_metadata()
}

#[napi]
pub fn operating_leverage_trend_strategy_defaults() -> serde_json::Value {
	strategies_core::operating_leverage_trend_strategy_defaults()
}

#[napi]
pub fn cash_conversion_cycle_check_strategy_metadata() -> serde_json::Value {
	strategies_core::cash_conversion_cycle_check_strategy_metadata()
}

#[napi]
pub fn cash_conversion_cycle_check_strategy_defaults() -> serde_json::Value {
	strategies_core::cash_conversion_cycle_check_strategy_defaults()
}

#[napi]
pub fn cash_conversion_cycle_analysis_strategy_metadata() -> serde_json::Value {
	strategies_core::cash_conversion_cycle_analysis_strategy_metadata()
}

#[napi]
pub fn cash_conversion_cycle_analysis_strategy_defaults() -> serde_json::Value {
	strategies_core::cash_conversion_cycle_analysis_strategy_defaults()
}

#[napi]
pub fn five_year_margin_expansion_strategy_metadata() -> serde_json::Value {
	strategies_core::five_year_margin_expansion_strategy_metadata()
}

#[napi]
pub fn five_year_margin_expansion_strategy_defaults() -> serde_json::Value {
	strategies_core::five_year_margin_expansion_strategy_defaults()
}

#[napi]
pub fn ebitda_growth_vs_competition_strategy_metadata() -> serde_json::Value {
	strategies_core::ebitda_growth_vs_competition_strategy_metadata()
}

#[napi]
pub fn ebitda_growth_vs_competition_strategy_defaults() -> serde_json::Value {
	strategies_core::ebitda_growth_vs_competition_strategy_defaults()
}

#[napi]
pub fn eps_vs_fcf_divergence_strategy_metadata() -> serde_json::Value {
	strategies_core::eps_vs_fcf_divergence_strategy_metadata()
}

#[napi]
pub fn eps_vs_fcf_divergence_strategy_defaults() -> serde_json::Value {
	strategies_core::eps_vs_fcf_divergence_strategy_defaults()
}

#[napi]
pub fn expense_surprise_detector_strategy_metadata() -> serde_json::Value {
	strategies_core::expense_surprise_detector_strategy_metadata()
}

#[napi]
pub fn expense_surprise_detector_strategy_defaults() -> serde_json::Value {
	strategies_core::expense_surprise_detector_strategy_defaults()
}

#[napi]
pub fn revenue_assets_efficiency_strategy_metadata() -> serde_json::Value {
	strategies_core::revenue_assets_efficiency_strategy_metadata()
}

#[napi]
pub fn revenue_assets_efficiency_strategy_defaults() -> serde_json::Value {
	strategies_core::revenue_assets_efficiency_strategy_defaults()
}

#[napi]
pub fn revenue_diversification_proxy_strategy_metadata() -> serde_json::Value {
	strategies_core::revenue_diversification_proxy_strategy_metadata()
}

#[napi]
pub fn revenue_diversification_proxy_strategy_defaults() -> serde_json::Value {
	strategies_core::revenue_diversification_proxy_strategy_defaults()
}

#[napi]
pub fn revenue_per_employee_strategy_metadata() -> serde_json::Value {
	strategies_core::revenue_per_employee_strategy_metadata()
}

#[napi]
pub fn revenue_per_employee_strategy_defaults() -> serde_json::Value {
	strategies_core::revenue_per_employee_strategy_defaults()
}

#[napi]
pub fn rnd_intensity_tracker_strategy_metadata() -> serde_json::Value {
	strategies_core::rnd_intensity_tracker_strategy_metadata()
}

#[napi]
pub fn rnd_intensity_tracker_strategy_defaults() -> serde_json::Value {
	strategies_core::rnd_intensity_tracker_strategy_defaults()
}

#[napi]
pub fn roic_durability_sweep_strategy_metadata() -> serde_json::Value {
	strategies_core::roic_durability_sweep_strategy_metadata()
}

#[napi]
pub fn roic_durability_sweep_strategy_defaults() -> serde_json::Value {
	strategies_core::roic_durability_sweep_strategy_defaults()
}

#[napi]
pub fn working_capital_efficiency_strategy_metadata() -> serde_json::Value {
	strategies_core::working_capital_efficiency_strategy_metadata()
}

#[napi]
pub fn working_capital_efficiency_strategy_defaults() -> serde_json::Value {
	strategies_core::working_capital_efficiency_strategy_defaults()
}
