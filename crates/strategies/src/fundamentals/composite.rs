#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};

use factors_core::{FactorPoint, FundamentalPoint, FundamentalPointData};

// ── Derived-field helpers ───────────────────────────────

fn roe(d: &FundamentalPointData) -> Option<f64> {
	let e = d.shareholders_equity?;
	if e == 0.0 {
		None
	} else {
		Some(d.net_income? / e)
	}
}
fn net_margin(d: &FundamentalPointData) -> Option<f64> {
	let r = d.revenue?;
	if r == 0.0 {
		None
	} else {
		Some(d.net_income? / r)
	}
}
fn op_margin(d: &FundamentalPointData) -> Option<f64> {
	let r = d.revenue?;
	if r == 0.0 {
		None
	} else {
		Some(d.operating_income? / r)
	}
}
fn gross_margin(d: &FundamentalPointData) -> Option<f64> {
	let r = d.revenue?;
	if r == 0.0 {
		None
	} else {
		Some(d.gross_profit? / r)
	}
}
fn de_to_equity(d: &FundamentalPointData) -> Option<f64> {
	let e = d.shareholders_equity?;
	if e == 0.0 {
		None
	} else {
		Some(d.total_debt? / e)
	}
}
fn current_ratio(d: &FundamentalPointData) -> Option<f64> {
	let l = d.current_liabilities?;
	if l == 0.0 {
		None
	} else {
		Some(d.current_assets? / l)
	}
}
fn wc_turnover(d: &FundamentalPointData) -> Option<f64> {
	let wc = d.current_assets? - d.current_liabilities?;
	if wc == 0.0 {
		None
	} else {
		Some(d.revenue? / wc)
	}
}
fn roa(d: &FundamentalPointData) -> Option<f64> {
	let a = d.total_assets?;
	if a == 0.0 {
		None
	} else {
		Some(d.net_income? / a)
	}
}
fn roic(d: &FundamentalPointData) -> Option<f64> {
	let cap = d.total_assets? - d.cash_and_equivalents.unwrap_or(0.0) - d.current_liabilities?;
	if cap == 0.0 {
		None
	} else {
		Some(d.operating_income? / cap)
	}
}
fn fcf(d: &FundamentalPointData) -> Option<f64> {
	Some(d.operating_cash_flow? - d.capital_expenditure?)
}
fn fcf_margin(d: &FundamentalPointData) -> Option<f64> {
	let r = d.revenue?;
	if r == 0.0 {
		None
	} else {
		Some(fcf(d)? / r)
	}
}
fn fcf_per_share(d: &FundamentalPointData) -> Option<f64> {
	let s = d.shares_outstanding?;
	if s == 0.0 {
		None
	} else {
		Some(fcf(d)? / s)
	}
}
fn interest_cov(d: &FundamentalPointData) -> Option<f64> {
	let i = d.interest_expense?;
	if i == 0.0 {
		None
	} else {
		Some(d.operating_income? / i)
	}
}
fn pe_ratio(d: &FundamentalPointData) -> Option<f64> {
	let n = d.net_income?;
	if n == 0.0 {
		None
	} else {
		Some(d.market_cap? / n)
	}
}
fn price_to_book(d: &FundamentalPointData) -> Option<f64> {
	let e = d.shareholders_equity?;
	if e == 0.0 {
		None
	} else {
		Some(d.market_cap? / e)
	}
}
fn earnings_yield(d: &FundamentalPointData) -> Option<f64> {
	let ev = d.enterprise_value?;
	if ev == 0.0 {
		None
	} else {
		Some(d.net_income? / ev)
	}
}
fn asset_turnover(d: &FundamentalPointData) -> Option<f64> {
	let a = d.total_assets?;
	if a == 0.0 {
		None
	} else {
		Some(d.revenue? / a)
	}
}
fn net_debt_ebitda(d: &FundamentalPointData) -> Option<f64> {
	let e = d.ebitda?;
	if e == 0.0 {
		None
	} else {
		Some((d.total_debt? - d.cash_and_equivalents.unwrap_or(0.0)) / e)
	}
}
fn rnd_to_rev(d: &FundamentalPointData) -> Option<f64> {
	let r = d.revenue?;
	if r == 0.0 {
		None
	} else {
		Some(d.research_and_development_expenses? / r)
	}
}
fn sustainable_sgr(d: &FundamentalPointData) -> Option<f64> {
	let r = roe(d)?;
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
pub struct AltmanZScoreConfig {
	pub z_score_threshold: Option<f64>,
}
impl Default for AltmanZScoreConfig {
	fn default() -> Self {
		Self {
			z_score_threshold: Some(3.0),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PiotroskiConfig {
	pub f_score_threshold: Option<u32>,
}
impl Default for PiotroskiConfig {
	fn default() -> Self {
		Self {
			f_score_threshold: Some(7),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MagicFormulaConfig {
	pub earnings_yield_threshold: Option<f64>,
	pub return_on_capital_threshold: Option<f64>,
}
impl Default for MagicFormulaConfig {
	fn default() -> Self {
		Self {
			earnings_yield_threshold: Some(0.1),
			return_on_capital_threshold: Some(0.25),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JoelGreenblattConfig {
	pub earnings_yield_threshold: Option<f64>,
	pub return_on_capital_threshold: Option<f64>,
}
impl Default for JoelGreenblattConfig {
	fn default() -> Self {
		Self {
			earnings_yield_threshold: Some(0.06),
			return_on_capital_threshold: Some(0.25),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuiteConfig {
	pub threshold: Option<f64>,
}
impl Default for SuiteConfig {
	fn default() -> Self {
		Self {
			threshold: Some(0.6),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MultiFactorSuiteConfig {
	pub value_weight: Option<f64>,
	pub growth_weight: Option<f64>,
	pub quality_weight: Option<f64>,
	pub threshold: Option<f64>,
}
impl Default for MultiFactorSuiteConfig {
	fn default() -> Self {
		Self {
			value_weight: Some(0.4),
			growth_weight: Some(0.3),
			quality_weight: Some(0.3),
			threshold: Some(0.6),
		}
	}
}

// ── Strategies ───────────────────────────────────────────

/// Altman Z Score
///
/// Altman Z-Score: buy when Z > 2.99 (safe zone), sell when Z < 1.81 (distress zone)
pub fn altman_z_score_strategy(
	factors: Vec<FactorPoint>,
	config: Option<AltmanZScoreConfig>,
) -> Vec<i8> {
	let thr = config.unwrap_or_default().z_score_threshold.unwrap_or(3.0);
	factors
		.iter()
		.map(|f| if f.value > thr { 1 } else { 0 })
		.collect()
}

/// Piotroski
///
/// Piotroski F-Score: buy when F-Score >= 7 (strong fundamentals), avoid when <= 3
pub fn piotroski_strategy(factors: Vec<FactorPoint>, config: Option<PiotroskiConfig>) -> Vec<i8> {
	let thr = config.unwrap_or_default().f_score_threshold.unwrap_or(7) as f64;
	factors
		.iter()
		.map(|f| if f.value >= thr { 1 } else { 0 })
		.collect()
}

/// Magic Formula
///
/// Greenblatt Magic Formula: buy when combined earnings yield + ROC rank is high
pub fn magic_formula_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<MagicFormulaConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let ey = cfg.earnings_yield_threshold.unwrap_or(0.1);
	let roc = cfg.return_on_capital_threshold.unwrap_or(0.25);
	points
		.iter()
		.map(|p| {
			let (ey_v, roc_v) = (earnings_yield(&p.data), roic(&p.data));
			match (ey_v, roc_v) {
				(Some(e), Some(r)) if e > ey && r > roc => 1,
				_ => 0,
			}
		})
		.collect()
}

/// Joel Greenblatt
///
/// Joel Greenblatt variant: buy when earnings yield and return on capital exceed thresholds
pub fn joel_greenblatt_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<JoelGreenblattConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let ey = cfg.earnings_yield_threshold.unwrap_or(0.06);
	let roc = cfg.return_on_capital_threshold.unwrap_or(0.25);
	points
		.iter()
		.map(|p| {
			let (ey_v, roc_v) = (earnings_yield(&p.data), roic(&p.data));
			match (ey_v, roc_v) {
				(Some(e), Some(r)) if e > ey && r > roc => 1,
				_ => 0,
			}
		})
		.collect()
}

/// Growth Investing Suite
///
/// Growth Investing Suite: composite buy signal when multiple growth criteria are met
pub fn growth_investing_suite_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<SuiteConfig>,
) -> Vec<i8> {
	let thr = config.unwrap_or_default().threshold.unwrap_or(0.6);
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut total = 0usize;
			if d.market_cap.unwrap_or(0.0) > 0.0 && d.net_income.unwrap_or(0.0) > 0.0 {
				total += 1;
			}
			if d.eps.unwrap_or(0.0) > 0.0 && roe(d).unwrap_or(0.0) > 0.0 {
				total += 1;
			}
			if reinvest_rate(d).map(|v| v > 0.8).unwrap_or(false) {
				total += 1;
			}
			if d.revenue.unwrap_or(0.0) > 1e6 && gross_margin(d).unwrap_or(0.0) > 0.0 {
				total += 1;
			}
			if sustainable_sgr(d).map(|v| v > 0.1).unwrap_or(false) {
				total += 1;
			}
			let score = total as f64 / 5.0;
			if score >= thr {
				1
			} else {
				0
			}
		})
		.collect()
}

/// Quality Investing Suite
///
/// Quality Investing Suite: composite buy signal when multiple quality criteria are met
pub fn quality_investing_suite_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<SuiteConfig>,
) -> Vec<i8> {
	let thr = config.unwrap_or_default().threshold.unwrap_or(0.6);
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut total = 0usize;
			let munger: [bool; 8] = [
				roic(d).map(|v| v > 0.20).unwrap_or(false),
				op_margin(d).map(|v| v > 0.15).unwrap_or(false),
				fcf_margin(d).map(|v| v > 0.10).unwrap_or(false),
				d.revenue.unwrap_or(0.0) > 0.0,
				net_debt_ebitda(d).map(|v| v < 3.0).unwrap_or(false),
				interest_cov(d).map(|v| v > 10.0).unwrap_or(false),
				asset_turnover(d).map(|v| v > 0.7).unwrap_or(false),
				pe_ratio(d).map(|v| v < 25.0).unwrap_or(false),
			];
			if munger.iter().filter(|&&x| x).count() >= 7 {
				total += 1;
			}
			let dupont: [bool; 4] = [
				roe(d).map(|v| v > 0.15).unwrap_or(false),
				net_margin(d).map(|v| v > 0.05).unwrap_or(false),
				asset_turnover(d).map(|v| v > 0.7).unwrap_or(false),
				de_to_equity(d).map(|v| 1.0 + v < 3.0).unwrap_or(false),
			];
			if dupont.iter().filter(|&&x| x).count() >= 3 {
				total += 1;
			}
			let fisher: [bool; 8] = [
				d.revenue.unwrap_or(0.0) > 0.0,
				rnd_to_rev(d).map(|v| v > 0.03).unwrap_or(false),
				op_margin(d).map(|v| v > 0.0).unwrap_or(false),
				gross_margin(d).map(|v| v > 0.30).unwrap_or(false),
				asset_turnover(d).map(|v| v > 0.5).unwrap_or(false),
				wc_turnover(d).map(|v| v > 4.0).unwrap_or(false),
				roa(d).map(|v| v > 0.08).unwrap_or(false),
				fcf_per_share(d).map(|v| v > 0.0).unwrap_or(false),
			];
			if fisher.iter().filter(|&&x| x).count() >= 7 {
				total += 1;
			}
			let score = total as f64 / 3.0;
			if score >= thr {
				1
			} else {
				0
			}
		})
		.collect()
}

/// Value Investing Suite
///
/// Value Investing Suite: composite buy signal when multiple value criteria are met
pub fn value_investing_suite_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<SuiteConfig>,
) -> Vec<i8> {
	let thr = config.unwrap_or_default().threshold.unwrap_or(0.6);
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut total = 0usize;
			if pe_ratio(d).map(|v| v < 15.0).unwrap_or(false)
				&& price_to_book(d).map(|v| v < 1.5).unwrap_or(false)
				&& de_to_equity(d).map(|v| v < 1.1).unwrap_or(false)
				&& current_ratio(d).map(|v| v > 1.5).unwrap_or(false)
			{
				total += 1;
			}
			if pe_ratio(d).map(|v| v < 10.0).unwrap_or(false)
				&& de_to_equity(d).map(|v| v < 1.0).unwrap_or(false)
			{
				total += 1;
			}
			if price_to_book(d).map(|v| v < 1.2).unwrap_or(false)
				&& de_to_equity(d).map(|v| v < 0.5).unwrap_or(false)
				&& d.net_income.map(|v| v > 0.0).unwrap_or(false)
			{
				total += 1;
			}
			if roe(d).map(|v| v > 0.15).unwrap_or(false)
				&& d.revenue.unwrap_or(0.0) > 0.0
				&& pe_ratio(d).map(|v| v < 25.0).unwrap_or(false)
			{
				total += 1;
			}
			let score = total as f64 / 4.0;
			if score >= thr {
				1
			} else {
				0
			}
		})
		.collect()
}

/// Multi Factor Suite
///
/// Multi-Factor Suite: composite buy signal across value, quality, and growth factors
pub fn multi_factor_suite_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<MultiFactorSuiteConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let vw = cfg.value_weight.unwrap_or(0.4);
	let gw = cfg.growth_weight.unwrap_or(0.3);
	let qw = cfg.quality_weight.unwrap_or(0.3);
	let thr = cfg.threshold.unwrap_or(0.6);
	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let vs = d
				.market_cap
				.zip(d.net_income)
				.map(|(mc, ni)| if mc == 0.0 { 0.0 } else { (ni / mc).min(1.0) })
				.unwrap_or(0.0);
			let gs = d
				.revenue
				.zip(d.operating_income)
				.map(|(rev, op)| if rev == 0.0 { 0.0 } else { (op / rev).min(1.0) })
				.unwrap_or(0.0);
			let qs = roe(d).map(|r| r.min(1.0)).unwrap_or(0.0);
			let composite = vw * vs + gw * gs + qw * qs;
			if composite > thr {
				1
			} else {
				0
			}
		})
		.collect()
}

// ── Metadata ─────────────────────────────────────────────

pub fn altman_z_score_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"altman-z-score","name":"Altman Z-Score Bankruptcy Prediction","category":"fundamental","default_timeframes":["1d","1w"],"description":"Invests in companies with low bankruptcy risk"})
}
pub fn altman_z_score_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"z_score_threshold":3},"optimization_bounds":[]})
}
pub fn piotroski_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"piotroski-f-score","name":"Piotroski F-Score Quality","category":"fundamental","default_timeframes":["1d","1w"],"description":"Companies with F-Score >= 7"})
}
pub fn piotroski_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"f_score_threshold":7},"optimization_bounds":[]})
}
pub fn magic_formula_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"magic-formula","name":"Magic Formula Investing","category":"fundamental","default_timeframes":["1d","1w"],"description":"High earnings yield and return on capital"})
}
pub fn magic_formula_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"earnings_yield_threshold":0.1,"return_on_capital_threshold":0.25},"optimization_bounds":[]})
}
pub fn joel_greenblatt_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"joel-greenblatt-magic-formula","name":"Joel Greenblatt Magic Formula","category":"fundamental","default_timeframes":["1d","1w"],"description":"Greenblatt's magic formula"})
}
pub fn joel_greenblatt_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"earnings_yield_threshold":0.06,"return_on_capital_threshold":0.25},"optimization_bounds":[]})
}
pub fn growth_investing_suite_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"growth-investing-suite","name":"Growth Investing Suite","category":"fundamental","default_timeframes":["1m","1y"],"description":"Combines multiple growth approaches"})
}
pub fn growth_investing_suite_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"threshold":0.6},"optimization_bounds":[]})
}
pub fn quality_investing_suite_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"quality-investing-suite","name":"Quality Investing Suite","category":"fundamental","default_timeframes":["1m","1y"],"description":"Combines Munger, DuPont, Fisher checklists"})
}
pub fn quality_investing_suite_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"threshold":0.6},"optimization_bounds":[]})
}
pub fn value_investing_suite_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"value-investing-suite","name":"Value Investing Suite","category":"fundamental","default_timeframes":["1m","1y"],"description":"Combines Graham, Templeton, Schloss, Miller"})
}
pub fn value_investing_suite_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"threshold":0.6},"optimization_bounds":[]})
}
pub fn multi_factor_suite_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"multi-factor-suite","name":"Multi-Factor Suite","category":"fundamental","default_timeframes":["1m","1y"],"description":"Weighted value, growth, quality factors"})
}
pub fn multi_factor_suite_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"value_weight":0.4,"growth_weight":0.3,"quality_weight":0.3,"threshold":0.6},"optimization_bounds":[]})
}
