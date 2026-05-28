use crate::types::configs::CorrelationReversionConfig;
use serde_json;

/// Correlation Reversion Strategy
///
/// Generates buy signals when correlation drops below reversion threshold (mean reversion entry)
/// Generates sell signals when correlation exceeds 1.5x reversion threshold (exit)
///
/// @strategy_id correlation-mean-reversion
/// @strategy_name Correlation Mean Reversion Strategy
/// @category statistics
/// @default_timeframes 1h,4h,1d
pub fn correlation_reversion_strategy(
	closes: &[f64],
	config: Option<CorrelationReversionConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20);
	let reversion_threshold = config.reversion_threshold.unwrap_or(0.2);
	let second_closes = config.second_closes.unwrap_or_default();

	if !(2..=100).contains(&period) {
		return Err("Period must be between 2 and 100".to_string());
	}
	if !(0.0..=1.0).contains(&reversion_threshold) {
		return Err("Reversion threshold must be between 0 and 1".to_string());
	}

	let data_len = closes.len();
	if second_closes.len() != data_len {
		return Err("secondCloses must have the same length as closes".to_string());
	}
	if data_len < period as usize + 1 {
		return Err("Insufficient data for Correlation Reversion strategy".to_string());
	}

	let second_closes_array: &[f64] = &second_closes;

	let corr_config = indicators_core::CorrelationConfig {
		period: Some(period),
	};
	let corr_values = indicators_core::correlation(closes, second_closes_array, Some(corr_config))?;

	let mut signals = Vec::with_capacity(data_len);

	for (i, &corr) in corr_values.iter().enumerate().take(data_len) {
		if i < period as usize {
			signals.push(0);
			continue;
		}

		if corr.is_nan() {
			signals.push(0);
			continue;
		}

		let signal = if corr < reversion_threshold {
			1
		} else if corr > reversion_threshold * 1.5 {
			-1
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}

pub fn correlation_reversion_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "correlation-mean-reversion",
		"name": "Correlation Mean Reversion Strategy",
		"category": "statistics",
		"default_timeframes": ["1h", "4h", "1d"],
		"description": "Generates buy signals when correlation drops below reversion threshold and sell signals when correlation exceeds 1.5x reversion threshold"
	})
}

pub fn correlation_reversion_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 20,
			"reversionThreshold": 0.2,
			"holdingPeriod": 5,
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
				"param_name": "reversionThreshold",
				"min": 0.1,
				"max": 0.5,
				"step": 0.05
			},
			{
				"param_name": "holdingPeriod",
				"min": 1.0,
				"max": 20.0,
				"step": 1.0
			}
		]
	})
}
