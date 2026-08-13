use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::validation::validate_non_empty;
use factors_core::Bar;
use factors_core::FactorPoint;
use factors_core::FundamentalPoint;
use strategies_core::CashBurnRunwayConfig;
use strategies_core::ClassicValueConfig;
use strategies_core::DebtEbitdarStressTestConfig;
use strategies_core::EvEbitdaFairValueConfig;
use strategies_core::EvFcf10yrBandConfig;
use strategies_core::EvRevenueMultiplesConfig;
use strategies_core::EvSalesFairValueConfig;
use strategies_core::FreeCashFlowAnalysisConfig;
use strategies_core::InterestCoverageBufferConfig;
use strategies_core::IntrinsicValueMultiMetricConfig;
use strategies_core::MarginOfSafetyTargetPriceConfig;
use strategies_core::NetCashPositionToggleConfig;
use strategies_core::NormalPeFutureFairValueConfig;
use strategies_core::OcfCoverageDividendsConfig;
use strategies_core::PriceSalesFairValueConfig;
use strategies_core::PriceToOwnerEarningsConfig;
use strategies_core::QuickRatioStressTestConfig;
use strategies_core::ReturnOfCapitalVsGrowthConfig;
use strategies_core::ValueChecklistConfig;
use strategies_core::ValueConfig;
use strategies_core::WaccVsRoicSpreadConfig;
use strategies_core::WorkingCapitalHealthConfig;

#[napi]
pub fn value_strategy(factors: Vec<FactorPoint>, config: Option<ValueConfig>) -> Result<Vec<i8>> {
	validate_non_empty(&factors, "factors")?;
	Ok(strategies_core::value_strategy(factors, config))
}

#[napi]
pub fn classic_value_strategy(
	factors: Vec<FactorPoint>,
	config: Option<ClassicValueConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&factors, "factors")?;
	Ok(strategies_core::classic_value_strategy(factors, config))
}

#[napi]
pub fn benjamin_graham_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<ValueChecklistConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	Ok(strategies_core::benjamin_graham_strategy(
		fundamentals,
		config,
	))
}

#[napi]
pub fn bill_miller_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<ValueChecklistConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	Ok(strategies_core::bill_miller_strategy(fundamentals, config))
}

#[napi]
pub fn john_templeton_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<ValueChecklistConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	Ok(strategies_core::john_templeton_strategy(
		fundamentals,
		config,
	))
}

#[napi]
pub fn walter_schloss_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<ValueChecklistConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	Ok(strategies_core::walter_schloss_strategy(
		fundamentals,
		config,
	))
}

#[napi]
pub fn free_cash_flow_analysis_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<FreeCashFlowAnalysisConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	Ok(strategies_core::free_cash_flow_analysis_strategy(
		fundamentals,
		config,
	))
}

#[napi]
pub fn wacc_vs_roic_spread_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<WaccVsRoicSpreadConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	Ok(strategies_core::wacc_vs_roic_spread_strategy(
		fundamentals,
		config,
	))
}

#[napi]
pub fn ev_ebitda_fair_value_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<EvEbitdaFairValueConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	Ok(strategies_core::ev_ebitda_fair_value_strategy(
		fundamentals,
		config,
	))
}

#[napi]
pub fn intrinsic_value_multi_metric_strategy(
	fundamentals: Vec<FundamentalPoint>,
	bars: Vec<Bar>,
	config: Option<IntrinsicValueMultiMetricConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	validate_non_empty(&bars, "bars")?;
	Ok(strategies_core::intrinsic_value_multi_metric_strategy(
		fundamentals,
		bars,
		config,
	))
}

#[napi]
pub fn cash_burn_runway_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<CashBurnRunwayConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	Ok(strategies_core::cash_burn_runway_strategy(
		fundamentals,
		config,
	))
}

#[napi]
pub fn debt_ebitdar_stress_test_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<DebtEbitdarStressTestConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	Ok(strategies_core::debt_ebitdar_stress_test_strategy(
		fundamentals,
		config,
	))
}

#[napi]
pub fn ev_fcf_10yr_band_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<EvFcf10yrBandConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	Ok(strategies_core::ev_fcf_10yr_band_strategy(
		fundamentals,
		config,
	))
}

#[napi]
pub fn ev_revenue_multiples_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<EvRevenueMultiplesConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	Ok(strategies_core::ev_revenue_multiples_strategy(
		fundamentals,
		config,
	))
}

#[napi]
pub fn ev_sales_fair_value_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<EvSalesFairValueConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	Ok(strategies_core::ev_sales_fair_value_strategy(
		fundamentals,
		config,
	))
}

#[napi]
pub fn interest_coverage_buffer_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<InterestCoverageBufferConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	Ok(strategies_core::interest_coverage_buffer_strategy(
		fundamentals,
		config,
	))
}

#[napi]
pub fn margin_of_safety_target_price_strategy(
	fundamentals: Vec<FundamentalPoint>,
	bars: Vec<Bar>,
	config: Option<MarginOfSafetyTargetPriceConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	validate_non_empty(&bars, "bars")?;
	Ok(strategies_core::margin_of_safety_target_price_strategy(
		fundamentals,
		bars,
		config,
	))
}

#[napi]
pub fn net_cash_position_toggle_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<NetCashPositionToggleConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	Ok(strategies_core::net_cash_position_toggle_strategy(
		fundamentals,
		config,
	))
}

#[napi]
pub fn normal_pe_future_fair_value_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<NormalPeFutureFairValueConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	Ok(strategies_core::normal_pe_future_fair_value_strategy(
		fundamentals,
		config,
	))
}

#[napi]
pub fn ocf_coverage_dividends_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<OcfCoverageDividendsConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	Ok(strategies_core::ocf_coverage_dividends_strategy(
		fundamentals,
		config,
	))
}

#[napi]
pub fn price_sales_fair_value_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<PriceSalesFairValueConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	Ok(strategies_core::price_sales_fair_value_strategy(
		fundamentals,
		config,
	))
}

#[napi]
pub fn price_to_owner_earnings_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<PriceToOwnerEarningsConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	Ok(strategies_core::price_to_owner_earnings_strategy(
		fundamentals,
		config,
	))
}

#[napi]
pub fn quick_ratio_stress_test_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<QuickRatioStressTestConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	Ok(strategies_core::quick_ratio_stress_test_strategy(
		fundamentals,
		config,
	))
}

#[napi]
pub fn return_of_capital_vs_growth_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<ReturnOfCapitalVsGrowthConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	Ok(strategies_core::return_of_capital_vs_growth_strategy(
		fundamentals,
		config,
	))
}

#[napi]
pub fn working_capital_health_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<WorkingCapitalHealthConfig>,
) -> Result<Vec<i8>> {
	validate_non_empty(&fundamentals, "fundamentals")?;
	Ok(strategies_core::working_capital_health_strategy(
		fundamentals,
		config,
	))
}

#[napi]
pub fn value_strategy_metadata() -> serde_json::Value {
	strategies_core::value_strategy_metadata()
}

#[napi]
pub fn value_strategy_defaults() -> serde_json::Value {
	strategies_core::value_strategy_defaults()
}

#[napi]
pub fn classic_value_strategy_metadata() -> serde_json::Value {
	strategies_core::classic_value_strategy_metadata()
}

#[napi]
pub fn classic_value_strategy_defaults() -> serde_json::Value {
	strategies_core::classic_value_strategy_defaults()
}

#[napi]
pub fn benjamin_graham_strategy_metadata() -> serde_json::Value {
	strategies_core::benjamin_graham_strategy_metadata()
}

#[napi]
pub fn benjamin_graham_strategy_defaults() -> serde_json::Value {
	strategies_core::benjamin_graham_strategy_defaults()
}

#[napi]
pub fn bill_miller_strategy_metadata() -> serde_json::Value {
	strategies_core::bill_miller_strategy_metadata()
}

#[napi]
pub fn bill_miller_strategy_defaults() -> serde_json::Value {
	strategies_core::bill_miller_strategy_defaults()
}

#[napi]
pub fn john_templeton_strategy_metadata() -> serde_json::Value {
	strategies_core::john_templeton_strategy_metadata()
}

#[napi]
pub fn john_templeton_strategy_defaults() -> serde_json::Value {
	strategies_core::john_templeton_strategy_defaults()
}

#[napi]
pub fn walter_schloss_strategy_metadata() -> serde_json::Value {
	strategies_core::walter_schloss_strategy_metadata()
}

#[napi]
pub fn walter_schloss_strategy_defaults() -> serde_json::Value {
	strategies_core::walter_schloss_strategy_defaults()
}

#[napi]
pub fn free_cash_flow_analysis_strategy_metadata() -> serde_json::Value {
	strategies_core::free_cash_flow_analysis_strategy_metadata()
}

#[napi]
pub fn free_cash_flow_analysis_strategy_defaults() -> serde_json::Value {
	strategies_core::free_cash_flow_analysis_strategy_defaults()
}

#[napi]
pub fn wacc_vs_roic_spread_strategy_metadata() -> serde_json::Value {
	strategies_core::wacc_vs_roic_spread_strategy_metadata()
}

#[napi]
pub fn wacc_vs_roic_spread_strategy_defaults() -> serde_json::Value {
	strategies_core::wacc_vs_roic_spread_strategy_defaults()
}

#[napi]
pub fn ev_ebitda_fair_value_strategy_metadata() -> serde_json::Value {
	strategies_core::ev_ebitda_fair_value_strategy_metadata()
}

#[napi]
pub fn ev_ebitda_fair_value_strategy_defaults() -> serde_json::Value {
	strategies_core::ev_ebitda_fair_value_strategy_defaults()
}

#[napi]
pub fn intrinsic_value_multi_metric_strategy_metadata() -> serde_json::Value {
	strategies_core::intrinsic_value_multi_metric_strategy_metadata()
}

#[napi]
pub fn intrinsic_value_multi_metric_strategy_defaults() -> serde_json::Value {
	strategies_core::intrinsic_value_multi_metric_strategy_defaults()
}

#[napi]
pub fn cash_burn_runway_strategy_metadata() -> serde_json::Value {
	strategies_core::cash_burn_runway_strategy_metadata()
}

#[napi]
pub fn cash_burn_runway_strategy_defaults() -> serde_json::Value {
	strategies_core::cash_burn_runway_strategy_defaults()
}

#[napi]
pub fn debt_ebitdar_stress_test_strategy_metadata() -> serde_json::Value {
	strategies_core::debt_ebitdar_stress_test_strategy_metadata()
}

#[napi]
pub fn debt_ebitdar_stress_test_strategy_defaults() -> serde_json::Value {
	strategies_core::debt_ebitdar_stress_test_strategy_defaults()
}

#[napi]
pub fn ev_fcf_10yr_band_strategy_metadata() -> serde_json::Value {
	strategies_core::ev_fcf_10yr_band_strategy_metadata()
}

#[napi]
pub fn ev_fcf_10yr_band_strategy_defaults() -> serde_json::Value {
	strategies_core::ev_fcf_10yr_band_strategy_defaults()
}

#[napi]
pub fn ev_revenue_multiples_strategy_metadata() -> serde_json::Value {
	strategies_core::ev_revenue_multiples_strategy_metadata()
}

#[napi]
pub fn ev_revenue_multiples_strategy_defaults() -> serde_json::Value {
	strategies_core::ev_revenue_multiples_strategy_defaults()
}

#[napi]
pub fn ev_sales_fair_value_strategy_metadata() -> serde_json::Value {
	strategies_core::ev_sales_fair_value_strategy_metadata()
}

#[napi]
pub fn ev_sales_fair_value_strategy_defaults() -> serde_json::Value {
	strategies_core::ev_sales_fair_value_strategy_defaults()
}

#[napi]
pub fn interest_coverage_buffer_strategy_metadata() -> serde_json::Value {
	strategies_core::interest_coverage_buffer_strategy_metadata()
}

#[napi]
pub fn interest_coverage_buffer_strategy_defaults() -> serde_json::Value {
	strategies_core::interest_coverage_buffer_strategy_defaults()
}

#[napi]
pub fn margin_of_safety_target_price_strategy_metadata() -> serde_json::Value {
	strategies_core::margin_of_safety_target_price_strategy_metadata()
}

#[napi]
pub fn margin_of_safety_target_price_strategy_defaults() -> serde_json::Value {
	strategies_core::margin_of_safety_target_price_strategy_defaults()
}

#[napi]
pub fn net_cash_position_toggle_strategy_metadata() -> serde_json::Value {
	strategies_core::net_cash_position_toggle_strategy_metadata()
}

#[napi]
pub fn net_cash_position_toggle_strategy_defaults() -> serde_json::Value {
	strategies_core::net_cash_position_toggle_strategy_defaults()
}

#[napi]
pub fn normal_pe_future_fair_value_strategy_metadata() -> serde_json::Value {
	strategies_core::normal_pe_future_fair_value_strategy_metadata()
}

#[napi]
pub fn normal_pe_future_fair_value_strategy_defaults() -> serde_json::Value {
	strategies_core::normal_pe_future_fair_value_strategy_defaults()
}

#[napi]
pub fn ocf_coverage_dividends_strategy_metadata() -> serde_json::Value {
	strategies_core::ocf_coverage_dividends_strategy_metadata()
}

#[napi]
pub fn ocf_coverage_dividends_strategy_defaults() -> serde_json::Value {
	strategies_core::ocf_coverage_dividends_strategy_defaults()
}

#[napi]
pub fn price_sales_fair_value_strategy_metadata() -> serde_json::Value {
	strategies_core::price_sales_fair_value_strategy_metadata()
}

#[napi]
pub fn price_sales_fair_value_strategy_defaults() -> serde_json::Value {
	strategies_core::price_sales_fair_value_strategy_defaults()
}

#[napi]
pub fn price_to_owner_earnings_strategy_metadata() -> serde_json::Value {
	strategies_core::price_to_owner_earnings_strategy_metadata()
}

#[napi]
pub fn price_to_owner_earnings_strategy_defaults() -> serde_json::Value {
	strategies_core::price_to_owner_earnings_strategy_defaults()
}

#[napi]
pub fn quick_ratio_stress_test_strategy_metadata() -> serde_json::Value {
	strategies_core::quick_ratio_stress_test_strategy_metadata()
}

#[napi]
pub fn quick_ratio_stress_test_strategy_defaults() -> serde_json::Value {
	strategies_core::quick_ratio_stress_test_strategy_defaults()
}

#[napi]
pub fn return_of_capital_vs_growth_strategy_metadata() -> serde_json::Value {
	strategies_core::return_of_capital_vs_growth_strategy_metadata()
}

#[napi]
pub fn return_of_capital_vs_growth_strategy_defaults() -> serde_json::Value {
	strategies_core::return_of_capital_vs_growth_strategy_defaults()
}

#[napi]
pub fn working_capital_health_strategy_metadata() -> serde_json::Value {
	strategies_core::working_capital_health_strategy_metadata()
}

#[napi]
pub fn working_capital_health_strategy_defaults() -> serde_json::Value {
	strategies_core::working_capital_health_strategy_defaults()
}
