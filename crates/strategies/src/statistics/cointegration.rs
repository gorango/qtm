use crate::types::configs::CointegrationConfig;
use crate::utils::signals::{crossed_over, crossed_under};
use serde_json;

/// Cointegration Strategy
///
/// Generates buy signals when spread z-score crosses under negative entry threshold
/// Generates sell signals when spread z-score crosses over entry threshold
///
/// @strategy_id cointegration-pair-trading
/// @strategy_name Cointegration Pair Trading Strategy
/// @category statistics
/// @default_timeframes 1h,4h,1d
pub fn cointegration_strategy(
	closes: &[f64],
	config: Option<CointegrationConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20);
	let beta_period = config.beta_period.unwrap_or(60);
	let entry_threshold = config.entry_threshold.unwrap_or(2.0);
	let second_closes = config.second_closes.unwrap_or_default();

	if !(2..=100).contains(&period) {
		return Err("Period must be between 2 and 100".to_string());
	}
	if !(2..=200).contains(&beta_period) {
		return Err("Beta period must be between 2 and 200".to_string());
	}
	if entry_threshold <= 0.0 {
		return Err("Entry threshold must be positive".to_string());
	}

	let data_len = closes.len();
	if second_closes.len() != data_len {
		return Err("secondCloses must have the same length as closes".to_string());
	}

	let min_required = period.max(beta_period) as usize;
	if data_len < min_required {
		return Err("Insufficient data for Cointegration strategy".to_string());
	}

	let second_closes_array = &second_closes;

	let coint_config = indicators_core::CointegrationConfig {
		period: Some(period),
		beta_period: Some(beta_period),
	};
	let z_scores = indicators_core::cointegration(closes, second_closes_array, Some(coint_config))?;

	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		if i < min_required {
			signals.push(0);
			continue;
		}

		let z_score = z_scores[i];
		if z_score.is_nan() {
			signals.push(0);
			continue;
		}

		let signal = if crossed_under(&z_scores, -entry_threshold, i as u32) {
			1 // Buy signal: z-score crosses under negative entry threshold
		} else if crossed_over(&z_scores, entry_threshold, i as u32) {
			-1 // Sell signal: z-score crosses over entry threshold
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}

pub fn cointegration_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "cointegration-pair-trading",
		"name": "Cointegration Pair Trading Strategy",
		"category": "statistics",
		"default_timeframes": ["1h", "4h", "1d"],
		"description": "Generates buy signals when spread z-score crosses under negative entry threshold and sell signals when spread z-score crosses over entry threshold"
	})
}

pub fn cointegration_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 20,
			"betaPeriod": 60,
			"entryThreshold": 2.0,
			"secondCloses": []
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 10.0,
				"max": 50.0,
				"step": 1.0
			},
			{
				"param_name": "betaPeriod",
				"min": 20.0,
				"max": 100.0,
				"step": 5.0
			},
			{
				"param_name": "entryThreshold",
				"min": 1.0,
				"max": 3.0,
				"step": 0.1
			}
		]
	})
}
