use crate::types::configs::MACDConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};

/// MACD Trend Strategy
///
/// Generates buy signals when MACD line crosses above signal line
/// Generates sell signals when MACD line crosses below signal line
///
/// @strategy_id macd
/// @strategy_name MACD Trend
/// @category trend
/// @default_timeframes 15m,1h,4h
pub fn macd_strategy(closes: &[f64], config: Option<MACDConfig>) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let fast_period = config.fast_period.unwrap_or(12);
	let slow_period = config.slow_period.unwrap_or(26);
	let signal_period = config.signal_period.unwrap_or(9);

	// Validate parameters
	if !(2..=100).contains(&fast_period) {
		return Err("MACD fast period must be between 2 and 100".to_string());
	}
	if !(2..=200).contains(&slow_period) {
		return Err("MACD slow period must be between 2 and 200".to_string());
	}
	if !(2..=50).contains(&signal_period) {
		return Err("MACD signal period must be between 2 and 50".to_string());
	}
	let data_len = closes.len();
	let min_periods = (slow_period + signal_period) as usize;
	if data_len < min_periods {
		return Err("Insufficient data for MACD strategy".to_string());
	}

	// Calculate MACD
	let macd_config = indicators_core::MACDConfig {
		fast_period: Some(fast_period),
		slow_period: Some(slow_period),
		signal_period: Some(signal_period),
	};
	let macd_result = indicators_core::macd(closes, Some(macd_config))?;

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_periods {
			0 // Not enough data
		} else if crossed_over_series(&macd_result.macd, &macd_result.signal, i as u32) {
			1 // Buy signal
		} else if crossed_under_series(&macd_result.macd, &macd_result.signal, i as u32) {
			-1 // Sell signal
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get MACD strategy metadata for registry
pub fn macd_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "macd",
		"name": "MACD Trend",
		"category": "trend",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when MACD line crosses above signal line and sell signals when MACD line crosses below signal line"
	})
}

/// Get MACD strategy default parameters
pub fn macd_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"fast_period": 12,
			"slow_period": 26,
			"signal_period": 9
		},
		"optimization_bounds": [
			{
				"param_name": "fast_period",
				"min": 5.0,
				"max": 20.0,
				"step": 1.0
			},
			{
				"param_name": "slow_period",
				"min": 20.0,
				"max": 50.0,
				"step": 1.0
			},
			{
				"param_name": "signal_period",
				"min": 5.0,
				"max": 20.0,
				"step": 1.0
			}
		]
	})
}
