#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};

use factors_core::{FundamentalPoint, FundamentalPointData};

fn debt_to_assets(d: &FundamentalPointData) -> Option<f64> {
	let a = d.total_assets?;
	if a == 0.0 {
		None
	} else {
		Some(d.total_debt? / a)
	}
}
fn interest_coverage(d: &FundamentalPointData) -> Option<f64> {
	let i = d.interest_expense?;
	if i == 0.0 {
		None
	} else {
		Some(d.operating_income? / i)
	}
}

// ── Configs ──────────────────────────────────────────────

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SolvencyConfig {
	pub max_debt_to_assets: Option<f64>,
	pub min_interest_coverage: Option<f64>,
}

impl Default for SolvencyConfig {
	fn default() -> Self {
		Self {
			max_debt_to_assets: Some(0.5),
			min_interest_coverage: Some(3.0),
		}
	}
}

// ── Strategies ───────────────────────────────────────────

/// Solvency: debt-to-assets below threshold + interest coverage above threshold
pub fn solvency_strategy(points: Vec<FundamentalPoint>, config: Option<SolvencyConfig>) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let max_da = cfg.max_debt_to_assets.unwrap_or(0.5);
	let min_ic = cfg.min_interest_coverage.unwrap_or(3.0);

	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let debt_ok = debt_to_assets(d).map(|v| v < max_da).unwrap_or(false);
			let interest_ok = interest_coverage(d).map(|v| v > min_ic).unwrap_or(false);
			if debt_ok && interest_ok {
				1
			} else {
				0
			}
		})
		.collect()
}

// ── Metadata ─────────────────────────────────────────────

pub fn solvency_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id": "solvency-focused","name": "Solvency Focused Fundamental","category": "fundamental","default_timeframes": ["1d","1w"],"description": "Strong balance sheets: low debt-to-assets, high interest coverage"})
}
pub fn solvency_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params": {"max_debt_to_assets": 0.5,"min_interest_coverage": 3},"optimization_bounds": [{"param_name": "max_debt_to_assets","min": 0,"max": 1,"step": 0.05},{"param_name": "min_interest_coverage","min": 1,"max": 10,"step": 0.5}]})
}
