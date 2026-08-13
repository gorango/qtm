#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};

use factors_core::{Bar, FactorPoint};

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MultiFactorConfig {
	pub rebalance_interval: Option<u32>,
	pub top_percentile: Option<f64>,
	pub value_weight: Option<f64>,
	pub quality_weight: Option<f64>,
	pub momentum_weight: Option<f64>,
}

impl Default for MultiFactorConfig {
	fn default() -> Self {
		Self {
			rebalance_interval: Some(20),
			top_percentile: Some(0.2),
			value_weight: Some(1.0),
			quality_weight: Some(1.0),
			momentum_weight: Some(1.0),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RiskParityConfig {
	pub volatility_period: Option<u32>,
	pub risk_target: Option<f64>,
}

impl Default for RiskParityConfig {
	fn default() -> Self {
		Self {
			volatility_period: Some(252),
			risk_target: Some(0.1),
		}
	}
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DualMomentumConfig {
	pub momentum_period: Option<u32>,
	pub risk_free_rate: Option<f64>,
}

impl Default for DualMomentumConfig {
	fn default() -> Self {
		Self {
			momentum_period: Some(252),
			risk_free_rate: Some(0.02),
		}
	}
}

fn z_score_normalize(values: &[f64]) -> Vec<f64> {
	let n = values.len();
	if n == 0 {
		return Vec::new();
	}
	let valid: Vec<f64> = values.iter().filter(|v| v.is_finite()).copied().collect();
	if valid.len() < 2 {
		return vec![0.0; n];
	}
	let mean = valid.iter().sum::<f64>() / valid.len() as f64;
	let variance = valid.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (valid.len() - 1) as f64;
	let stdev = variance.sqrt();
	if stdev == 0.0 {
		return values.iter().map(|_| 0.0).collect();
	}
	values
		.iter()
		.map(|v| {
			if v.is_finite() {
				(v - mean) / stdev
			} else {
				0.0
			}
		})
		.collect()
}

/// Multi Factor
///
/// Generates buy/sell signals combining quantitative and fundamental factors.
pub fn multi_factor_strategy(
	value_factors: Vec<FactorPoint>,
	quality_factors: Vec<FactorPoint>,
	momentum_factors: Vec<FactorPoint>,
	prices: Vec<Bar>,
	config: Option<MultiFactorConfig>,
) -> Vec<f64> {
	let cfg = config.unwrap_or_default();
	let top_percentile = cfg.top_percentile.unwrap_or(0.2);
	let value_weight = cfg.value_weight.unwrap_or(1.0);
	let quality_weight = cfg.quality_weight.unwrap_or(1.0);
	let momentum_weight = cfg.momentum_weight.unwrap_or(1.0);

	let n = value_factors.len();
	if n == 0 || quality_factors.len() != n || momentum_factors.len() != n || prices.len() != n {
		return Vec::new();
	}

	let raw_value: Vec<f64> = value_factors.iter().map(|fp| fp.value).collect();
	let raw_quality: Vec<f64> = quality_factors.iter().map(|fp| fp.value).collect();
	let raw_momentum: Vec<f64> = momentum_factors.iter().map(|fp| fp.value).collect();

	let z_value = z_score_normalize(&raw_value);
	let z_quality = z_score_normalize(&raw_quality);
	let z_momentum = z_score_normalize(&raw_momentum);

	let mut scores: Vec<(usize, f64)> = (0..n)
		.map(|i| {
			let composite = z_value[i] * value_weight
				+ z_quality[i] * quality_weight
				+ z_momentum[i] * momentum_weight;
			(i, composite)
		})
		.collect();

	scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

	let top_n = std::cmp::max(1, (n as f64 * top_percentile).floor() as usize);
	let mut selected = vec![false; n];
	for i in 0..top_n {
		if i < scores.len() {
			selected[scores[i].0] = true;
		}
	}

	let weight = 1.0 / top_n as f64;
	(0..n)
		.map(|i| if selected[i] { weight } else { 0.0 })
		.collect()
}

/// Risk Parity
///
/// Generates buy/sell signals combining quantitative and fundamental factors.
pub fn risk_parity_strategy(closes: Vec<Vec<f64>>, config: Option<RiskParityConfig>) -> Vec<f64> {
	let cfg = config.unwrap_or_default();
	let vol_period = cfg.volatility_period.unwrap_or(252) as usize;

	let n = closes.len();
	if n == 0 {
		return Vec::new();
	}

	let mut vols: Vec<f64> = Vec::with_capacity(n);
	for prices in &closes {
		if prices.len() < vol_period + 1 {
			vols.push(0.0);
			continue;
		}
		let start = prices.len() - 1 - vol_period;
		let mut returns = Vec::with_capacity(vol_period);
		for i in (start + 1)..prices.len() {
			let prev = prices[i - 1];
			if prev != 0.0 {
				returns.push((prices[i] - prev) / prev);
			}
		}
		if returns.len() < 2 {
			vols.push(0.0);
			continue;
		}
		let mean = returns.iter().sum::<f64>() / returns.len() as f64;
		let variance =
			returns.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / returns.len() as f64;
		vols.push(variance.sqrt());
	}

	let total_inv_vol: f64 = vols.iter().filter(|v| **v > 0.0).map(|v| 1.0 / v).sum();
	if total_inv_vol <= 0.0 {
		return vec![0.0; n];
	}

	vols.iter()
		.map(|v| {
			if *v > 0.0 {
				(1.0 / v) / total_inv_vol
			} else {
				0.0
			}
		})
		.collect()
}

/// Dual Momentum
///
/// Generates buy/sell signals combining quantitative and fundamental factors.
pub fn dual_momentum_strategy(
	closes: Vec<Vec<f64>>,
	config: Option<DualMomentumConfig>,
) -> Vec<f64> {
	let cfg = config.unwrap_or_default();
	let momentum_period = cfg.momentum_period.unwrap_or(252) as usize;

	let n = closes.len();
	if n == 0 {
		return Vec::new();
	}

	let mut returns: Vec<(usize, f64)> = Vec::with_capacity(n);
	for (i, prices) in closes.iter().enumerate() {
		if prices.len() < momentum_period + 1 {
			continue;
		}
		let current = prices[prices.len() - 1];
		let past = prices[prices.len() - 1 - momentum_period];
		if past != 0.0 {
			returns.push((i, (current - past) / past));
		}
	}

	if returns.is_empty() {
		return vec![0.0; n];
	}

	returns.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
	let best_idx = returns[0].0;
	let best_return = returns[0].1;

	if best_return <= 0.0 {
		return vec![0.0; n];
	}

	let mut weights = vec![0.0; n];
	weights[best_idx] = 1.0;
	weights
}

pub fn multi_factor_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "multi-factor-portfolio",
		"name": "Multi-Factor Portfolio Strategy",
		"category": "quantamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Z-score normalizes value/quality/momentum factors, composites with weights, selects top percentile, equal-weight allocation"
	})
}

pub fn risk_parity_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "risk-parity",
		"name": "Risk Parity Portfolio Strategy",
		"category": "quantamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Allocates weights inversely proportional to each asset's historical volatility"
	})
}

pub fn dual_momentum_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "dual-momentum",
		"name": "Dual Momentum Strategy",
		"category": "quantamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Allocates 100% to the best performing asset with positive absolute momentum"
	})
}

pub fn multi_factor_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"rebalanceInterval": 20,
			"topPercentile": 0.2,
			"valueWeight": 1.0,
			"qualityWeight": 1.0,
			"momentumWeight": 1.0
		},
		"optimization_bounds": [
			{"param_name": "topPercentile", "min": 0.05, "max": 0.5, "step": 0.05},
			{"param_name": "valueWeight", "min": 0.0, "max": 3.0, "step": 0.5},
			{"param_name": "qualityWeight", "min": 0.0, "max": 3.0, "step": 0.5},
			{"param_name": "momentumWeight", "min": 0.0, "max": 3.0, "step": 0.5}
		]
	})
}

pub fn risk_parity_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"volatilityPeriod": 252,
			"riskTarget": 0.1
		},
		"optimization_bounds": [
			{"param_name": "volatilityPeriod", "min": 10.0, "max": 500.0, "step": 10.0},
			{"param_name": "riskTarget", "min": 0.02, "max": 0.3, "step": 0.01}
		]
	})
}

pub fn dual_momentum_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"momentumPeriod": 252,
			"riskFreeRate": 0.02
		},
		"optimization_bounds": [
			{"param_name": "momentumPeriod", "min": 20.0, "max": 500.0, "step": 10.0}
		]
	})
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
		multi_factor_strategy_defaults => MultiFactorConfig,
		risk_parity_strategy_defaults => RiskParityConfig,
		dual_momentum_strategy_defaults => DualMomentumConfig,
	}
}
