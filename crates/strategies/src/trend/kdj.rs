use crate::types::configs::KdjConfig;
use crate::utils::signals::{crossed_over, crossed_under};
use indicators_core::{StochConfig, StochResult};

/// KDJ Trend Strategy
///
/// Generates buy signals when J crosses above oversold level
/// Generates sell signals when J crosses below overbought level
///
/// @strategy_id kdj
/// @strategy_name KDJ Trend
/// @category trend
/// @default_timeframes 15m,1h,4h
pub fn kdj_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<KdjConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14);
	let period1 = config.period1.unwrap_or(3);
	let period2 = config.period2.unwrap_or(3);
	let overbought = config.overbought.unwrap_or(80.0);
	let oversold = config.oversold.unwrap_or(20.0);

	// Validate parameters
	if !(5..=100).contains(&period) {
		return Err("KDJ period must be between 5 and 100".to_string());
	}
	if !(2..=20).contains(&period1) {
		return Err("KDJ period1 must be between 2 and 20".to_string());
	}
	if !(2..=20).contains(&period2) {
		return Err("KDJ period2 must be between 2 and 20".to_string());
	}
	if oversold >= overbought {
		return Err("KDJ oversold must be less than overbought".to_string());
	}
	let data_len = closes.len();
	let min_periods = (period + period1) as usize;
	if data_len < min_periods {
		return Err("Insufficient data for KDJ strategy".to_string());
	}

	// Calculate KDJ
	let stoch_result: StochResult = indicators_core::stochastic_oscillator(
		highs,
		lows,
		closes,
		Some(StochConfig {
			k_period: Some(period),
			d_period: Some(period1),
		}),
	);
	let k = stoch_result.k;
	let d = stoch_result.d;
	let j: Vec<f64> = k
		.iter()
		.zip(d.iter())
		.map(|(&k_val, &d_val)| 3.0 * k_val - 2.0 * d_val)
		.collect();

	// Generate signals based on J line
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_periods {
			0
		} else {
			// Use crossovers to generate pulse signals
			if crossed_over(&j, oversold, i as u32) {
				1 // Buy: J crosses UP over oversold
			} else if crossed_under(&j, overbought, i as u32) {
				-1 // Sell: J crosses DOWN under overbought
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get KDJ strategy metadata for registry
pub fn kdj_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "kdj",
		"name": "KDJ Trend",
		"category": "trend",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when J crosses above oversold level and sell signals when J crosses below overbought level"
	})
}

/// Get KDJ strategy default parameters
pub fn kdj_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 14,
			"period1": 3,
			"period2": 3,
			"overbought": 80.0,
			"oversold": 20.0
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 5.0,
				"max": 30.0,
				"step": 1.0
			},
			{
				"param_name": "period1",
				"min": 2.0,
				"max": 10.0,
				"step": 1.0
			},
			{
				"param_name": "period2",
				"min": 2.0,
				"max": 10.0,
				"step": 1.0
			},
			{
				"param_name": "overbought",
				"min": 70.0,
				"max": 90.0,
				"step": 1.0
			},
			{
				"param_name": "oversold",
				"min": 10.0,
				"max": 30.0,
				"step": 1.0
			}
		]
	})
}
