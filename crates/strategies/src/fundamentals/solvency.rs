#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};

use factors_core::{debt_to_assets_value, interest_coverage_value, FundamentalPoint};

// ── Configs ──────────────────────────────────────────────

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
			let debt_ok = debt_to_assets_value(&p.data)
				.map(|v| v < max_da)
				.unwrap_or(false);
			let interest_ok = interest_coverage_value(&p.data)
				.map(|v| v > min_ic)
				.unwrap_or(false);
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
	serde_json::json!({"params": {"maxDebtToAssets": 0.5,"minInterestCoverage": 3},"optimization_bounds": [{"param_name": "maxDebtToAssets","min": 0,"max": 1,"step": 0.05},{"param_name": "minInterestCoverage","min": 1,"max": 10,"step": 0.5}]})
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
		solvency_strategy_defaults => SolvencyConfig,
	}
}
