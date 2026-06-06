use napi_derive::napi;

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
pub fn quality_strategy(factors: Vec<FactorPoint>, config: Option<QualityConfig>) -> Vec<i8> {
	strategies_core::quality_strategy(factors, config)
}

#[napi]
pub fn dupont_roe_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<DupontRoeConfig>,
) -> Vec<i8> {
	strategies_core::dupont_roe_strategy(points, config)
}

#[napi]
pub fn charlie_munger_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<QualityChecklistConfig>,
) -> Vec<i8> {
	strategies_core::charlie_munger_strategy(points, config)
}

#[napi]
pub fn philip_fisher_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<QualityChecklistConfig>,
) -> Vec<i8> {
	strategies_core::philip_fisher_strategy(points, config)
}

#[napi]
pub fn return_on_capital_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<ReturnOnCapitalConfig>,
) -> Vec<i8> {
	strategies_core::return_on_capital_strategy(points, config)
}

#[napi]
pub fn operating_margin_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<MarginChecklistConfig>,
) -> Vec<i8> {
	strategies_core::operating_margin_strategy(points, config)
}

#[napi]
pub fn earnings_quality_analysis_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<EarningsQualityConfig>,
) -> Vec<i8> {
	strategies_core::earnings_quality_analysis_strategy(points, config)
}

#[napi]
pub fn capex_discipline_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<CapexDisciplineConfig>,
) -> Vec<i8> {
	strategies_core::capex_discipline_strategy(points, config)
}

#[napi]
pub fn ebitda_margin_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<EbitdaMarginConfig>,
) -> Vec<i8> {
	strategies_core::ebitda_margin_strategy(points, config)
}

#[napi]
pub fn gross_profit_analysis_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<GrossProfitConfig>,
) -> Vec<i8> {
	strategies_core::gross_profit_analysis_strategy(points, config)
}

#[napi]
pub fn operating_cashflow_analysis_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<OcfAnalysisConfig>,
) -> Vec<i8> {
	strategies_core::operating_cashflow_analysis_strategy(points, config)
}

#[napi]
pub fn operating_leverage_trend_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<OperatingLeverageConfig>,
) -> Vec<i8> {
	strategies_core::operating_leverage_trend_strategy(points, config)
}

#[napi]
pub fn cash_conversion_cycle_check_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<CccCheckConfig>,
) -> Vec<i8> {
	strategies_core::cash_conversion_cycle_check_strategy(points, config)
}

#[napi]
pub fn cash_conversion_cycle_analysis_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<CccAnalysisConfig>,
) -> Vec<i8> {
	strategies_core::cash_conversion_cycle_analysis_strategy(points, config)
}

#[napi]
pub fn five_year_margin_expansion_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<MarginExpansionConfig>,
) -> Vec<i8> {
	strategies_core::five_year_margin_expansion_strategy(points, config)
}

#[napi]
pub fn ebitda_growth_vs_competition_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<EbitdaGrowthVsCompetitionConfig>,
) -> Vec<i8> {
	strategies_core::ebitda_growth_vs_competition_strategy(points, config)
}

#[napi]
pub fn eps_vs_fcf_divergence_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<EpsVsFcfDivergenceConfig>,
) -> Vec<i8> {
	strategies_core::eps_vs_fcf_divergence_strategy(points, config)
}

#[napi]
pub fn expense_surprise_detector_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<ExpenseSurpriseConfig>,
) -> Vec<i8> {
	strategies_core::expense_surprise_detector_strategy(points, config)
}

#[napi]
pub fn revenue_assets_efficiency_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<RevenueAssetsEfficiencyConfig>,
) -> Vec<i8> {
	strategies_core::revenue_assets_efficiency_strategy(points, config)
}

#[napi]
pub fn revenue_diversification_proxy_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<RevenueDiversificationConfig>,
) -> Vec<i8> {
	strategies_core::revenue_diversification_proxy_strategy(points, config)
}

#[napi]
pub fn revenue_per_employee_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<RevenuePerEmployeeConfig>,
) -> Vec<i8> {
	strategies_core::revenue_per_employee_strategy(points, config)
}

#[napi]
pub fn rnd_intensity_tracker_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<RndIntensityConfig>,
) -> Vec<i8> {
	strategies_core::rnd_intensity_tracker_strategy(points, config)
}

#[napi]
pub fn roic_durability_sweep_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<RoicDurabilityConfig>,
) -> Vec<i8> {
	strategies_core::roic_durability_sweep_strategy(points, config)
}

#[napi]
pub fn working_capital_efficiency_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<WorkingCapitalEfficiencyConfig>,
) -> Vec<i8> {
	strategies_core::working_capital_efficiency_strategy(points, config)
}

pub fn quality_strategy_metadata() -> serde_json::Value {
	strategies_core::quality_strategy_metadata()
}

pub fn quality_strategy_defaults() -> serde_json::Value {
	strategies_core::quality_strategy_defaults()
}

pub fn dupont_roe_strategy_metadata() -> serde_json::Value {
	strategies_core::dupont_roe_strategy_metadata()
}

pub fn dupont_roe_strategy_defaults() -> serde_json::Value {
	strategies_core::dupont_roe_strategy_defaults()
}

pub fn charlie_munger_strategy_metadata() -> serde_json::Value {
	strategies_core::charlie_munger_strategy_metadata()
}

pub fn charlie_munger_strategy_defaults() -> serde_json::Value {
	strategies_core::charlie_munger_strategy_defaults()
}

pub fn philip_fisher_strategy_metadata() -> serde_json::Value {
	strategies_core::philip_fisher_strategy_metadata()
}

pub fn philip_fisher_strategy_defaults() -> serde_json::Value {
	strategies_core::philip_fisher_strategy_defaults()
}

pub fn return_on_capital_strategy_metadata() -> serde_json::Value {
	strategies_core::return_on_capital_strategy_metadata()
}

pub fn return_on_capital_strategy_defaults() -> serde_json::Value {
	strategies_core::return_on_capital_strategy_defaults()
}

pub fn operating_margin_strategy_metadata() -> serde_json::Value {
	strategies_core::operating_margin_strategy_metadata()
}

pub fn operating_margin_strategy_defaults() -> serde_json::Value {
	strategies_core::operating_margin_strategy_defaults()
}

pub fn earnings_quality_analysis_strategy_metadata() -> serde_json::Value {
	strategies_core::earnings_quality_analysis_strategy_metadata()
}

pub fn earnings_quality_analysis_strategy_defaults() -> serde_json::Value {
	strategies_core::earnings_quality_analysis_strategy_defaults()
}

pub fn capex_discipline_strategy_metadata() -> serde_json::Value {
	strategies_core::capex_discipline_strategy_metadata()
}

pub fn capex_discipline_strategy_defaults() -> serde_json::Value {
	strategies_core::capex_discipline_strategy_defaults()
}

pub fn ebitda_margin_strategy_metadata() -> serde_json::Value {
	strategies_core::ebitda_margin_strategy_metadata()
}

pub fn ebitda_margin_strategy_defaults() -> serde_json::Value {
	strategies_core::ebitda_margin_strategy_defaults()
}

pub fn gross_profit_analysis_strategy_metadata() -> serde_json::Value {
	strategies_core::gross_profit_analysis_strategy_metadata()
}

pub fn gross_profit_analysis_strategy_defaults() -> serde_json::Value {
	strategies_core::gross_profit_analysis_strategy_defaults()
}

pub fn operating_cashflow_analysis_strategy_metadata() -> serde_json::Value {
	strategies_core::operating_cashflow_analysis_strategy_metadata()
}

pub fn operating_cashflow_analysis_strategy_defaults() -> serde_json::Value {
	strategies_core::operating_cashflow_analysis_strategy_defaults()
}

pub fn operating_leverage_trend_strategy_metadata() -> serde_json::Value {
	strategies_core::operating_leverage_trend_strategy_metadata()
}

pub fn operating_leverage_trend_strategy_defaults() -> serde_json::Value {
	strategies_core::operating_leverage_trend_strategy_defaults()
}

pub fn cash_conversion_cycle_check_strategy_metadata() -> serde_json::Value {
	strategies_core::cash_conversion_cycle_check_strategy_metadata()
}

pub fn cash_conversion_cycle_check_strategy_defaults() -> serde_json::Value {
	strategies_core::cash_conversion_cycle_check_strategy_defaults()
}

pub fn cash_conversion_cycle_analysis_strategy_metadata() -> serde_json::Value {
	strategies_core::cash_conversion_cycle_analysis_strategy_metadata()
}

pub fn cash_conversion_cycle_analysis_strategy_defaults() -> serde_json::Value {
	strategies_core::cash_conversion_cycle_analysis_strategy_defaults()
}

pub fn five_year_margin_expansion_strategy_metadata() -> serde_json::Value {
	strategies_core::five_year_margin_expansion_strategy_metadata()
}

pub fn five_year_margin_expansion_strategy_defaults() -> serde_json::Value {
	strategies_core::five_year_margin_expansion_strategy_defaults()
}

pub fn ebitda_growth_vs_competition_strategy_metadata() -> serde_json::Value {
	strategies_core::ebitda_growth_vs_competition_strategy_metadata()
}

pub fn ebitda_growth_vs_competition_strategy_defaults() -> serde_json::Value {
	strategies_core::ebitda_growth_vs_competition_strategy_defaults()
}

pub fn eps_vs_fcf_divergence_strategy_metadata() -> serde_json::Value {
	strategies_core::eps_vs_fcf_divergence_strategy_metadata()
}

pub fn eps_vs_fcf_divergence_strategy_defaults() -> serde_json::Value {
	strategies_core::eps_vs_fcf_divergence_strategy_defaults()
}

pub fn expense_surprise_detector_strategy_metadata() -> serde_json::Value {
	strategies_core::expense_surprise_detector_strategy_metadata()
}

pub fn expense_surprise_detector_strategy_defaults() -> serde_json::Value {
	strategies_core::expense_surprise_detector_strategy_defaults()
}

pub fn revenue_assets_efficiency_strategy_metadata() -> serde_json::Value {
	strategies_core::revenue_assets_efficiency_strategy_metadata()
}

pub fn revenue_assets_efficiency_strategy_defaults() -> serde_json::Value {
	strategies_core::revenue_assets_efficiency_strategy_defaults()
}

pub fn revenue_diversification_proxy_strategy_metadata() -> serde_json::Value {
	strategies_core::revenue_diversification_proxy_strategy_metadata()
}

pub fn revenue_diversification_proxy_strategy_defaults() -> serde_json::Value {
	strategies_core::revenue_diversification_proxy_strategy_defaults()
}

pub fn revenue_per_employee_strategy_metadata() -> serde_json::Value {
	strategies_core::revenue_per_employee_strategy_metadata()
}

pub fn revenue_per_employee_strategy_defaults() -> serde_json::Value {
	strategies_core::revenue_per_employee_strategy_defaults()
}

pub fn rnd_intensity_tracker_strategy_metadata() -> serde_json::Value {
	strategies_core::rnd_intensity_tracker_strategy_metadata()
}

pub fn rnd_intensity_tracker_strategy_defaults() -> serde_json::Value {
	strategies_core::rnd_intensity_tracker_strategy_defaults()
}

pub fn roic_durability_sweep_strategy_metadata() -> serde_json::Value {
	strategies_core::roic_durability_sweep_strategy_metadata()
}

pub fn roic_durability_sweep_strategy_defaults() -> serde_json::Value {
	strategies_core::roic_durability_sweep_strategy_defaults()
}

pub fn working_capital_efficiency_strategy_metadata() -> serde_json::Value {
	strategies_core::working_capital_efficiency_strategy_metadata()
}

pub fn working_capital_efficiency_strategy_defaults() -> serde_json::Value {
	strategies_core::working_capital_efficiency_strategy_defaults()
}
