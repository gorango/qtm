use crate::types::configs::StandardDeviationConfig;

/// Standard Deviation
///
/// Generates buy/sell signals based on volatility channel breakouts and mean reversion.
pub fn standard_deviation_strategy(
	closes: &[f64],
	config: Option<StandardDeviationConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14);
	let threshold = config.threshold.unwrap_or(2.0);

	if !(2..=100).contains(&period) {
		return Err("Standard Deviation period must be between 2 and 100".to_string());
	}
	if !(0.1..=10.0).contains(&threshold) {
		return Err("Standard Deviation threshold must be between 0.1 and 10.0".to_string());
	}

	let data_len = closes.len();
	let mstd_config = indicators_core::MSTDConfig {
		period: Some(period),
	};
	let std = indicators_core::moving_standard_deviation(closes, Some(mstd_config))?;
	let mut signals = Vec::with_capacity(data_len);

	for (i, &std_value) in std.iter().enumerate().take(data_len) {
		let signal = if i < period as usize {
			0
		} else if std_value < threshold {
			1
		} else if std_value > threshold {
			-1
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}

pub fn standard_deviation_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "standardDeviation",
		"name": "Standard Deviation Strategy",
		"category": "volatility",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when standard deviation is below threshold and sell signals when it exceeds threshold"
	})
}

pub fn standard_deviation_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 14,
			"threshold": 2.0
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 5.0,
				"max": 50.0,
				"step": 1.0
			},
			{
				"param_name": "threshold",
				"min": 1.0,
				"max": 5.0,
				"step": 0.1
			}
		]
	})
}
