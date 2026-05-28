use crate::types::configs::CorrelationPairConfig;
use crate::utils::signals::{crossed_over, crossed_under};
use serde_json;

/// Correlation Pair Strategy
///
/// Generates buy signals when correlation between two assets crosses over entry threshold
/// Generates sell signals when correlation falls crosses under exit threshold
///
/// @strategy_id correlation-pair-trading
/// @strategy_name Correlation Pair Trading Strategy
/// @category statistics
/// @default_timeframes 1h,4h,1d
pub fn correlation_pair_strategy(
	closes: &[f64],
	config: Option<CorrelationPairConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20);
	let entry_threshold = config.entry_threshold.unwrap_or(0.7);
	let exit_threshold = config.exit_threshold.unwrap_or(0.3);
	let second_closes = config.second_closes.unwrap_or_default();

	if !(2..=100).contains(&period) {
		return Err("Period must be between 2 and 100".to_string());
	}
	if !(0.0..=1.0).contains(&entry_threshold) || !(0.0..=1.0).contains(&exit_threshold) {
		return Err("Thresholds must be between 0 and 1".to_string());
	}

	let data_len = closes.len();
	if second_closes.len() != data_len {
		return Err("secondCloses must have the same length as closes".to_string());
	}
	if data_len < period as usize + 1 {
		return Err("Insufficient data for Correlation Pair strategy".to_string());
	}

	let second_closes_array: &[f64] = &second_closes;

	let corr_config = indicators_core::CorrelationConfig {
		period: Some(period),
	};
	let corr_values = indicators_core::correlation(closes, second_closes_array, Some(corr_config))?;

	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		if i < period as usize {
			signals.push(0);
			continue;
		}

		let corr = corr_values[i];
		if corr.is_nan() {
			signals.push(0);
			continue;
		}

		let signal = if crossed_over(&corr_values, entry_threshold, i as u32) {
			1 // Buy signal: correlation crosses over entry threshold
		} else if crossed_under(&corr_values, exit_threshold, i as u32) {
			-1 // Sell signal: correlation crosses under exit threshold
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}

pub fn correlation_pair_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "correlation-pair-trading",
		"name": "Correlation Pair Trading Strategy",
		"category": "statistics",
		"default_timeframes": ["1h", "4h", "1d"],
		"description": "Generates buy signals when correlation between two assets crosses over entry threshold and sell signals when correlation crosses under exit threshold"
	})
}

pub fn correlation_pair_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 20,
			"entryThreshold": 0.7,
			"exitThreshold": 0.3,
			"spreadMethod": "ratio",
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
				"param_name": "entryThreshold",
				"min": 0.1,
				"max": 0.9,
				"step": 0.05
			},
			{
				"param_name": "exitThreshold",
				"min": 0.1,
				"max": 0.9,
				"step": 0.05
			}
		]
	})
}
