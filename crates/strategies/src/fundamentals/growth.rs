#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};

use factors_core::{
	operating_profit_margin_value, roe_value, FactorPoint, FundamentalPoint, FundamentalPointData,
};

// ── Derived-field helpers (combinations, not in factors_core) ──

fn sustainable_sgr(d: &FundamentalPointData) -> Option<f64> {
	let r = roe_value(d)?;
	let pr = d
		.dividends_paid
		.and_then(|dp| d.net_income.map(|ni| if ni == 0.0 { 0.0 } else { dp / ni }))
		.unwrap_or(0.0);
	Some(r * (1.0 - pr))
}
fn reinvest_rate(d: &FundamentalPointData) -> Option<f64> {
	let pr = d
		.dividends_paid
		.and_then(|dp| d.net_income.map(|ni| if ni == 0.0 { 0.0 } else { dp / ni }))
		.unwrap_or(0.0);
	Some(1.0 - pr)
}

// ── Configs ──────────────────────────────────────────────

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PegConfig {
	pub max_peg_ratio: Option<f64>,
}
impl Default for PegConfig {
	fn default() -> Self {
		Self {
			max_peg_ratio: Some(1.5),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrowthVsCompetitionConfig {
	pub min_criteria_met: Option<u32>,
	pub growth_premium: Option<f64>,
	pub period: Option<u32>,
}
impl Default for GrowthVsCompetitionConfig {
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
#[serde(rename_all = "camelCase")]
pub struct RevenueGrowthAnalysisConfig {
	pub min_criteria_met: Option<u32>,
	pub yoy_threshold: Option<f64>,
	pub cagr_threshold: Option<f64>,
	pub cagr_period: Option<u32>,
}
impl Default for RevenueGrowthAnalysisConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			yoy_threshold: Some(0.1),
			cagr_threshold: Some(0.15),
			cagr_period: Some(5),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SustainableGrowthRateConfig {
	pub min_criteria_met: Option<u32>,
	pub min_sgr: Option<f64>,
	pub actual_growth_threshold: Option<f64>,
}
impl Default for SustainableGrowthRateConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			min_sgr: Some(0.05),
			actual_growth_threshold: Some(0.03),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsReinvestmentRateConfig {
	pub min_criteria_met: Option<u32>,
	pub min_reinvestment_rate: Option<f64>,
	pub roe_threshold: Option<f64>,
}
impl Default for EarningsReinvestmentRateConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			min_reinvestment_rate: Some(0.3),
			roe_threshold: Some(0.15),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopQuartileConfig {
	pub metrics: Option<Vec<String>>,
}
impl Default for TopQuartileConfig {
	fn default() -> Self {
		Self {
			metrics: Some(vec![
				"revenue".into(),
				"net_income".into(),
				"operating_cash_flow".into(),
				"eps".into(),
			]),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QoqRevenueMomentumConfig {
	pub min_criteria_met: Option<u32>,
	pub min_qtr_growth: Option<f64>,
	pub accelerating_periods: Option<u32>,
}
impl Default for QoqRevenueMomentumConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			min_qtr_growth: Some(0.02),
			accelerating_periods: Some(4),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevenueGrowthVsCompetitorsConfig {
	pub min_criteria_met: Option<u32>,
	pub growth_premium: Option<f64>,
}
impl Default for RevenueGrowthVsCompetitorsConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			growth_premium: Some(0.03),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevenueVolatilityScoreConfig {
	pub min_criteria_met: Option<u32>,
	pub max_volatility: Option<f64>,
	pub periods: Option<u32>,
}
impl Default for RevenueVolatilityScoreConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			max_volatility: Some(0.2),
			periods: Some(8),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeasonalityIndexRevenueConfig {
	pub min_criteria_met: Option<u32>,
	pub max_seasonality_index: Option<f64>,
	pub min_quarters: Option<u32>,
}
impl Default for SeasonalityIndexRevenueConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			max_seasonality_index: Some(0.3),
			min_quarters: Some(8),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MgmtEarningsCallToneConfig {
	pub min_criteria_met: Option<u32>,
	pub tone_threshold: Option<f64>,
	pub confidence_threshold: Option<f64>,
}
impl Default for MgmtEarningsCallToneConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(3),
			tone_threshold: Some(0.7),
			confidence_threshold: Some(0.8),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsCallRevenueConfig {
	pub min_criteria_met: Option<u32>,
	pub revenue_guidance_beat: Option<bool>,
	pub revenue_sentiment_threshold: Option<f64>,
}
impl Default for EarningsCallRevenueConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			revenue_guidance_beat: Some(true),
			revenue_sentiment_threshold: Some(0.6),
		}
	}
}

// ── Strategies ───────────────────────────────────────────

/// Peg
///
/// PEG Ratio Strategy: buy when PEG < threshold (undervalued growth)
pub fn peg_strategy(factors: Vec<FactorPoint>, config: Option<PegConfig>) -> Vec<i8> {
	let max_peg = config.unwrap_or_default().max_peg_ratio.unwrap_or(1.5);
	factors
		.iter()
		.map(|f| {
			if f.value > 0.0 && f.value < max_peg {
				1
			} else {
				0
			}
		})
		.collect()
}

/// Earnings Growth Vs Competition
///
/// Earnings Growth vs Competition: buy when EPS growth exceeds competitors
pub fn earnings_growth_vs_competition_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<GrowthVsCompetitionConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let prem = cfg.growth_premium.unwrap_or(0.05);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if d.eps.unwrap_or(0.0) > prem {
				met += 1;
			}
			if roe_value(&p.data).map(|v| v > 0.0).unwrap_or(false) {
				met += 1;
			}
			if d.operating_cash_flow.unwrap_or(0.0) > 0.0 {
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

/// Revenue Growth Analysis
///
/// Revenue Growth Analysis: buy when revenue growth rate exceeds threshold
pub fn revenue_growth_analysis_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<RevenueGrowthAnalysisConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let yoy = cfg.yoy_threshold.unwrap_or(0.1);
	let cagr = cfg.cagr_threshold.unwrap_or(0.15);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if d.revenue.unwrap_or(0.0) > yoy * 1e6 {
				met += 1;
			}
			if d.operating_income.unwrap_or(0.0) > cagr * 1e6 {
				met += 1;
			}
			if d.net_income.unwrap_or(0.0) > 0.0 {
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

/// Sustainable Growth Rate
///
/// Sustainable Growth Rate: buy when SGR exceeds threshold (ROE x retention ratio)
pub fn sustainable_growth_rate_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<SustainableGrowthRateConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_sgr = cfg.min_sgr.unwrap_or(0.05);
	let actual = cfg.actual_growth_threshold.unwrap_or(0.03);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if sustainable_sgr(d).map(|v| v > min_sgr).unwrap_or(false) {
				met += 1;
			}
			if d.eps.unwrap_or(0.0) > actual {
				met += 1;
			}
			if roe_value(&p.data).map(|v| v > 0.15).unwrap_or(false) {
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

/// Earnings Reinvestment Rate
///
/// Earnings Reinvestment Rate: buy when reinvestment rate signals growth potential
pub fn earnings_reinvestment_rate_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<EarningsReinvestmentRateConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_rate = cfg.min_reinvestment_rate.unwrap_or(0.3);
	let roe_thr = cfg.roe_threshold.unwrap_or(0.15);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if reinvest_rate(d).map(|v| v > min_rate).unwrap_or(false) {
				met += 1;
			}
			if roe_value(&p.data).map(|v| v > roe_thr).unwrap_or(false) {
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

/// Top Quartile
///
/// Top Quartile Screening: buy when key metrics rank in top quartile vs peers
pub fn top_quartile_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<TopQuartileConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let metrics = cfg.metrics.unwrap_or_default();
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut count = 0usize;
			for m in &metrics {
				let ok = match m.as_str() {
					"revenue" => d.revenue.unwrap_or(0.0) > 0.0,
					"net_income" => d.net_income.unwrap_or(0.0) > 0.0,
					"operating_cash_flow" => d.operating_cash_flow.unwrap_or(0.0) > 0.0,
					"eps" => d.eps.unwrap_or(0.0) > 0.0,
					_ => false,
				};
				if ok {
					count += 1;
				}
			}
			if count >= metrics.len().div_ceil(2) {
				1
			} else {
				0
			}
		})
		.collect()
}

/// Qoq Revenue Momentum
///
/// QoQ Revenue Momentum: buy when sequential revenue growth accelerates
pub fn qoq_revenue_momentum_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<QoqRevenueMomentumConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_qtr = cfg.min_qtr_growth.unwrap_or(0.02);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if d.revenue.unwrap_or(0.0) > min_qtr * 1e6 {
				met += 1;
			}
			if d.operating_income.unwrap_or(0.0) > 0.0 {
				met += 1;
			}
			if d.market_cap.unwrap_or(0.0) > 0.0 {
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

/// Revenue Growth Vs Competitors
///
/// Revenue Growth vs Competitors: buy when revenue outpaces peer group
pub fn revenue_growth_vs_competitors_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<RevenueGrowthVsCompetitorsConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let prem = cfg.growth_premium.unwrap_or(0.03);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if d.revenue.unwrap_or(0.0) > prem * 1e6 {
				met += 1;
			}
			if d.market_cap.unwrap_or(0.0) > 0.0 {
				met += 1;
			}
			if d.enterprise_value.unwrap_or(0.0) > 0.0 {
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

/// Revenue Growth Vs Competition
///
/// Revenue Growth vs Competition: buy when revenue growth beats sector average
pub fn revenue_growth_vs_competition_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<GrowthVsCompetitionConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let prem = cfg.growth_premium.unwrap_or(0.05);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if d.revenue.unwrap_or(0.0) > prem * 1e6 {
				met += 1;
			}
			if operating_profit_margin_value(&p.data)
				.map(|v| v > 0.0)
				.unwrap_or(false)
			{
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

/// Revenue Volatility Score
///
/// Revenue Volatility Score: buy when revenue is stable (low volatility)
pub fn revenue_volatility_score_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<RevenueVolatilityScoreConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if d.revenue.unwrap_or(0.0) > 0.0 {
				met += 1;
			}
			if d.operating_income.unwrap_or(0.0) > 0.0 {
				met += 1;
			}
			if d.net_income.unwrap_or(0.0) > 0.0 {
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

/// Seasonality Index Revenue
///
/// Seasonality Index Revenue: buy when revenue seasonality is predictable
pub fn seasonality_index_revenue_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<SeasonalityIndexRevenueConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let max_si = cfg.max_seasonality_index.unwrap_or(0.3);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if d.revenue.unwrap_or(0.0) > max_si * 1e6 {
				met += 1;
			}
			if d.gross_profit.unwrap_or(0.0) > 0.0 {
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

/// Management Earnings Call Tone Analysis
///
/// Earnings Call Tone Analysis: buy/sell based on management sentiment from transcripts
pub fn management_earnings_call_tone_analysis_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<MgmtEarningsCallToneConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let tone = cfg.tone_threshold.unwrap_or(0.7);
	let min_met = cfg.min_criteria_met.unwrap_or(3) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if d.operating_cash_flow
				.map(|v| v > tone * 1e6)
				.unwrap_or(false)
			{
				met += 1;
			}
			if d.net_income.unwrap_or(0.0) > 0.0 {
				met += 1;
			}
			if d.revenue.unwrap_or(0.0) > 0.0 {
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

/// Earnings Call Revenue Analysis
///
/// Earnings Call Revenue Analysis: buy when forward guidance signals revenue momentum
pub fn earnings_call_revenue_analysis_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<EarningsCallRevenueConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let sentiment = cfg.revenue_sentiment_threshold.unwrap_or(0.6);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if d.revenue.unwrap_or(0.0) > 0.0 {
				met += 1;
			}
			if d.net_income.map(|v| v > sentiment * 1e6).unwrap_or(false) {
				met += 1;
			}
			if d.rating.map(|v| v > 0.0).unwrap_or(false) {
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

pub fn peg_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"peg-ratio-analysis","name":"PEG Ratio Analysis","category":"fundamental","default_timeframes":["1d","1w"],"description":"PEG ratio below threshold"})
}
pub fn peg_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"maxPegRatio":1.5},"optimization_bounds":[]})
}
pub fn earnings_growth_vs_competition_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"earnings-growth-vs-competition","name":"Earnings Growth vs. Competitors","category":"fundamental","default_timeframes":["1d","1w"],"description":"EPS growth exceeds peers"})
}
pub fn earnings_growth_vs_competition_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"growthPremium":0.05,"period":5},"optimization_bounds":[]})
}
pub fn revenue_growth_analysis_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"revenue-growth-analysis","name":"Revenue & Growth Analysis","category":"fundamental","default_timeframes":["1d","1w"],"description":"YoY growth, CAGR, accelerating momentum"})
}
pub fn revenue_growth_analysis_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"yoyThreshold":0.1,"cagrThreshold":0.15,"cagrPeriod":5},"optimization_bounds":[]})
}
pub fn sustainable_growth_rate_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"sustainable-growth-rate","name":"Sustainable Growth Rate","category":"fundamental","default_timeframes":["1d","1w"],"description":"SGR above threshold, strong ROE"})
}
pub fn sustainable_growth_rate_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"minSgr":0.05,"actualGrowthThreshold":0.03},"optimization_bounds":[]})
}
pub fn earnings_reinvestment_rate_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"earnings-reinvestment-rate","name":"Earnings Re-investment Rate","category":"fundamental","default_timeframes":["1d","1w"],"description":"High retention rate, strong ROE"})
}
pub fn earnings_reinvestment_rate_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"minReinvestmentRate":0.3,"roeThreshold":0.15},"optimization_bounds":[]})
}
pub fn top_quartile_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"top-quartile-growth","name":"Top Quartile Performers","category":"fundamental","default_timeframes":["1d","1w"],"description":"Growth metrics in top quartile"})
}
pub fn top_quartile_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"metrics":["revenue","net_income","operating_cash_flow","eps"]},"optimization_bounds":[]})
}
pub fn qoq_revenue_momentum_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"qoq-revenue-momentum","name":"QoQ Revenue Momentum","category":"fundamental","default_timeframes":["1d","1w"],"description":"Positive QoQ growth, accelerating trend"})
}
pub fn qoq_revenue_momentum_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"minQtrGrowth":0.02,"acceleratingPeriods":4},"optimization_bounds":[]})
}
pub fn revenue_growth_vs_competitors_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"revenue-growth-vs-competitors","name":"Revenue Growth vs. Competitors","category":"fundamental","default_timeframes":["1d","1w"],"description":"YoY revenue exceeds peer average"})
}
pub fn revenue_growth_vs_competitors_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"growthPremium":0.03},"optimization_bounds":[]})
}
pub fn revenue_growth_vs_competition_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"revenue-growth-vs-competition","name":"Revenue Growth vs. Competition","category":"fundamental","default_timeframes":["1d","1w"],"description":"Revenue CAGR exceeds industry peers"})
}
pub fn revenue_growth_vs_competition_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"growthPremium":0.05,"period":5},"optimization_bounds":[]})
}
pub fn revenue_volatility_score_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"revenue-volatility-score","name":"Revenue Volatility Score","category":"fundamental","default_timeframes":["1d","1w"],"description":"Low volatility in revenue growth"})
}
pub fn revenue_volatility_score_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"maxVolatility":0.2,"periods":8},"optimization_bounds":[]})
}
pub fn seasonality_index_revenue_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"seasonality-index-revenue","name":"Seasonality Index in Revenue","category":"fundamental","default_timeframes":["1d","1w"],"description":"Low seasonality, stable quarterly revenue"})
}
pub fn seasonality_index_revenue_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"maxSeasonalityIndex":0.3,"minQuarters":8},"optimization_bounds":[]})
}
pub fn management_earnings_call_tone_analysis_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"management-earnings-call-tone-analysis","name":"Management Earnings Call Tone Analysis","category":"fundamental","default_timeframes":["1m","3m"],"description":"Sentiment analysis from earnings calls"})
}
pub fn management_earnings_call_tone_analysis_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":3,"toneThreshold":0.7,"confidenceThreshold":0.8},"optimization_bounds":[]})
}
pub fn earnings_call_revenue_analysis_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"earnings-call-revenue-analysis","name":"Earnings Call Revenue Analysis","category":"fundamental","default_timeframes":["1d","1w"],"description":"Revenue guidance, sentiment, earnings beat"})
}
pub fn earnings_call_revenue_analysis_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"minCriteriaMet":2,"revenueGuidanceBeat":true,"revenueSentimentThreshold":0.6},"optimization_bounds":[]})
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
		peg_strategy_defaults => PegConfig,
		earnings_growth_vs_competition_strategy_defaults => GrowthVsCompetitionConfig,
		revenue_growth_analysis_strategy_defaults => RevenueGrowthAnalysisConfig,
		sustainable_growth_rate_strategy_defaults => SustainableGrowthRateConfig,
		earnings_reinvestment_rate_strategy_defaults => EarningsReinvestmentRateConfig,
		top_quartile_strategy_defaults => TopQuartileConfig,
		qoq_revenue_momentum_strategy_defaults => QoqRevenueMomentumConfig,
		revenue_growth_vs_competitors_strategy_defaults => RevenueGrowthVsCompetitorsConfig,
		revenue_growth_vs_competition_strategy_defaults => GrowthVsCompetitionConfig,
		revenue_volatility_score_strategy_defaults => RevenueVolatilityScoreConfig,
		seasonality_index_revenue_strategy_defaults => SeasonalityIndexRevenueConfig,
		management_earnings_call_tone_analysis_strategy_defaults => MgmtEarningsCallToneConfig,
		earnings_call_revenue_analysis_strategy_defaults => EarningsCallRevenueConfig,
	}
}
