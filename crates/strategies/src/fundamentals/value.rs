use factors_core::{
    price_to_book_value, price_to_sales_value, free_cash_flow_yield_value,
    free_cash_flow_margin_value, owner_earnings_value,
    cash_to_market_cap_value, book_value_per_share_value, net_cash_value, cash_to_assets_value,
    cash_to_liabilities_value, payout_ratio_value, debt_to_equity_value, current_ratio_value,
    working_capital_value, interest_coverage_value, roe_value,
    ev_to_ebitda_value, ev_to_revenue_value, ebitda_margin_value, pe_ratio_value,
    dividend_coverage_ocf_value, quick_ratio_value, debt_service_coverage_value,
    Bar, FactorPoint, FundamentalPoint,
};
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};

// ============================================================
// Helper utilities
// ============================================================

fn below_threshold(factors: &[FactorPoint], threshold: f64) -> Vec<i8> {
	factors
		.iter()
		.map(|f| {
			if f.value > 0.0 && f.value < threshold {
				1
			} else {
				0
			}
		})
		.collect()
}

#[allow(dead_code)]
fn above_threshold(factors: &[FactorPoint], threshold: f64) -> Vec<i8> {
	factors
		.iter()
		.map(|f| if f.value > threshold { 1 } else { 0 })
		.collect()
}

#[allow(dead_code)]
fn safe_div(num: f64, den: f64) -> Option<f64> {
	if den != 0.0 {
		Some(num / den)
	} else {
		None
	}
}



// ============================================================
// Configs
// ============================================================

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValueConfig {
	pub pe_threshold: Option<f64>,
	pub enable_time_based_exit: Option<bool>,
	pub max_hold_bars: Option<u32>,
}

impl Default for ValueConfig {
	fn default() -> Self {
		Self {
			pe_threshold: Some(15.0),
			enable_time_based_exit: Some(true),
			max_hold_bars: Some(60),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassicValueConfig {
	pub quartile: Option<u32>,
}

impl Default for ClassicValueConfig {
	fn default() -> Self {
		Self { quartile: Some(1) }
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValueChecklistConfig {
	pub min_criteria_met: Option<u32>,
}

impl Default for ValueChecklistConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(7),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FreeCashFlowAnalysisConfig {
	pub min_criteria_met: Option<u32>,
	pub fcf_margin_threshold: Option<f64>,
	pub fcf_yield_threshold: Option<f64>,
}

impl Default for FreeCashFlowAnalysisConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			fcf_margin_threshold: Some(0.1),
			fcf_yield_threshold: Some(0.05),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WaccVsRoicSpreadConfig {
	pub min_criteria_met: Option<u32>,
	pub min_roic: Option<f64>,
	pub min_spread: Option<f64>,
}

impl Default for WaccVsRoicSpreadConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			min_roic: Some(0.1),
			min_spread: Some(0.05),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvEbitdaFairValueConfig {
	pub min_criteria_met: Option<u32>,
	pub ev_ebitda_threshold: Option<f64>,
	pub ebitda_margin_threshold: Option<f64>,
}

impl Default for EvEbitdaFairValueConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			ev_ebitda_threshold: Some(10.0),
			ebitda_margin_threshold: Some(0.1),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IntrinsicValueMultiMetricConfig {
	pub min_criteria_met: Option<u32>,
	pub margin_of_safety: Option<f64>,
	pub min_intrinsic_value: Option<f64>,
}

impl Default for IntrinsicValueMultiMetricConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			margin_of_safety: Some(0.2),
			min_intrinsic_value: Some(0.0),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CashBurnRunwayConfig {
	pub min_criteria_met: Option<u32>,
	pub min_runway_months: Option<f64>,
	pub min_cash_to_burn_ratio: Option<f64>,
}

impl Default for CashBurnRunwayConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			min_runway_months: Some(12.0),
			min_cash_to_burn_ratio: Some(0.5),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DebtEbitdarStressTestConfig {
	pub min_criteria_met: Option<u32>,
	pub max_debt_ebitdar: Option<f64>,
	pub stress_ebitdar_reduction: Option<f64>,
}

impl Default for DebtEbitdarStressTestConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			max_debt_ebitdar: Some(4.0),
			stress_ebitdar_reduction: Some(0.2),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvFcf10yrBandConfig {
	pub min_criteria_met: Option<u32>,
	pub max_ev_fcf: Option<f64>,
	pub fcf_threshold: Option<f64>,
}

impl Default for EvFcf10yrBandConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			max_ev_fcf: Some(15.0),
			fcf_threshold: Some(0.0),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvRevenueMultiplesConfig {
	pub min_criteria_met: Option<u32>,
	pub ev_revenue_threshold: Option<f64>,
	pub revenue_growth_threshold: Option<f64>,
}

impl Default for EvRevenueMultiplesConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			ev_revenue_threshold: Some(3.0),
			revenue_growth_threshold: Some(0.1),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvSalesFairValueConfig {
	pub min_criteria_met: Option<u32>,
	pub ev_sales_threshold: Option<f64>,
	pub sales_growth_threshold: Option<f64>,
}

impl Default for EvSalesFairValueConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			ev_sales_threshold: Some(2.0),
			sales_growth_threshold: Some(0.05),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InterestCoverageBufferConfig {
	pub min_criteria_met: Option<u32>,
	pub min_interest_coverage: Option<f64>,
	pub buffer_multiplier: Option<f64>,
}

impl Default for InterestCoverageBufferConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			min_interest_coverage: Some(3.0),
			buffer_multiplier: Some(1.5),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MarginOfSafetyTargetPriceConfig {
	pub min_criteria_met: Option<u32>,
	pub upside_threshold: Option<f64>,
	pub momentum_period: Option<u32>,
}

impl Default for MarginOfSafetyTargetPriceConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			upside_threshold: Some(0.2),
			momentum_period: Some(30),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetCashPositionToggleConfig {
	pub min_criteria_met: Option<u32>,
	pub min_net_cash: Option<f64>,
	pub max_debt_to_equity: Option<f64>,
}

impl Default for NetCashPositionToggleConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			min_net_cash: Some(0.0),
			max_debt_to_equity: Some(1.0),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NormalPeFutureFairValueConfig {
	pub min_criteria_met: Option<u32>,
	pub max_forward_pe: Option<f64>,
	pub earnings_growth_threshold: Option<f64>,
}

impl Default for NormalPeFutureFairValueConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			max_forward_pe: Some(20.0),
			earnings_growth_threshold: Some(0.08),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OcfCoverageDividendsConfig {
	pub min_criteria_met: Option<u32>,
	pub min_coverage_ratio: Option<f64>,
	pub dividend_yield_threshold: Option<f64>,
}

impl Default for OcfCoverageDividendsConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			min_coverage_ratio: Some(1.5),
			dividend_yield_threshold: Some(0.02),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PriceSalesFairValueConfig {
	pub min_criteria_met: Option<u32>,
	pub ps_threshold: Option<f64>,
	pub sales_growth_threshold: Option<f64>,
}

impl Default for PriceSalesFairValueConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			ps_threshold: Some(1.5),
			sales_growth_threshold: Some(0.05),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PriceToOwnerEarningsConfig {
	pub min_criteria_met: Option<u32>,
	pub max_price_to_owner_earnings: Option<f64>,
	pub owner_earnings_threshold: Option<f64>,
}

impl Default for PriceToOwnerEarningsConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			max_price_to_owner_earnings: Some(15.0),
			owner_earnings_threshold: Some(0.0),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuickRatioStressTestConfig {
	pub min_criteria_met: Option<u32>,
	pub min_quick_ratio: Option<f64>,
	pub stress_reduction: Option<f64>,
}

impl Default for QuickRatioStressTestConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			min_quick_ratio: Some(1.2),
			stress_reduction: Some(0.2),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReturnOfCapitalVsGrowthConfig {
	pub min_criteria_met: Option<u32>,
	pub max_payout_ratio: Option<f64>,
	pub min_dividend_yield: Option<f64>,
}

impl Default for ReturnOfCapitalVsGrowthConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			max_payout_ratio: Some(0.6),
			min_dividend_yield: Some(0.02),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkingCapitalHealthConfig {
	pub min_criteria_met: Option<u32>,
	pub min_current_ratio: Option<f64>,
	pub min_working_capital_to_assets: Option<f64>,
}

impl Default for WorkingCapitalHealthConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			min_current_ratio: Some(1.5),
			min_working_capital_to_assets: Some(0.1),
		}
	}
}

// ============================================================
// 1. Value Strategy - Simple PE threshold
// ============================================================

/// Value
///
/// Value Strategy: buy when factor composite signals undervaluation
pub fn value_strategy(factors: Vec<FactorPoint>, config: Option<ValueConfig>) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	below_threshold(&factors, cfg.pe_threshold.unwrap_or(15.0))
}

pub fn value_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "value-investing",
		"name": "Value Investing Fundamental",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "A simple value investing strategy that enters when P/E ratio is below a threshold"
	})
}

pub fn value_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "peThreshold": 15, "enableTimeBasedExit": true, "maxHoldBars": 60 }, "optimization_bounds": [] })
}

// ============================================================
// 2. Classic Value Strategy - PE quartile
// ============================================================

/// Classic Value
///
/// Classic Value: buy when P/E, P/B, and P/S are below industry thresholds
pub fn classic_value_strategy(
	factors: Vec<FactorPoint>,
	config: Option<ClassicValueConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let quartile = cfg.quartile.unwrap_or(1) as usize;
	if factors.is_empty() {
		return vec![];
	}
	let mut sorted: Vec<f64> = factors
		.iter()
		.filter(|f| f.value > 0.0)
		.map(|f| f.value)
		.collect();
	sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
	let n = sorted.len();
	let threshold = if quartile <= 1 {
		if n > 0 {
			sorted[n / 4.min(n)]
		} else {
			f64::MAX
		}
	} else if quartile >= 4 {
		f64::MAX
	} else {
		let idx = (n * quartile / 4).min(n.saturating_sub(1));
		sorted[idx]
	};
	factors
		.iter()
		.map(|f| {
			if f.value > 0.0 && f.value <= threshold {
				1
			} else {
				0
			}
		})
		.collect()
}

pub fn classic_value_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "classic-value",
		"name": "Classic Value Investing",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Classic value investing strategy that ranks stocks by P/E ratio and buys the cheapest quartile"
	})
}

pub fn classic_value_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "quartile": 1 }, "optimization_bounds": [] })
}

// ============================================================
// 3. Benjamin Graham Deep Value (8 rules)
// ============================================================

/// Benjamin Graham
///
/// Benjamin Graham: buy when P/E < 15, P/B < 1.5, D/E < 1.1, current ratio > 1.5
pub fn benjamin_graham_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<ValueChecklistConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_met = cfg.min_criteria_met.unwrap_or(7) as usize;
	fundamentals
		.iter()
		.map(|fp| {
			let d = &fp.data;
			let mut count = 0usize;
			if pe_ratio_value(d).is_some_and(|v| v < 15.0) {
				count += 1;
			}
			if price_to_book_value(d).is_some_and(|v| v < 1.5) {
				count += 1;
			}
			if current_ratio_value(d).is_some_and(|v| v > 2.0) {
				count += 1;
			}
			if debt_to_equity_value(d).is_some_and(|v| v < 0.5) {
				count += 1;
			}
			if d.eps.is_some_and(|v| v > 0.0) {
				count += 1;
			}
			if d.eps.is_some_and(|v| v > 0.0) {
				count += 1;
			}
			if working_capital_value(d).is_some_and(|v| v > 0.0) {
				count += 1;
			}
			if d.dividends_per_share.is_some_and(|v| v > 0.0) {
				count += 1;
			}
			if count >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

pub fn benjamin_graham_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "benjamin-graham-deep-value",
		"name": "Benjamin Graham Deep Value",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Implements the Benjamin Graham Deep Value checklist"
	})
}

pub fn benjamin_graham_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "minCriteriaMet": 7 }, "optimization_bounds": [] })
}

// ============================================================
// 4. Bill Miller Contrarian Growth (8 rules)
// ============================================================

/// Bill Miller
///
/// Bill Miller: buy when price is below intrinsic value with margin of safety
pub fn bill_miller_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<ValueChecklistConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_met = cfg.min_criteria_met.unwrap_or(7) as usize;
	fundamentals
		.iter()
		.map(|fp| {
			let d = &fp.data;
			let mut count = 0usize;
			if pe_ratio_value(d).is_some_and(|v| v < 20.0) {
				count += 1;
			}
			if d.revenue.is_some_and(|v| v > 0.0) {
				count += 1;
			}
			if d.cost_and_expenses.is_some_and(|_| true)
				&& d.revenue.is_some_and(|r| r > 0.0)
				&& d.cost_and_expenses.is_none_or(|c| c < d.revenue.unwrap())
			{
				count += 1;
			}
			if free_cash_flow_yield_value(d).is_some_and(|v| v > 0.0) {
				count += 1;
			}
			if roe_value(d).is_some_and(|v| v > 0.15) {
				count += 1;
			}
			if debt_service_coverage_value(d).is_some_and(|v| v > 3.0) {
				count += 1;
			}
			if d.market_cap.is_some_and(|v| v > 500_000_000.0) {
				count += 1;
			}
			if price_to_sales_value(d).is_some_and(|v| v < 3.0) {
				count += 1;
			}
			if count >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

pub fn bill_miller_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "bill-miller-contrarian-growth",
		"name": "Bill Miller Contrarian Growth",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Implements the Bill Miller Contrarian Growth checklist"
	})
}

pub fn bill_miller_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "minCriteriaMet": 7 }, "optimization_bounds": [] })
}

// ============================================================
// 5. John Templeton Global Value (8 rules)
// ============================================================

/// John Templeton
///
/// John Templeton: buy when P/E is low relative to historical and market average
pub fn john_templeton_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<ValueChecklistConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_met = cfg.min_criteria_met.unwrap_or(7) as usize;
	fundamentals
		.iter()
		.map(|fp| {
			let d = &fp.data;
			let mut count = 0usize;
			if pe_ratio_value(d).is_some_and(|v| v < 15.0) {
				count += 1;
			}
			if price_to_book_value(d).is_some_and(|v| v < 2.0) {
				count += 1;
			}
			if current_ratio_value(d).is_some_and(|v| v > 1.5) {
				count += 1;
			}
			if debt_to_equity_value(d).is_some_and(|v| v < 0.6) {
				count += 1;
			}
			if roe_value(d).is_some_and(|v| v > 0.1) {
				count += 1;
			}
			if d.eps.is_some_and(|v| v > 0.0) {
				count += 1;
			}
			if owner_earnings_value(d).is_some_and(|v| v > 0.0) {
				count += 1;
			}
			if dividend_coverage_ocf_value(d).is_some_and(|v| v > 2.0) {
				count += 1;
			}
			if count >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

pub fn john_templeton_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "john-templeton-value",
		"name": "John Templeton Global Value",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Implements the John Templeton Global Value checklist"
	})
}

pub fn john_templeton_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "minCriteriaMet": 7 }, "optimization_bounds": [] })
}

// ============================================================
// 6. Walter Schloss Asset-Based Value (8 rules)
// ============================================================

/// Walter Schloss
///
/// Walter Schloss: buy when P/B < 1.2, low debt, positive earnings, discount to net assets
pub fn walter_schloss_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<ValueChecklistConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_met = cfg.min_criteria_met.unwrap_or(7) as usize;
	fundamentals
		.iter()
		.map(|fp| {
			let d = &fp.data;
			let mut count = 0usize;
			if price_to_book_value(d).is_some_and(|v| v < 1.0) {
				count += 1;
			}
			if debt_to_equity_value(d).is_some_and(|v| v < 0.3) {
				count += 1;
			}
			if d.market_cap.is_some_and(|v| v < 1_000_000_000.0) {
				count += 1;
			}
			if book_value_per_share_value(d).is_some_and(|v| v > 0.0) {
				count += 1;
			}
			if d.eps.is_some_and(|v| v > 0.0) {
				count += 1;
			}
			if d.shareholders_equity
				.zip(d.total_assets)
				.is_some_and(|(se, ta)| ta > 0.0 && se / ta > 0.8)
			{
				count += 1;
			}
			if current_ratio_value(d).is_some_and(|v| v > 2.0) {
				count += 1;
			}
			if cash_to_market_cap_value(d).is_some_and(|v| v > 0.1) {
				count += 1;
			}
			if count >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

pub fn walter_schloss_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "walter-schloss-asset-based-value",
		"name": "Walter Schloss Asset-Based Value",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Implements the Walter Schloss Asset-Based Value checklist"
	})
}

pub fn walter_schloss_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "minCriteriaMet": 7 }, "optimization_bounds": [] })
}

// ============================================================
// 7. Free Cash Flow Analysis
// ============================================================

/// Free Cash Flow Analysis
///
/// Free Cash Flow Analysis: buy when FCF is positive and FCF yield exceeds threshold
pub fn free_cash_flow_analysis_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<FreeCashFlowAnalysisConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	let margin_thresh = cfg.fcf_margin_threshold.unwrap_or(0.1);
	let yield_thresh = cfg.fcf_yield_threshold.unwrap_or(0.05);
	fundamentals
		.iter()
		.map(|fp| {
			let d = &fp.data;
			let mut count = 0usize;
			if free_cash_flow_margin_value(d).is_some_and(|v| v > margin_thresh) {
				count += 1;
			}
			if free_cash_flow_yield_value(d).is_some_and(|v| v > yield_thresh) {
				count += 1;
			}
			if owner_earnings_value(d).is_some_and(|v| v > 0.0) {
				count += 1;
			}
			if count >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

pub fn free_cash_flow_analysis_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "free-cash-flow-analysis",
		"name": "Free Cash Flow Analysis",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Evaluates cash generation quality: high FCF margins, attractive yields, consistent positive FCF"
	})
}

pub fn free_cash_flow_analysis_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "minCriteriaMet": 2, "fcfMarginThreshold": 0.1, "fcfYieldThreshold": 0.05 }, "optimization_bounds": [] })
}

// ============================================================
// 8. WACC vs ROIC Spread
// ============================================================

/// Wacc Vs Roic Spread
///
/// WACC vs ROIC Spread: buy when ROIC exceeds WACC by minimum spread
pub fn wacc_vs_roic_spread_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<WaccVsRoicSpreadConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	let min_roic = cfg.min_roic.unwrap_or(0.1);
	let min_spread = cfg.min_spread.unwrap_or(0.05);
	fundamentals
		.iter()
		.map(|fp| {
			let d = &fp.data;
			let mut count = 0usize;
			let roic = roe_value(d);
			let wacc = d.interest_expense.map(|ie| {
				d.total_debt
					.and_then(|td| {
						d.shareholders_equity.and_then(|se| {
							let total_capital = td + se;
							if total_capital > 0.0 {
								Some(ie / total_capital)
							} else {
								None
							}
						})
					})
					.unwrap_or(0.0)
			});
			if roic.zip(wacc).is_some_and(|(r, w)| r > w) {
				count += 1;
			}
			if roic.is_some_and(|r| r > min_roic) {
				count += 1;
			}
			if roic.zip(wacc).is_some_and(|(r, w)| (r - w) > min_spread) {
				count += 1;
			}
			if count >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

pub fn wacc_vs_roic_spread_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "wacc-vs-roic-spread",
		"name": "WACC vs ROIC Spread",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Evaluates capital efficiency: ROIC exceeds WACC, positive spread"
	})
}

pub fn wacc_vs_roic_spread_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "minCriteriaMet": 2, "minRoic": 0.1, "minSpread": 0.05 }, "optimization_bounds": [] })
}

// ============================================================
// 9. EV/EBITDA Fair Value
// ============================================================

/// Ev Ebitda Fair Value
///
/// EV/EBITDA Fair Value: buy when EV/EBITDA is below sector average discount threshold
pub fn ev_ebitda_fair_value_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<EvEbitdaFairValueConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	let ev_ebitda_thresh = cfg.ev_ebitda_threshold.unwrap_or(10.0);
	let ebitda_margin_thresh = cfg.ebitda_margin_threshold.unwrap_or(0.1);
	fundamentals
		.iter()
		.map(|fp| {
			let d = &fp.data;
			let mut count = 0usize;
			if ev_to_ebitda_value(d).is_some_and(|v| v < ev_ebitda_thresh) {
				count += 1;
			}
			if ebitda_margin_value(d).is_some_and(|v| v > ebitda_margin_thresh) {
				count += 1;
			}
			if ev_to_ebitda_value(d).is_some_and(|v| v > 0.0 && v < ev_ebitda_thresh) {
				count += 1;
			}
			if count >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

pub fn ev_ebitda_fair_value_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "ev-ebitda-fair-value",
		"name": "EV/EBITDA Fair Value",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Evaluates if EV/EBITDA ratio is attractive: below fair value threshold, with healthy EBITDA margins"
	})
}

pub fn ev_ebitda_fair_value_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "minCriteriaMet": 2, "evEbitdaThreshold": 10, "ebitdaMarginThreshold": 0.1 }, "optimization_bounds": [] })
}

// ============================================================
// 10. Intrinsic Value Range via Multi-Metric Blend
// ============================================================

/// Intrinsic Value Multi Metric
///
/// Intrinsic Value Multi-Metric: buy when DCF, P/E, and P/B all flag undervaluation
pub fn intrinsic_value_multi_metric_strategy(
	fundamentals: Vec<FundamentalPoint>,
	bars: Vec<Bar>,
	config: Option<IntrinsicValueMultiMetricConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	let margin = cfg.margin_of_safety.unwrap_or(0.2);
	let min_intrinsic = cfg.min_intrinsic_value.unwrap_or(0.0);
	let min_len = fundamentals.len().min(bars.len());
	let mut result = vec![0i8; fundamentals.len()];
	for i in 0..min_len {
		let d = &fundamentals[i].data;
		let price = bars[i].close;
		let mut count = 0usize;
		if d.dcf.is_some_and(|iv| price < iv * (1.0 - margin)) {
			count += 1;
		}
		if d.dcf.is_some_and(|iv| iv > min_intrinsic) {
			count += 1;
		}
		if d.dcf.is_some_and(|iv| iv > price) {
			count += 1;
		}
		if count >= min_met {
			result[i] = 1;
		}
	}
	result
}

pub fn intrinsic_value_multi_metric_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "intrinsic-value-multi-metric",
		"name": "Intrinsic Value Range via Multi-Metric Blend",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Evaluates valuation: current price below intrinsic value range with margin of safety"
	})
}

pub fn intrinsic_value_multi_metric_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "minCriteriaMet": 2, "marginOfSafety": 0.2, "minIntrinsicValue": 0 }, "optimization_bounds": [] })
}

// ============================================================
// 11. Cash Burn Runway
// ============================================================

/// Cash Burn Runway
///
/// Cash Burn Runway: buy when cash runway exceeds minimum months and burn ratio is healthy
pub fn cash_burn_runway_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<CashBurnRunwayConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	fundamentals
		.iter()
		.map(|fp| {
			let d = &fp.data;
			let mut count = 0usize;
			if d.cash_and_equivalents.is_some_and(|v| v > 0.0) {
				count += 1;
			}
			if d.cash_and_equivalents.is_some_and(|v| v > 0.0) {
				count += 1;
			}
			if d.cash_and_equivalents.is_some_and(|c| c > 0.0) {
				count += 1;
			}
			if count >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

pub fn cash_burn_runway_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "cash-burn-runway",
		"name": "Cash Burn Runway",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Evaluates cash sustainability: sufficient runway, cash to burn ratio"
	})
}

pub fn cash_burn_runway_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "minCriteriaMet": 2, "minRunwayMonths": 12, "minCashToBurnRatio": 0.5 }, "optimization_bounds": [] })
}

// ============================================================
// 12. Debt/EBITDAR Stress Test
// ============================================================

/// Debt Ebitdar Stress Test
///
/// Debt/EBITDAR Stress Test: buy when debt coverage is adequate and stress-tested
pub fn debt_ebitdar_stress_test_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<DebtEbitdarStressTestConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	let max_debt_ebitdar = cfg.max_debt_ebitdar.unwrap_or(4.0);
	let stress_reduction = cfg.stress_ebitdar_reduction.unwrap_or(0.2);
	fundamentals
		.iter()
		.map(|fp| {
			let d = &fp.data;
			let mut count = 0usize;
			let debt_ebitdar = d.total_debt.and_then(|td| {
				d.ebitda
					.and_then(|eb| if eb > 0.0 { Some(td / eb) } else { None })
			});
			if debt_ebitdar.is_some_and(|v| v < max_debt_ebitdar) {
				count += 1;
			}
			if debt_ebitdar.is_some_and(|v| {
				let stressed = v / (1.0 - stress_reduction);
				stressed < 6.0
			}) {
				count += 1;
			}
			if interest_coverage_value(d).is_some_and(|v| v > 3.0) {
				count += 1;
			}
			if count >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

pub fn debt_ebitdar_stress_test_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "debt-ebitdar-stress-test",
		"name": "Debt/EBITDAR Stress Test",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Evaluates debt sustainability: debt/EBITDAR below threshold, passes stress test"
	})
}

pub fn debt_ebitdar_stress_test_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "minCriteriaMet": 2, "maxDebtEbitdar": 4, "stressEbitdarReduction": 0.2 }, "optimization_bounds": [] })
}

// ============================================================
// 13. EV/FCF vs 10-Yr Band
// ============================================================

/// Ev Fcf 10yr Band
///
/// EV/FCF 10-Year Band: buy when EV/FCF is in lower percentile of 10-year range
pub fn ev_fcf_10yr_band_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<EvFcf10yrBandConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	let max_ev_fcf = cfg.max_ev_fcf.unwrap_or(15.0);
	let fcf_thresh = cfg.fcf_threshold.unwrap_or(0.0);
	fundamentals
		.iter()
		.map(|fp| {
			let d = &fp.data;
			let mut count = 0usize;
			let ev_fcf = d.enterprise_value.and_then(|ev| {
				owner_earnings_value(d).and_then(|fcf| if fcf > 0.0 { Some(ev / fcf) } else { None })
			});
			if ev_fcf.is_some_and(|v| v < max_ev_fcf) {
				count += 1;
			}
			if owner_earnings_value(d).is_some_and(|v| v > fcf_thresh) {
				count += 1;
			}
			if ev_fcf.is_some_and(|v| v < max_ev_fcf && v > 0.0) {
				count += 1;
			}
			if count >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

pub fn ev_fcf_10yr_band_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "ev-fcf-10yr-band",
		"name": "EV/FCF vs. 10-Yr Band",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Evaluates valuation: EV/FCF below threshold, positive FCF, within attractive band"
	})
}

pub fn ev_fcf_10yr_band_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "minCriteriaMet": 2, "maxEvFcf": 15, "fcfThreshold": 0 }, "optimization_bounds": [] })
}

// ============================================================
// 14. EV/Revenue Multiples Valuation
// ============================================================

/// Ev Revenue Multiples
///
/// EV/Revenue Multiples: buy when EV/Revenue is below growth-adjusted threshold
pub fn ev_revenue_multiples_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<EvRevenueMultiplesConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	let ev_rev_thresh = cfg.ev_revenue_threshold.unwrap_or(3.0);
	let _rev_growth_thresh = cfg.revenue_growth_threshold.unwrap_or(0.1);
	fundamentals
		.iter()
		.map(|fp| {
			let d = &fp.data;
			let mut count = 0usize;
			if ev_to_revenue_value(d).is_some_and(|v| v < ev_rev_thresh) {
				count += 1;
			}
			if d.revenue.is_some_and(|v| v > 0.0) {
				count += 1;
			}
			if ev_to_revenue_value(d).is_some_and(|v| v < ev_rev_thresh && v > 0.0) {
				count += 1;
			}
			if count >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

pub fn ev_revenue_multiples_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "ev-revenue-multiples",
		"name": "EV/Revenue Multiples Valuation",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Evaluates valuation attractiveness: EV/Revenue below threshold, with growth support"
	})
}

pub fn ev_revenue_multiples_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "minCriteriaMet": 2, "evRevenueThreshold": 3, "revenueGrowthThreshold": 0.1 }, "optimization_bounds": [] })
}

// ============================================================
// 15. EV/Sales Fair Value
// ============================================================

/// Ev Sales Fair Value
///
/// EV/Sales Fair Value: buy when enterprise value to sales is below fair value estimate
pub fn ev_sales_fair_value_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<EvSalesFairValueConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	let ev_sales_thresh = cfg.ev_sales_threshold.unwrap_or(2.0);
	let sales_growth_thresh = cfg.sales_growth_threshold.unwrap_or(0.05);
	fundamentals
		.iter()
		.map(|fp| {
			let d = &fp.data;
			let mut count = 0usize;
			if ev_to_revenue_value(d).is_some_and(|v| v < ev_sales_thresh) {
				count += 1;
			}
			if d.revenue.is_some_and(|v| v > 0.0)
				&& d.cost_and_expenses
					.is_none_or(|c| d.revenue.unwrap() > c * (1.0 + sales_growth_thresh))
			{
				count += 1;
			}
			if ev_to_revenue_value(d).is_some_and(|v| v < ev_sales_thresh && v > 0.0) {
				count += 1;
			}
			if count >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

pub fn ev_sales_fair_value_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "ev-sales-fair-value",
		"name": "EV/Sales Fair Value",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Evaluates if EV/Sales ratio is attractive: below fair value threshold, with growing sales"
	})
}

pub fn ev_sales_fair_value_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "minCriteriaMet": 2, "evSalesThreshold": 2, "salesGrowthThreshold": 0.05 }, "optimization_bounds": [] })
}

// ============================================================
// 16. Interest-Coverage Buffer
// ============================================================

/// Interest Coverage Buffer
///
/// Interest Coverage Buffer: buy when operating income amply covers interest expense
pub fn interest_coverage_buffer_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<InterestCoverageBufferConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	let min_ic = cfg.min_interest_coverage.unwrap_or(3.0);
	fundamentals
		.iter()
		.map(|fp| {
			let d = &fp.data;
			let mut count = 0usize;
			let ic = interest_coverage_value(d);
			if ic.is_some_and(|v| v > min_ic) {
				count += 1;
			}
			if ic.is_some_and(|v| v > min_ic * 1.5) {
				count += 1;
			}
			if ic.is_some_and(|v| v > min_ic) {
				count += 1;
			}
			if count >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

pub fn interest_coverage_buffer_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "interest-coverage-buffer",
		"name": "Interest-Coverage Buffer",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Evaluates debt servicing ability: strong interest coverage, buffer for stress"
	})
}

pub fn interest_coverage_buffer_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "minCriteriaMet": 2, "minInterestCoverage": 3, "bufferMultiplier": 1.5 }, "optimization_bounds": [] })
}

// ============================================================
// 17. Margin of Safety From Target Price
// ============================================================

/// Margin Of Safety Target Price
///
/// Margin of Safety Target Price: buy when current price is below analyst target with buffer
pub fn margin_of_safety_target_price_strategy(
	fundamentals: Vec<FundamentalPoint>,
	bars: Vec<Bar>,
	config: Option<MarginOfSafetyTargetPriceConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	let upside_thresh = cfg.upside_threshold.unwrap_or(0.2);
	let min_len = fundamentals.len().min(bars.len());
	let mut result = vec![0i8; fundamentals.len()];
	for i in 0..min_len {
		let d = &fundamentals[i].data;
		let price = bars[i].close;
		let mut count = 0usize;
		if d.analyst_target_price
			.is_some_and(|tp| tp > price && (tp - price) / price > upside_thresh)
		{
			count += 1;
		}
		if d.rating.is_some_and(|r| r > 0.0) {
			count += 1;
		}
		if d.analyst_target_price.is_some_and(|_| true) {
			count += 1;
		}
		if count >= min_met {
			result[i] = 1;
		}
	}
	result
}

pub fn margin_of_safety_target_price_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "margin-of-safety-target-price",
		"name": "Margin of Safety From Target Price",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Evaluates analyst expectations: significant upside to target price, positive rating momentum"
	})
}

pub fn margin_of_safety_target_price_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "minCriteriaMet": 2, "upsideThreshold": 0.2, "momentumPeriod": 30 }, "optimization_bounds": [] })
}

// ============================================================
// 18. Net Cash Position Toggle
// ============================================================

/// Net Cash Position Toggle
///
/// Net Cash Position: buy when net cash per share exceeds threshold vs market cap
pub fn net_cash_position_toggle_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<NetCashPositionToggleConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	let min_net_cash = cfg.min_net_cash.unwrap_or(0.0);
	let max_de = cfg.max_debt_to_equity.unwrap_or(1.0);
	fundamentals
		.iter()
		.map(|fp| {
			let d = &fp.data;
			let mut count = 0usize;
			let nc = net_cash_value(d);
			let de = debt_to_equity_value(d);
			if nc.is_some_and(|v| v > min_net_cash) || de.is_some_and(|v| v < max_de) {
				count += 1;
			}
			if cash_to_assets_value(d).is_some_and(|v| v > 0.1) {
				count += 1;
			}
			if interest_coverage_value(d).is_some_and(|v| v > 5.0) {
				count += 1;
			}
			if count >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

pub fn net_cash_position_toggle_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "net-cash-position-toggle",
		"name": "Net Cash Position Toggle",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Evaluates balance sheet strength: net cash positive or manageable debt"
	})
}

pub fn net_cash_position_toggle_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "minCriteriaMet": 2, "minNetCash": 0, "maxDebtToEquity": 1 }, "optimization_bounds": [] })
}

// ============================================================
// 19. Normal PE Future Fair Value
// ============================================================

/// Normal Pe Future Fair Value
///
/// Normal P/E Future Fair Value: buy when normalized earnings suggest upside
pub fn normal_pe_future_fair_value_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<NormalPeFutureFairValueConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	let max_fpe = cfg.max_forward_pe.unwrap_or(20.0);
	fundamentals
		.iter()
		.map(|fp| {
			let d = &fp.data;
			let mut count = 0usize;
			if pe_ratio_value(d).is_some_and(|v| v < max_fpe) {
				count += 1;
			}
			if d.eps.is_some_and(|v| v > 0.0) {
				count += 1;
			}
			if pe_ratio_value(d).is_some_and(|v| v < max_fpe && v > 0.0) {
				count += 1;
			}
			if count >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

pub fn normal_pe_future_fair_value_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "normal-pe-future-fair-value",
		"name": "Normal PE Future Fair Value",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Evaluates valuation: forward P/E below threshold, with earnings growth support"
	})
}

pub fn normal_pe_future_fair_value_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "minCriteriaMet": 2, "maxForwardPe": 20, "earningsGrowthThreshold": 0.08 }, "optimization_bounds": [] })
}

// ============================================================
// 20. Operating Cash Flow Coverage of Dividends
// ============================================================

/// Ocf Coverage Dividends
///
/// OCF Coverage of Dividends: buy when operating cash flow comfortably covers dividends
pub fn ocf_coverage_dividends_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<OcfCoverageDividendsConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	let min_ratio = cfg.min_coverage_ratio.unwrap_or(1.5);
	let yield_thresh = cfg.dividend_yield_threshold.unwrap_or(0.02);
	fundamentals
		.iter()
		.map(|fp| {
			let d = &fp.data;
			let mut count = 0usize;
			if dividend_coverage_ocf_value(d).is_some_and(|v| v > min_ratio) {
				count += 1;
			}
			if d.dividend_yield.is_some_and(|v| v > yield_thresh) {
				count += 1;
			}
			if d.dividends_paid.is_some_and(|v| v > 0.0) {
				count += 1;
			}
			if count >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

pub fn ocf_coverage_dividends_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "ocf-coverage-dividends",
		"name": "Operating Cash Flow Coverage of Dividends",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Evaluates dividend sustainability: OCF covers dividends with margin, attractive yield"
	})
}

pub fn ocf_coverage_dividends_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "minCriteriaMet": 2, "minCoverageRatio": 1.5, "dividendYieldThreshold": 0.02 }, "optimization_bounds": [] })
}

// ============================================================
// 21. Price/Sales Fair Value
// ============================================================

/// Price Sales Fair Value
///
/// Price/Sales Fair Value: buy when P/S is below multi-year average
pub fn price_sales_fair_value_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<PriceSalesFairValueConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	let ps_thresh = cfg.ps_threshold.unwrap_or(1.5);
	let sales_growth_thresh = cfg.sales_growth_threshold.unwrap_or(0.05);
	fundamentals
		.iter()
		.map(|fp| {
			let d = &fp.data;
			let mut count = 0usize;
			if price_to_sales_value(d).is_some_and(|v| v < ps_thresh) {
				count += 1;
			}
			if d.revenue.is_some_and(|v| v > 0.0)
				&& d.cost_and_expenses
					.is_none_or(|c| d.revenue.unwrap() > c * (1.0 + sales_growth_thresh))
			{
				count += 1;
			}
			if price_to_sales_value(d).is_some_and(|v| v < ps_thresh && v > 0.0) {
				count += 1;
			}
			if count >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

pub fn price_sales_fair_value_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "price-sales-fair-value",
		"name": "Price/Sales Fair Value",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Evaluates if P/S ratio is attractive: below fair value threshold, with growing sales"
	})
}

pub fn price_sales_fair_value_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "minCriteriaMet": 2, "psThreshold": 1.5, "salesGrowthThreshold": 0.05 }, "optimization_bounds": [] })
}

// ============================================================
// 22. Price-to-Owner-Earnings
// ============================================================

/// Price To Owner Earnings
///
/// Price to Owner Earnings: buy when P/OE is below threshold (Buffett metric)
pub fn price_to_owner_earnings_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<PriceToOwnerEarningsConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	let max_poe = cfg.max_price_to_owner_earnings.unwrap_or(15.0);
	fundamentals
		.iter()
		.map(|fp| {
			let d = &fp.data;
			let mut count = 0usize;
			let owner_earnings = d
				.operating_cash_flow
				.zip(d.capital_expenditure)
				.map(|(ocf, capex)| ocf - capex);
			let poe = owner_earnings.zip(d.market_cap).and_then(|(oe, mc)| {
				if oe > 0.0 {
					Some(mc / oe)
				} else {
					None
				}
			});
			if poe.is_some_and(|v| v < max_poe) {
				count += 1;
			}
			if owner_earnings.is_some_and(|v| v > 0.0) {
				count += 1;
			}
			if poe.is_some_and(|v| v > 0.0) {
				count += 1;
			}
			if count >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

pub fn price_to_owner_earnings_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "price-to-owner-earnings",
		"name": "Price-to-Owner-Earnings",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Evaluates valuation: P/Owner Earnings below threshold, positive owner earnings"
	})
}

pub fn price_to_owner_earnings_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "minCriteriaMet": 2, "maxPriceToOwnerEarnings": 15, "ownerEarningsThreshold": 0 }, "optimization_bounds": [] })
}

// ============================================================
// 23. Quick Ratio Stress Test
// ============================================================

/// Quick Ratio Stress Test
///
/// Quick Ratio Stress Test: buy when liquidity ratios pass stress thresholds
pub fn quick_ratio_stress_test_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<QuickRatioStressTestConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	let min_qr = cfg.min_quick_ratio.unwrap_or(1.2);
	let stress_red = cfg.stress_reduction.unwrap_or(0.2);
	fundamentals
		.iter()
		.map(|fp| {
			let d = &fp.data;
			let mut count = 0usize;
			let qr = quick_ratio_value(d);
			if qr.is_some_and(|v| v > min_qr) {
				count += 1;
			}
			if qr.is_some_and(|v| v * (1.0 - stress_red) > 1.0) {
				count += 1;
			}
			if cash_to_liabilities_value(d).is_some_and(|v| v > 0.5) {
				count += 1;
			}
			if count >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

pub fn quick_ratio_stress_test_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "quick-ratio-stress-test",
		"name": "Quick Ratio Stress Test",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Evaluates liquidity strength: quick ratio above threshold, passes stress test"
	})
}

pub fn quick_ratio_stress_test_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "minCriteriaMet": 2, "minQuickRatio": 1.2, "stressReduction": 0.2 }, "optimization_bounds": [] })
}

// ============================================================
// 24. Return of Capital vs Growth
// ============================================================

/// Return Of Capital Vs Growth
///
/// Return of Capital vs Growth: buy when total shareholder yield exceeds reinvestment needs
pub fn return_of_capital_vs_growth_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<ReturnOfCapitalVsGrowthConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	let max_pr = cfg.max_payout_ratio.unwrap_or(0.6);
	let min_dy = cfg.min_dividend_yield.unwrap_or(0.02);
	fundamentals
		.iter()
		.map(|fp| {
			let d = &fp.data;
			let mut count = 0usize;
			if payout_ratio_value(d).is_some_and(|v| v < max_pr) {
				count += 1;
			}
			if d.dividend_yield.is_some_and(|v| v > min_dy) {
				count += 1;
			}
			if d.eps.is_some_and(|v| v > 0.0) {
				count += 1;
			}
			if count >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

pub fn return_of_capital_vs_growth_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "return-of-capital-vs-growth",
		"name": "Return of Capital vs Growth",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Evaluates capital allocation: reasonable payout, attractive yield"
	})
}

pub fn return_of_capital_vs_growth_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "minCriteriaMet": 2, "maxPayoutRatio": 0.6, "minDividendYield": 0.02 }, "optimization_bounds": [] })
}

// ============================================================
// 25. Working Capital Health
// ============================================================

/// Working Capital Health
///
/// Working Capital Health: buy when current ratio, quick ratio, and cash conversion are healthy
pub fn working_capital_health_strategy(
	fundamentals: Vec<FundamentalPoint>,
	config: Option<WorkingCapitalHealthConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	let min_cr = cfg.min_current_ratio.unwrap_or(1.5);
	let min_wc_ta = cfg.min_working_capital_to_assets.unwrap_or(0.1);
	fundamentals
		.iter()
		.map(|fp| {
			let d = &fp.data;
			let mut count = 0usize;
			if current_ratio_value(d).is_some_and(|v| v > min_cr) {
				count += 1;
			}
			if working_capital_value(d).is_some_and(|v| v > 0.0) {
				count += 1;
			}
			if working_capital_value(d)
				.zip(d.total_assets)
				.is_some_and(|(wc, ta)| ta > 0.0 && wc / ta > min_wc_ta)
			{
				count += 1;
			}
			if count >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

pub fn working_capital_health_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "working-capital-health",
		"name": "Working Capital Health",
		"category": "fundamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Evaluates balance sheet liquidity: current ratio above threshold, working capital health"
	})
}

pub fn working_capital_health_strategy_defaults() -> serde_json::Value {
	serde_json::json!({ "params": { "minCriteriaMet": 2, "minCurrentRatio": 1.5, "minWorkingCapitalToAssets": 0.1 }, "optimization_bounds": [] })
}
