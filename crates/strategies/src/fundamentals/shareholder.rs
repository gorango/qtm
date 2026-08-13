#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};

use factors_core::{dividend_payout_ratio_value, shareholder_yield_value, FundamentalPoint};

// ── Configs ──────────────────────────────────────────────

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DividendConfig {
	pub payout_ratio_max: Option<f64>,
	pub min_shareholder_yield: Option<f64>,
}

impl Default for DividendConfig {
	fn default() -> Self {
		Self {
			payout_ratio_max: Some(0.6),
			min_shareholder_yield: Some(0.03),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DividendGrowthConsistencyConfig {
	pub min_criteria_met: Option<u32>,
	pub min_years_consistent: Option<f64>,
	pub min_avg_growth_rate: Option<f64>,
}

impl Default for DividendGrowthConsistencyConfig {
	fn default() -> Self {
		Self {
			min_criteria_met: Some(2),
			min_years_consistent: Some(5.0),
			min_avg_growth_rate: Some(0.05),
		}
	}
}

// ── Strategies ───────────────────────────────────────────

/// Dividend-focused: sustainable payout + shareholder yield
pub fn dividend_strategy(points: Vec<FundamentalPoint>, config: Option<DividendConfig>) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let payout_max = cfg.payout_ratio_max.unwrap_or(0.6);
	let min_yield = cfg.min_shareholder_yield.unwrap_or(0.03);

	points
		.iter()
		.map(|p| {
			let payout_ok = dividend_payout_ratio_value(&p.data)
				.map(|v| v < payout_max)
				.unwrap_or(false);
			let yield_ok = shareholder_yield_value(&p.data)
				.map(|v| v > min_yield)
				.unwrap_or(false);
			if payout_ok && yield_ok {
				1
			} else {
				0
			}
		})
		.collect()
}

/// Dividend Growth Consistency: 3 criteria
pub fn dividend_growth_consistency_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<DividendGrowthConsistencyConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_growth = cfg.min_avg_growth_rate.unwrap_or(0.05);
	let min_met = cfg.min_criteria_met.unwrap_or(2) as usize;

	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let mut met = 0usize;
			if d.dividend_growth_5y.map(|v| v > 0.0).unwrap_or(false)
				|| d.dividend_growth_3y.map(|v| v > 0.0).unwrap_or(false)
			{
				met += 1;
			}
			if d.dividend_growth_5y
				.map(|v| v > min_growth)
				.unwrap_or(false)
				|| d.dividend_growth_3y
					.map(|v| v > min_growth)
					.unwrap_or(false)
			{
				met += 1;
			}
			if d.dividends_per_share.map(|v| v > 0.0).unwrap_or(false) {
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

pub fn dividend_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id": "dividend-focused","name": "Dividend Focused Fundamental","category": "fundamental","default_timeframes": ["1d","1w"],"description": "Sustainable dividends and attractive shareholder yields"})
}
pub fn dividend_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params": {"payoutRatioMax": 0.6,"minShareholderYield": 0.03},"optimization_bounds": [{"param_name": "payoutRatioMax","min": 0,"max": 1,"step": 0.05},{"param_name": "minShareholderYield","min": 0.01,"max": 0.2,"step": 0.005}]})
}

pub fn dividend_growth_consistency_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id": "dividend-growth-consistency","name": "Dividend Growth Consistency","category": "fundamental","default_timeframes": ["1d","1w"],"description": "Consistent dividend payments with growing payouts"})
}
pub fn dividend_growth_consistency_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params": {"minCriteriaMet": 2,"minYearsConsistent": 5,"minAvgGrowthRate": 0.05},"optimization_bounds": [{"param_name": "minCriteriaMet","min": 1,"max": 3,"step": 1},{"param_name": "minYearsConsistent","min": 1,"max": 20,"step": 1},{"param_name": "minAvgGrowthRate","min": 0,"max": 0.5,"step": 0.01}]})
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
		dividend_strategy_defaults => DividendConfig,
		dividend_growth_consistency_strategy_defaults => DividendGrowthConsistencyConfig,
	}
}
