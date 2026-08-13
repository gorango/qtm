#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};

use factors_core::{
	current_ratio_value, debt_to_equity_value, ebitda_margin_value, fcf_margin_value,
	fcf_per_share_value, gross_margin_value, interest_coverage_value, net_debt_to_ebitda_value,
	net_margin_value, operating_profit_margin_value, pe_ratio_value, rnd_to_revenue_value,
	roa_value, roe_value, roic_value, working_capital_turnover_value, working_capital_value,
	FactorPoint, FundamentalPoint,
};

// ── Configs ──────────────────────────────────────────────

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct QualityConfig {
	pub roe_threshold: Option<f64>,
	pub periods: Option<u32>,
}
impl Default for QualityConfig {
	fn default() -> Self {
		Self {
			roe_threshold: Some(0.15),
			periods: Some(4),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DupontRoeConfig {
	pub min_roe: Option<f64>,
	pub min_net_margin: Option<f64>,
	pub min_asset_turnover: Option<f64>,
	pub max_equity_multiplier: Option<f64>,
	pub min_criteria_met: Option<u32>,
}
impl Default for DupontRoeConfig {
	fn default() -> Self {
		Self {
			min_roe: Some(0.15),
			min_net_margin: Some(0.05),
			min_asset_turnover: Some(0.7),
			max_equity_multiplier: Some(3.0),
			min_criteria_met: Some(3),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct QualityChecklistConfig {
	pub min_criteria_met: Option<u32>,
}
impl Default for QualityChecklistConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(7),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReturnOnCapitalConfig {
	pub roe_threshold: Option<f64>,
	pub roa_threshold: Option<f64>,
	pub roic_threshold: Option<f64>,
	pub min_criteria_met: Option<u32>,
}
impl Default for ReturnOnCapitalConfig {
	fn default() -> Self {
		Self {
			roe_threshold: Some(0.15),
			roa_threshold: Some(0.05),
			roic_threshold: Some(0.1),
			min_criteria_met: Some(2),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MarginChecklistConfig {
	pub min_criteria_met: Option<u32>,
	pub margin_threshold: Option<f64>,
	pub increasing_periods: Option<u32>,
}
impl Default for MarginChecklistConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(3),
			margin_threshold: Some(0.15),
			increasing_periods: Some(4),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EarningsQualityConfig {
	pub min_criteria_met: Option<u32>,
	pub sloan_max_ratio: Option<f64>,
	pub cash_flow_coverage: Option<f64>,
}
impl Default for EarningsQualityConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			sloan_max_ratio: Some(0.1),
			cash_flow_coverage: Some(1.2),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CapexDisciplineConfig {
	pub min_criteria_met: Option<u32>,
	pub max_capex_to_revenue: Option<f64>,
	pub capex_efficiency_threshold: Option<f64>,
}
impl Default for CapexDisciplineConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			max_capex_to_revenue: Some(0.1),
			capex_efficiency_threshold: Some(1.5),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EbitdaMarginConfig {
	pub min_criteria_met: Option<u32>,
	pub margin_threshold: Option<f64>,
	pub increasing_periods: Option<u32>,
}
impl Default for EbitdaMarginConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(3),
			margin_threshold: Some(0.2),
			increasing_periods: Some(4),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GrossProfitConfig {
	pub min_criteria_met: Option<u32>,
	pub margin_threshold: Option<f64>,
	pub stability_period: Option<u32>,
	pub variance: Option<f64>,
}
impl Default for GrossProfitConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			margin_threshold: Some(0.2),
			stability_period: Some(4),
			variance: Some(0.05),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OcfAnalysisConfig {
	pub min_criteria_met: Option<u32>,
	pub ocf_margin_threshold: Option<f64>,
	pub growth_periods: Option<u32>,
}
impl Default for OcfAnalysisConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			ocf_margin_threshold: Some(0.1),
			growth_periods: Some(4),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OperatingLeverageConfig {
	pub min_criteria_met: Option<u32>,
	pub min_operating_leverage: Option<f64>,
	pub improving_periods: Option<u32>,
}
impl Default for OperatingLeverageConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			min_operating_leverage: Some(1.5),
			improving_periods: Some(4),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CccCheckConfig {
	pub min_criteria_met: Option<u32>,
	pub max_cash_conversion_cycle: Option<f64>,
	pub min_improvement: Option<f64>,
}
impl Default for CccCheckConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			max_cash_conversion_cycle: Some(60.0),
			min_improvement: Some(5.0),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CccAnalysisConfig {
	pub min_criteria_met: Option<u32>,
	pub ccc_threshold: Option<f64>,
	pub improving_periods: Option<u32>,
}
impl Default for CccAnalysisConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			ccc_threshold: Some(30.0),
			improving_periods: Some(4),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MarginExpansionConfig {
	pub min_criteria_met: Option<u32>,
	pub margin_expansion_threshold: Option<f64>,
	pub periods: Option<u32>,
}
impl Default for MarginExpansionConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			margin_expansion_threshold: Some(0.05),
			periods: Some(20),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EbitdaGrowthVsCompetitionConfig {
	pub min_criteria_met: Option<u32>,
	pub growth_premium: Option<f64>,
	pub period: Option<u32>,
}
impl Default for EbitdaGrowthVsCompetitionConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			growth_premium: Some(0.05),
			period: Some(5),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EpsVsFcfDivergenceConfig {
	pub min_criteria_met: Option<u32>,
	pub max_divergence: Option<f64>,
	pub fcf_threshold: Option<f64>,
}
impl Default for EpsVsFcfDivergenceConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			max_divergence: Some(0.2),
			fcf_threshold: Some(0.0),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExpenseSurpriseConfig {
	pub min_criteria_met: Option<u32>,
	pub expense_beat_threshold: Option<f64>,
	pub margin_expansion_threshold: Option<f64>,
}
impl Default for ExpenseSurpriseConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			expense_beat_threshold: Some(0.05),
			margin_expansion_threshold: Some(0.02),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RevenueAssetsEfficiencyConfig {
	pub min_criteria_met: Option<u32>,
	pub revenue_to_assets_threshold: Option<f64>,
	pub improving_periods: Option<u32>,
}
impl Default for RevenueAssetsEfficiencyConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			revenue_to_assets_threshold: Some(0.5),
			improving_periods: Some(4),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RevenueDiversificationConfig {
	pub min_criteria_met: Option<u32>,
	pub max_revenue_concentration: Option<f64>,
	pub min_customer_count: Option<f64>,
}
impl Default for RevenueDiversificationConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			max_revenue_concentration: Some(0.3),
			min_customer_count: Some(1000.0),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RevenuePerEmployeeConfig {
	pub min_criteria_met: Option<u32>,
	pub revenue_per_employee_threshold: Option<f64>,
	pub improving_periods: Option<u32>,
}
impl Default for RevenuePerEmployeeConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			revenue_per_employee_threshold: Some(200000.0),
			improving_periods: Some(4),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RndIntensityConfig {
	pub min_criteria_met: Option<u32>,
	pub min_rnd_intensity: Option<f64>,
	pub max_rnd_intensity: Option<f64>,
}
impl Default for RndIntensityConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			min_rnd_intensity: Some(0.05),
			max_rnd_intensity: Some(0.2),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RoicDurabilityConfig {
	pub min_criteria_met: Option<u32>,
	pub min_roic_threshold: Option<f64>,
	pub volatility_threshold: Option<f64>,
	pub trend_threshold: Option<f64>,
}
impl Default for RoicDurabilityConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(3),
			min_roic_threshold: Some(0.08),
			volatility_threshold: Some(0.02),
			trend_threshold: Some(0.01),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkingCapitalEfficiencyConfig {
	pub min_criteria_met: Option<u32>,
	pub current_ratio_threshold: Option<f64>,
	pub working_capital_threshold: Option<f64>,
	pub turnover_threshold: Option<f64>,
}
impl Default for WorkingCapitalEfficiencyConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			current_ratio_threshold: Some(1.5),
			working_capital_threshold: Some(0.0),
			turnover_threshold: Some(2.0),
		}
	}
}

// ── Strategies ───────────────────────────────────────────

/// ROE threshold
pub fn quality_strategy(factors: Vec<FactorPoint>, config: Option<QualityConfig>) -> Vec<i8> {
	let thr = config.unwrap_or_default().roe_threshold.unwrap_or(0.15);
	factors
		.iter()
		.map(|f| if f.value > thr { 1 } else { 0 })
		.collect()
}

/// DuPont ROE: 4 criteria (ROE, Net Margin, Asset Turnover, Equity Multiplier)
pub fn dupont_roe_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<DupontRoeConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let (m_roe, m_nm, m_at, m_em) = (
		cfg.min_roe.unwrap_or(0.15),
		cfg.min_net_margin.unwrap_or(0.05),
		cfg.min_asset_turnover.unwrap_or(0.7),
		cfg.max_equity_multiplier.unwrap_or(3.0),
	);
	let min_met = cfg.min_criteria_met.unwrap_or(3) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let met = [
				roe_value(&p.data).map(|v| v > m_roe).unwrap_or(false),
				net_margin_value(&p.data).map(|v| v > m_nm).unwrap_or(false),
				d.asset_turnover.map(|v| v > m_at).unwrap_or(false),
				debt_to_equity_value(&p.data)
					.map(|v| 1.0 + v < m_em)
					.unwrap_or(false),
			]
			.iter()
			.filter(|&&x| x)
			.count();
			if met >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

/// Charlie Munger: 8 criteria
pub fn charlie_munger_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<QualityChecklistConfig>,
) -> Vec<i8> {
	let min_met = config.unwrap_or_default().min_criteria_met.unwrap_or(7) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let checks: [bool; 8] = [
				roic_value(&p.data).map(|v| v > 0.20).unwrap_or(false),
				operating_profit_margin_value(&p.data)
					.map(|v| v > 0.15)
					.unwrap_or(false),
				fcf_margin_value(&p.data).map(|v| v > 0.10).unwrap_or(false),
				d.revenue.unwrap_or(0.0) > 0.0,
				net_debt_to_ebitda_value(&p.data)
					.map(|v| v < 3.0)
					.unwrap_or(false),
				interest_coverage_value(&p.data)
					.map(|v| v > 10.0)
					.unwrap_or(false),
				d.asset_turnover.map(|v| v > 0.7).unwrap_or(false),
				pe_ratio_value(&p.data).map(|v| v < 25.0).unwrap_or(false),
			];
			if checks.iter().filter(|&&x| x).count() >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

/// Philip Fisher: 8 criteria
pub fn philip_fisher_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<QualityChecklistConfig>,
) -> Vec<i8> {
	let min_met = config.unwrap_or_default().min_criteria_met.unwrap_or(7) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let checks: [bool; 8] = [
				d.revenue.unwrap_or(0.0) > 0.0,
				rnd_to_revenue_value(&p.data)
					.map(|v| v > 0.03)
					.unwrap_or(false),
				operating_profit_margin_value(&p.data)
					.map(|v| v > 0.0)
					.unwrap_or(false),
				gross_margin_value(&p.data)
					.map(|v| v > 0.30)
					.unwrap_or(false),
				d.asset_turnover.map(|v| v > 0.5).unwrap_or(false),
				working_capital_turnover_value(&p.data)
					.map(|v| v > 4.0)
					.unwrap_or(false),
				roa_value(&p.data).map(|v| v > 0.08).unwrap_or(false),
				fcf_per_share_value(&p.data)
					.map(|v| v > 0.0)
					.unwrap_or(false),
			];
			if checks.iter().filter(|&&x| x).count() >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

/// Return on Capital: ROE, ROA, ROIC
pub fn return_on_capital_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<ReturnOnCapitalConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let (r_roe, r_roa, r_roic) = (
		cfg.roe_threshold.unwrap_or(0.15),
		cfg.roa_threshold.unwrap_or(0.05),
		cfg.roic_threshold.unwrap_or(0.1),
	);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let met = [
				roe_value(&p.data).map(|v| v > r_roe).unwrap_or(false),
				roa_value(&p.data).map(|v| v > r_roa).unwrap_or(false),
				roic_value(&p.data).map(|v| v > r_roic).unwrap_or(false),
			]
			.iter()
			.filter(|&&x| x)
			.count();
			if met >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

/// Operating Margin: 4 criteria
pub fn operating_margin_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<MarginChecklistConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let thr = cfg.margin_threshold.unwrap_or(0.15);
	let min_met = cfg.min_criteria_met.unwrap_or(3) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if operating_profit_margin_value(&p.data)
				.map(|v| v > thr)
				.unwrap_or(false)
			{
				met += 1;
			}
			if d.revenue.unwrap_or(0.0) > 0.0 {
				met += 1;
			}
			if d.operating_income.map(|v| v > 0.0).unwrap_or(false) {
				met += 1;
			}
			if d.operating_cash_flow.map(|v| v > 0.0).unwrap_or(false) {
				met += 1;
			}
			if met >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

/// Earnings Quality: Sloan Ratio, Cash Flow Coverage, Earnings Persistence
pub fn earnings_quality_analysis_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<EarningsQualityConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let sloan_max = cfg.sloan_max_ratio.unwrap_or(0.1);
	let cf_cov = cfg.cash_flow_coverage.unwrap_or(1.2);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			// Sloan Ratio ≈ (Net Income - Operating Cash Flow) / Total Assets
			if let (Some(ni), Some(ocf), Some(ta)) =
				(d.net_income, d.operating_cash_flow, d.total_assets)
			{
				if ta > 0.0 && (ni - ocf).abs() / ta < sloan_max {
					met += 1;
				}
			}
			if let (Some(ocf), Some(ni)) = (d.operating_cash_flow, d.net_income) {
				if ni > 0.0 && ocf / ni > cf_cov {
					met += 1;
				}
			}
			if d.net_income.map(|v| v > 0.0).unwrap_or(false)
				&& d.eps.map(|v| v > 0.0).unwrap_or(false)
			{
				met += 1;
			}
			if met >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

/// CapEx Discipline: 3 criteria
pub fn capex_discipline_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<CapexDisciplineConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let max_cr = cfg.max_capex_to_revenue.unwrap_or(0.1);
	let ce_eff = cfg.capex_efficiency_threshold.unwrap_or(1.5);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if let (Some(capex), Some(rev)) = (d.capital_expenditure, d.revenue) {
				if rev > 0.0 && capex / rev < max_cr {
					met += 1;
				}
			}
			if let (Some(capex), Some(inc_roe)) = (d.capital_expenditure, d.net_income) {
				if capex > 0.0 && inc_roe / capex > ce_eff {
					met += 1;
				}
			}
			if let (Some(capex), Some(ocf)) = (d.capital_expenditure, d.operating_cash_flow) {
				if ocf > capex {
					met += 1;
				}
			}
			if met >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

/// EBITDA Margin: 4 criteria
pub fn ebitda_margin_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<EbitdaMarginConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let thr = cfg.margin_threshold.unwrap_or(0.2);
	let min_met = cfg.min_criteria_met.unwrap_or(3) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if ebitda_margin_value(&p.data)
				.map(|v| v > thr)
				.unwrap_or(false)
			{
				met += 1;
			}
			if d.ebitda.unwrap_or(0.0) > 0.0 {
				met += 1;
			}
			if fcf_margin_value(&p.data).map(|v| v > 0.10).unwrap_or(false) {
				met += 1;
			}
			if let (Some(eb), Some(capex)) = (d.ebitda, d.capital_expenditure) {
				if eb > capex {
					met += 1;
				}
			}
			if met >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

/// Gross Profit Analysis: 3 criteria
pub fn gross_profit_analysis_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<GrossProfitConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let thr = cfg.margin_threshold.unwrap_or(0.2);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if gross_margin_value(&p.data)
				.map(|v| v > thr)
				.unwrap_or(false)
			{
				met += 1;
			}
			if d.gross_profit.unwrap_or(0.0) > 0.0 {
				met += 1;
			}
			if d.cost_of_revenue.unwrap_or(0.0) < d.revenue.unwrap_or(0.0) {
				met += 1;
			}
			if met >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

/// Operating Cashflow Analysis: 3 criteria
pub fn operating_cashflow_analysis_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<OcfAnalysisConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let ocf_margin = cfg.ocf_margin_threshold.unwrap_or(0.1);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if let (Some(ocf), Some(rev)) = (d.operating_cash_flow, d.revenue) {
				if rev > 0.0 && ocf / rev > ocf_margin {
					met += 1;
				}
			}
			if d.operating_cash_flow.unwrap_or(0.0) > 0.0 {
				met += 1;
			}
			if d.operating_cash_flow.unwrap_or(f64::MIN) > 0.0 {
				met += 1;
			} // redundant but matches TS
			if met >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

/// Operating Leverage Trend: 3 criteria
pub fn operating_leverage_trend_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<OperatingLeverageConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_lev = cfg.min_operating_leverage.unwrap_or(1.5);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if let (Some(op), Some(rev)) = (d.operating_income, d.revenue) {
				if rev > 0.0 && op / rev > 0.0 {
					met += 1;
				}
			}
			if d.operating_income.unwrap_or(0.0) > 0.0 {
				met += 1;
			}
			if let (Some(fc), Some(tc)) = (d.cost_and_expenses, d.revenue) {
				if tc > 0.0 && fc / tc > min_lev {
					met += 1;
				}
			}
			if met >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

/// Cash Conversion Cycle Check: 3 criteria
pub fn cash_conversion_cycle_check_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<CccCheckConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let max_ccc = cfg.max_cash_conversion_cycle.unwrap_or(60.0);
	let min_impr = cfg.min_improvement.unwrap_or(5.0);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	// Use the vec as a whole: prev_ccc from prior element
	points
		.iter()
		.enumerate()
		.map(|(i, p)| {
			let d = &p.data;
			let mut met = 0usize;
			let ccc = d.operating_cash_flow.zip(d.revenue).and_then(|(ocf, rev)| {
				if rev == 0.0 {
					None
				} else {
					Some(ocf / rev)
				}
			});
			if ccc.map(|v| v < max_ccc).unwrap_or(false) {
				met += 1;
			}
			if i > 0 {
				let prev = points[i - 1]
					.data
					.operating_cash_flow
					.zip(points[i - 1].data.revenue)
					.and_then(|(ocf, rev)| if rev == 0.0 { None } else { Some(ocf / rev) });
				if let (Some(cur_ccc), Some(prev_ccc)) = (ccc, prev) {
					if prev_ccc - cur_ccc > min_impr {
						met += 1;
					}
				}
			}
			if ccc.map(|v| v > 0.0).unwrap_or(false) {
				met += 1;
			}
			if met >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

/// Cash Conversion Cycle Analysis: 3 criteria
pub fn cash_conversion_cycle_analysis_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<CccAnalysisConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let ccc_thr = cfg.ccc_threshold.unwrap_or(30.0);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			let ccc = d.operating_cash_flow.zip(d.revenue).and_then(|(ocf, rev)| {
				if rev == 0.0 {
					None
				} else {
					Some(ocf / rev)
				}
			});
			if ccc.map(|v| v < ccc_thr).unwrap_or(false) {
				met += 1;
			}
			if d.asset_turnover.map(|v| v > 6.0).unwrap_or(false) {
				met += 1;
			}
			if d.operating_cash_flow.map(|v| v > 0.0).unwrap_or(false) {
				met += 1;
			}
			if met >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

/// Five-Year Margin Expansion: 3 criteria
pub fn five_year_margin_expansion_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<MarginExpansionConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let exp_thr = cfg.margin_expansion_threshold.unwrap_or(0.05);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			let nm = net_margin_value(&p.data);
			if nm.map(|v| v > 0.0).unwrap_or(false) {
				met += 1;
			}
			if nm.unwrap_or(0.0) > exp_thr {
				met += 1;
			}
			if d.operating_income.unwrap_or(0.0) > 0.0 {
				met += 1;
			}
			if met >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

/// EBITDA Growth vs. Competition: 3 criteria
pub fn ebitda_growth_vs_competition_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<EbitdaGrowthVsCompetitionConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let prem = cfg.growth_premium.unwrap_or(0.05);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if d.ebitda.unwrap_or(0.0) > 0.0 {
				met += 1;
			}
			if ebitda_margin_value(&p.data)
				.map(|v| v > prem)
				.unwrap_or(false)
			{
				met += 1;
			}
			if operating_profit_margin_value(&p.data)
				.map(|v| v > 0.0)
				.unwrap_or(false)
			{
				met += 1;
			}
			if met >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

/// EPS vs. FCF Divergence: 3 criteria
pub fn eps_vs_fcf_divergence_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<EpsVsFcfDivergenceConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let max_div = cfg.max_divergence.unwrap_or(0.2);
	let fcf_thr = cfg.fcf_threshold.unwrap_or(0.0);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if let (Some(eps_v), Some(fcf_v)) = (d.eps, fcf_per_share_value(&p.data)) {
				if eps_v > 0.0 && (eps_v - fcf_v).abs() / eps_v < max_div {
					met += 1;
				}
			}
			if fcf_per_share_value(&p.data)
				.map(|v| v > fcf_thr)
				.unwrap_or(false)
			{
				met += 1;
			}
			if let (Some(eps_v), Some(fcf_v)) = (d.eps, fcf_per_share_value(&p.data)) {
				if fcf_v > eps_v {
					met += 1;
				}
			}
			if met >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

/// Expense Surprise Detector: 3 criteria
pub fn expense_surprise_detector_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<ExpenseSurpriseConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let margin_exp = cfg.margin_expansion_threshold.unwrap_or(0.02);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if operating_profit_margin_value(&p.data)
				.map(|v| v > margin_exp)
				.unwrap_or(false)
			{
				met += 1;
			}
			if net_margin_value(&p.data)
				.map(|v| v > margin_exp)
				.unwrap_or(false)
			{
				met += 1;
			}
			if d.operating_cash_flow.map(|v| v > 0.0).unwrap_or(false) {
				met += 1;
			}
			if met >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

/// Revenue-to-Assets Efficiency: 3 criteria
pub fn revenue_assets_efficiency_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<RevenueAssetsEfficiencyConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let rta = cfg.revenue_to_assets_threshold.unwrap_or(0.5);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if let (Some(rev), Some(assets)) = (d.revenue, d.total_assets) {
				if assets > 0.0 && rev / assets > rta {
					met += 1;
				}
			}
			if d.asset_turnover.map(|v| v > 1.0).unwrap_or(false) {
				met += 1;
			}
			if d.revenue.unwrap_or(0.0) > 0.0 {
				met += 1;
			}
			if met >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

/// Revenue Diversification Proxy: 3 criteria
pub fn revenue_diversification_proxy_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<RevenueDiversificationConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let max_conc = cfg.max_revenue_concentration.unwrap_or(0.3);
	let min_cust = cfg.min_customer_count.unwrap_or(1000.0);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if d.revenue.map(|v| v > 0.0).unwrap_or(false) {
				met += 1;
			}
			if d.total_assets.map(|v| v > min_cust).unwrap_or(false) {
				met += 1;
			}
			if d.market_cap.map(|v| v > max_conc).unwrap_or(false) {
				met += 1;
			}
			if met >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

/// Revenue per Employee: 3 criteria
pub fn revenue_per_employee_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<RevenuePerEmployeeConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let thr = cfg.revenue_per_employee_threshold.unwrap_or(200000.0);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if d.revenue.map(|v| v > thr).unwrap_or(false) {
				met += 1;
			}
			if d.operating_income.map(|v| v > 0.0).unwrap_or(false) {
				met += 1;
			}
			if d.net_income.map(|v| v > 0.0).unwrap_or(false) {
				met += 1;
			}
			if met >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

/// R&D Intensity Tracker: 3 criteria
pub fn rnd_intensity_tracker_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<RndIntensityConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_int = cfg.min_rnd_intensity.unwrap_or(0.05);
	let max_int = cfg.max_rnd_intensity.unwrap_or(0.2);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if let Some(r) = rnd_to_revenue_value(&p.data) {
				if r >= min_int && r <= max_int {
					met += 1;
				}
			}
			if d.research_and_development_expenses.unwrap_or(0.0) > 0.0 {
				met += 1;
			}
			if d.operating_income.unwrap_or(0.0)
				> d.research_and_development_expenses.unwrap_or(0.0)
			{
				met += 1;
			}
			if met >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

/// ROIC Durability Sweep: 4 criteria
pub fn roic_durability_sweep_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<RoicDurabilityConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_roic = cfg.min_roic_threshold.unwrap_or(0.08);
	let min_met = cfg.min_criteria_met.unwrap_or(3) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if roic_value(&p.data).map(|v| v > min_roic).unwrap_or(false) {
				met += 1;
			}
			if roe_value(&p.data).map(|v| v > 0.0).unwrap_or(false) {
				met += 1;
			}
			if d.operating_income.unwrap_or(0.0) > 0.0 {
				met += 1;
			}
			if d.net_income.map(|v| v > 0.0).unwrap_or(false) {
				met += 1;
			}
			if met >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

/// Working Capital Efficiency: 3 criteria
pub fn working_capital_efficiency_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<WorkingCapitalEfficiencyConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let cr_thr = cfg.current_ratio_threshold.unwrap_or(1.5);
	let wc_thr = cfg.working_capital_threshold.unwrap_or(0.0);
	let to_thr = cfg.turnover_threshold.unwrap_or(2.0);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let mut met = 0usize;
			if current_ratio_value(&p.data)
				.map(|v| v > cr_thr)
				.unwrap_or(false)
			{
				met += 1;
			}
			if working_capital_value(&p.data)
				.map(|v| v > wc_thr)
				.unwrap_or(false)
			{
				met += 1;
			}
			if working_capital_turnover_value(&p.data)
				.map(|v| v > to_thr)
				.unwrap_or(false)
			{
				met += 1;
			}
			if met >= min_met {
				1
			} else {
				0
			}
		})
		.collect()
}

// ── Metadata ─────────────────────────────────────────────

pub fn quality_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"quality-investing","name":"Quality Investing Fundamental","category":"fundamental","default_timeframes":["1d","1w"],"description":"Quality investing strategy focusing on high ROE"})
}
pub fn quality_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"roeThreshold":0.15,"periods":4},"optimization_bounds":[]})
}
pub fn dupont_roe_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"dupont-roe-analysis","name":"DuPont ROE Analysis","category":"fundamental","default_timeframes":["1d","1w"],"description":"ROE breakdown: profitability, efficiency, leverage"})
}
pub fn dupont_roe_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minRoe":0.15,"minNetMargin":0.05,"minAssetTurnover":0.7,"maxEquityMultiplier":3,"minCriteriaMet":3},"optimization_bounds":[]})
}
pub fn charlie_munger_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"charlie-munger-quality","name":"Charlie Munger Quality at Fair Price","category":"fundamental","default_timeframes":["1d","1w"],"description":"8-criteria checklist"})
}
pub fn charlie_munger_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":7},"optimization_bounds":[]})
}
pub fn philip_fisher_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"philip-fisher-growth","name":"Philip Fisher Growth","category":"fundamental","default_timeframes":["1d","1w"],"description":"8-criteria growth checklist"})
}
pub fn philip_fisher_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":7},"optimization_bounds":[]})
}
pub fn return_on_capital_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"return-on-capital-metrics","name":"Return on Capital Metrics","category":"fundamental","default_timeframes":["1d","1w"],"description":"ROE, ROA, ROIC threshold"})
}
pub fn return_on_capital_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"roeThreshold":0.15,"roaThreshold":0.05,"roicThreshold":0.1,"minCriteriaMet":2},"optimization_bounds":[]})
}
pub fn operating_margin_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"operating-margin-analysis","name":"Operating Margin Analysis","category":"fundamental","default_timeframes":["1d","1w"],"description":"Operating margin quality and trend"})
}
pub fn operating_margin_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":3,"marginThreshold":0.15,"increasingPeriods":4},"optimization_bounds":[]})
}
pub fn earnings_quality_analysis_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"earnings-quality-analysis","name":"Earnings Quality Analysis","category":"fundamental","default_timeframes":["1d","1w"],"description":"Sloan Ratio, Cash Flow Coverage, Earnings Persistence"})
}
pub fn earnings_quality_analysis_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"sloanMaxRatio":0.1,"cashFlowCoverage":1.2},"optimization_bounds":[]})
}
pub fn capex_discipline_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"capex-discipline","name":"CapEx Discipline","category":"fundamental","default_timeframes":["1d","1w"],"description":"Capital allocation efficiency"})
}
pub fn capex_discipline_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"maxCapexToRevenue":0.1,"capexEfficiencyThreshold":1.5},"optimization_bounds":[]})
}
pub fn ebitda_margin_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"ebitda-margin-analysis","name":"EBITDA Margin Analysis","category":"fundamental","default_timeframes":["1d","1w"],"description":"EBITDA margin quality"})
}
pub fn ebitda_margin_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":3,"marginThreshold":0.2,"increasingPeriods":4},"optimization_bounds":[]})
}
pub fn gross_profit_analysis_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"gross-profit-analysis","name":"Gross Profit Analysis","category":"fundamental","default_timeframes":["1d","1w"],"description":"Gross margin threshold and stability"})
}
pub fn gross_profit_analysis_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"marginThreshold":0.2,"stabilityPeriod":4,"variance":0.05},"optimization_bounds":[]})
}
pub fn operating_cashflow_analysis_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"operating-cashflow-analysis","name":"Operating Cashflow Analysis","category":"fundamental","default_timeframes":["1d","1w"],"description":"OCF margin, growth, positivity"})
}
pub fn operating_cashflow_analysis_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"ocfMarginThreshold":0.1,"growthPeriods":4},"optimization_bounds":[]})
}
pub fn operating_leverage_trend_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"operating-leverage-trend","name":"Operating Leverage Trend","category":"fundamental","default_timeframes":["1d","1w"],"description":"Operating leverage level"})
}
pub fn operating_leverage_trend_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"minOperatingLeverage":1.5,"improvingPeriods":4},"optimization_bounds":[]})
}
pub fn cash_conversion_cycle_check_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"cash-conversion-cycle-check","name":"Cash Conversion Cycle Check","category":"fundamental","default_timeframes":["1d","1w"],"description":"CCC below threshold, improving, positive"})
}
pub fn cash_conversion_cycle_check_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"maxCashConversionCycle":60,"minImprovement":5},"optimization_bounds":[]})
}
pub fn cash_conversion_cycle_analysis_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"cash-conversion-cycle-analysis","name":"Cash Conversion Cycle Analysis","category":"fundamental","default_timeframes":["1d","1w"],"description":"CCC threshold, receivables efficiency"})
}
pub fn cash_conversion_cycle_analysis_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"cccThreshold":30,"improvingPeriods":4},"optimization_bounds":[]})
}
pub fn five_year_margin_expansion_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"five-year-margin-expansion","name":"Five-Year Margin Expansion","category":"fundamental","default_timeframes":["1d","1w"],"description":"Net margin expansion over 5 years"})
}
pub fn five_year_margin_expansion_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"marginExpansionThreshold":0.05,"periods":20},"optimization_bounds":[]})
}
pub fn ebitda_growth_vs_competition_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"ebitda-growth-vs-competition","name":"EBITDA Growth vs. Competitors","category":"fundamental","default_timeframes":["1d","1w"],"description":"EBITDA growth exceeds peers"})
}
pub fn ebitda_growth_vs_competition_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"growthPremium":0.05,"period":5},"optimization_bounds":[]})
}
pub fn eps_vs_fcf_divergence_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"eps-vs-fcf-divergence","name":"EPS vs. FCF Per-Share Divergence","category":"fundamental","default_timeframes":["1d","1w"],"description":"Earnings quality via EPS/FCF alignment"})
}
pub fn eps_vs_fcf_divergence_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"maxDivergence":0.2,"fcfThreshold":0},"optimization_bounds":[]})
}
pub fn expense_surprise_detector_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"expense-surprise-detector","name":"Expense Surprise Detector","category":"fundamental","default_timeframes":["1m","3m"],"description":"Expense beat expectations"})
}
pub fn expense_surprise_detector_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"expenseBeatThreshold":0.05,"marginExpansionThreshold":0.02},"optimization_bounds":[]})
}
pub fn revenue_assets_efficiency_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"revenue-assets-efficiency","name":"Revenue-to-Assets Efficiency","category":"fundamental","default_timeframes":["1d","1w"],"description":"Asset utilization efficiency"})
}
pub fn revenue_assets_efficiency_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"revenueToAssetsThreshold":0.5,"improvingPeriods":4},"optimization_bounds":[]})
}
pub fn revenue_diversification_proxy_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"revenue-diversification-proxy","name":"Revenue Diversification Proxy","category":"fundamental","default_timeframes":["1d","1w"],"description":"Revenue concentration and diversification"})
}
pub fn revenue_diversification_proxy_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"maxRevenueConcentration":0.3,"minCustomerCount":1000},"optimization_bounds":[]})
}
pub fn revenue_per_employee_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"revenue-per-employee","name":"Revenue per Employee Productivity","category":"fundamental","default_timeframes":["1d","1w"],"description":"Workforce productivity"})
}
pub fn revenue_per_employee_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"revenuePerEmployeeThreshold":200000,"improvingPeriods":4},"optimization_bounds":[]})
}
pub fn rnd_intensity_tracker_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"rnd-intensity-tracker","name":"R&D Intensity Tracker","category":"fundamental","default_timeframes":["1d","1w"],"description":"Innovation investment analysis"})
}
pub fn rnd_intensity_tracker_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"minRndIntensity":0.05,"maxRndIntensity":0.2},"optimization_bounds":[]})
}
pub fn roic_durability_sweep_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"roic-durability-sweep","name":"ROIC Durability Sweep","category":"fundamental","default_timeframes":["1m","1y"],"description":"ROIC consistency and trend"})
}
pub fn roic_durability_sweep_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":3,"minRoicThreshold":0.08,"volatilityThreshold":0.02,"trendThreshold":0.01},"optimization_bounds":[]})
}
pub fn working_capital_efficiency_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"working-capital-efficiency","name":"Working Capital Efficiency","category":"fundamental","default_timeframes":["1d","1w"],"description":"Working capital management quality"})
}
pub fn working_capital_efficiency_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"currentRatioThreshold":1.5,"workingCapitalThreshold":0,"turnoverThreshold":2},"optimization_bounds":[]})
}

#[cfg(test)]
mod defaults_tests {
	use super::*;

	macro_rules! check_defaults {
		($($defaults_fn:ident => $cfg:ty),* $(,)?) => {
			$(
				#[test]
				fn $defaults_fn() {
					let defaults = super::$defaults_fn();
					let params = defaults["params"].clone();
					let cfg: $cfg = serde_json::from_value(params.clone())
						.expect("defaults params must deserialize");
					let canonical = serde_json::to_value(&cfg).unwrap();
					for (k, v) in params.as_object().unwrap() {
						let expected = canonical.get(k).unwrap_or(&serde_json::Value::Null);
						let matches = match (expected.as_f64(), v.as_f64()) {
							(Some(a), Some(b)) => a == b,
							_ => expected == v,
						};
						assert!(
							matches,
							"key `{k}` is not a recognized field of {}",
							stringify!($cfg)
						);
					}
				}
			)*
		};
	}

	check_defaults! {
		quality_strategy_defaults => QualityConfig,
		dupont_roe_strategy_defaults => DupontRoeConfig,
		charlie_munger_strategy_defaults => QualityChecklistConfig,
		philip_fisher_strategy_defaults => QualityChecklistConfig,
		return_on_capital_strategy_defaults => ReturnOnCapitalConfig,
		operating_margin_strategy_defaults => MarginChecklistConfig,
		earnings_quality_analysis_strategy_defaults => EarningsQualityConfig,
		capex_discipline_strategy_defaults => CapexDisciplineConfig,
		ebitda_margin_strategy_defaults => EbitdaMarginConfig,
		gross_profit_analysis_strategy_defaults => GrossProfitConfig,
		operating_cashflow_analysis_strategy_defaults => OcfAnalysisConfig,
		operating_leverage_trend_strategy_defaults => OperatingLeverageConfig,
		cash_conversion_cycle_check_strategy_defaults => CccCheckConfig,
		cash_conversion_cycle_analysis_strategy_defaults => CccAnalysisConfig,
		five_year_margin_expansion_strategy_defaults => MarginExpansionConfig,
		ebitda_growth_vs_competition_strategy_defaults => EbitdaGrowthVsCompetitionConfig,
		eps_vs_fcf_divergence_strategy_defaults => EpsVsFcfDivergenceConfig,
		expense_surprise_detector_strategy_defaults => ExpenseSurpriseConfig,
		revenue_assets_efficiency_strategy_defaults => RevenueAssetsEfficiencyConfig,
		revenue_diversification_proxy_strategy_defaults => RevenueDiversificationConfig,
		revenue_per_employee_strategy_defaults => RevenuePerEmployeeConfig,
		rnd_intensity_tracker_strategy_defaults => RndIntensityConfig,
		roic_durability_sweep_strategy_defaults => RoicDurabilityConfig,
		working_capital_efficiency_strategy_defaults => WorkingCapitalEfficiencyConfig,
	}
}
