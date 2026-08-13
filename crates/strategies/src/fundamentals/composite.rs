#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};

use factors_core::{
	current_ratio_value, debt_to_equity_value, earnings_yield_value, fcf_margin_value,
	fcf_per_share_value, gross_margin_value, interest_coverage_value, net_debt_to_ebitda_value,
	net_margin_value, operating_profit_margin_value, pe_ratio_value, price_to_book_value,
	rnd_to_revenue_value, roa_value, roe_value, roic_value, working_capital_turnover_value,
	FactorPoint, FundamentalPoint, FundamentalPointData,
};

// ── Derived-field helpers ───────────────────────────────

fn asset_turnover(d: &FundamentalPointData) -> Option<f64> {
	let a = d.total_assets?;
	if a == 0.0 {
		None
	} else {
		Some(d.revenue? / a)
	}
}
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
#[serde(rename_all = "camelCase", deny_unknown_fields)]
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
#[serde(rename_all = "camelCase", deny_unknown_fields)]
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
#[serde(rename_all = "camelCase", deny_unknown_fields)]
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
#[serde(rename_all = "camelCase", deny_unknown_fields)]
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
#[serde(rename_all = "camelCase", deny_unknown_fields)]
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
#[serde(rename_all = "camelCase", deny_unknown_fields)]
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
			let (ey_v, roc_v) = (earnings_yield_value(&p.data), roic_value(&p.data));
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
			let (ey_v, roc_v) = (earnings_yield_value(&p.data), roic_value(&p.data));
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
			if d.eps.unwrap_or(0.0) > 0.0 && roe_value(&p.data).unwrap_or(0.0) > 0.0 {
				total += 1;
			}
			if reinvest_rate(d).map(|v| v > 0.8).unwrap_or(false) {
				total += 1;
			}
			if d.revenue.unwrap_or(0.0) > 1e6 && gross_margin_value(&p.data).unwrap_or(0.0) > 0.0 {
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
				asset_turnover(&p.data).map(|v| v > 0.7).unwrap_or(false),
				pe_ratio_value(&p.data).map(|v| v < 25.0).unwrap_or(false),
			];
			if munger.iter().filter(|&&x| x).count() >= 7 {
				total += 1;
			}
			let dupont: [bool; 4] = [
				roe_value(&p.data).map(|v| v > 0.15).unwrap_or(false),
				net_margin_value(&p.data).map(|v| v > 0.05).unwrap_or(false),
				asset_turnover(&p.data).map(|v| v > 0.7).unwrap_or(false),
				debt_to_equity_value(&p.data)
					.map(|v| 1.0 + v < 3.0)
					.unwrap_or(false),
			];
			if dupont.iter().filter(|&&x| x).count() >= 3 {
				total += 1;
			}
			let fisher: [bool; 8] = [
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
				asset_turnover(&p.data).map(|v| v > 0.5).unwrap_or(false),
				working_capital_turnover_value(&p.data)
					.map(|v| v > 4.0)
					.unwrap_or(false),
				roa_value(&p.data).map(|v| v > 0.08).unwrap_or(false),
				fcf_per_share_value(&p.data)
					.map(|v| v > 0.0)
					.unwrap_or(false),
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
			if pe_ratio_value(&p.data).map(|v| v < 15.0).unwrap_or(false)
				&& price_to_book_value(&p.data)
					.map(|v| v < 1.5)
					.unwrap_or(false)
				&& debt_to_equity_value(&p.data)
					.map(|v| v < 1.1)
					.unwrap_or(false)
				&& current_ratio_value(&p.data)
					.map(|v| v > 1.5)
					.unwrap_or(false)
			{
				total += 1;
			}
			if pe_ratio_value(&p.data).map(|v| v < 10.0).unwrap_or(false)
				&& debt_to_equity_value(&p.data)
					.map(|v| v < 1.0)
					.unwrap_or(false)
			{
				total += 1;
			}
			if price_to_book_value(&p.data)
				.map(|v| v < 1.2)
				.unwrap_or(false)
				&& debt_to_equity_value(&p.data)
					.map(|v| v < 0.5)
					.unwrap_or(false)
				&& d.net_income.map(|v| v > 0.0).unwrap_or(false)
			{
				total += 1;
			}
			if roe_value(&p.data).map(|v| v > 0.15).unwrap_or(false)
				&& d.revenue.unwrap_or(0.0) > 0.0
				&& pe_ratio_value(&p.data).map(|v| v < 25.0).unwrap_or(false)
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
			let qs = roe_value(&p.data).map(|r| r.min(1.0)).unwrap_or(0.0);
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
	serde_json::json!({"params":{"zScoreThreshold":3},"optimization_bounds":[]})
}
pub fn piotroski_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"piotroski-f-score","name":"Piotroski F-Score Quality","category":"fundamental","default_timeframes":["1d","1w"],"description":"Companies with F-Score >= 7"})
}
pub fn piotroski_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"fScoreThreshold":7},"optimization_bounds":[]})
}
pub fn magic_formula_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"magic-formula","name":"Magic Formula Investing","category":"fundamental","default_timeframes":["1d","1w"],"description":"High earnings yield and return on capital"})
}
pub fn magic_formula_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"earningsYieldThreshold":0.1,"returnOnCapitalThreshold":0.25},"optimization_bounds":[]})
}
pub fn joel_greenblatt_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id":"joel-greenblatt-magic-formula","name":"Joel Greenblatt Magic Formula","category":"fundamental","default_timeframes":["1d","1w"],"description":"Greenblatt's magic formula"})
}
pub fn joel_greenblatt_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params":{"earningsYieldThreshold":0.06,"returnOnCapitalThreshold":0.25},"optimization_bounds":[]})
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
	serde_json::json!({"params":{"valueWeight":0.4,"growthWeight":0.3,"qualityWeight":0.3,"threshold":0.6},"optimization_bounds":[]})
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
		altman_z_score_strategy_defaults => AltmanZScoreConfig,
		piotroski_strategy_defaults => PiotroskiConfig,
		magic_formula_strategy_defaults => MagicFormulaConfig,
		joel_greenblatt_strategy_defaults => JoelGreenblattConfig,
		growth_investing_suite_strategy_defaults => SuiteConfig,
		quality_investing_suite_strategy_defaults => SuiteConfig,
		value_investing_suite_strategy_defaults => SuiteConfig,
		multi_factor_suite_strategy_defaults => MultiFactorSuiteConfig,
	}
}
