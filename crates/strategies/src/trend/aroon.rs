use crate::types::configs::AroonConfig;
use crate::utils::signals::{crossed_over, crossed_under};
use serde_json;

/// Aroon Trend Strategy
///
/// Generates buy signals when Aroon Up crosses over overbought level
/// Generates sell signals when Aroon Down crosses under oversold level
///
/// @strategy_id aroon
/// @strategy_name Aroon Trend
/// @category trend
/// @default_timeframes 1h,4h,1d
pub fn aroon_strategy(
	highs: &[f64],
	lows: &[f64],
	config: Option<AroonConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14);
	let overbought = config.overbought.unwrap_or(70.0);
	let oversold = config.oversold.unwrap_or(30.0);

	// Validate parameters
	if !(2..=100).contains(&period) {
		return Err("Aroon period must be between 2 and 100".to_string());
	}
	let data_len = highs.len();
	let min_periods = period as usize;
	if data_len < min_periods {
		return Err("Insufficient data for Aroon strategy".to_string());
	}

	// Calculate Aroon
	let aroon_config = indicators_core::AroonConfig {
		period: Some(period),
	};
	let aroon_result = indicators_core::aroon(highs, lows, Some(aroon_config))?;

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_periods {
			0 // Not enough data
		} else if crossed_over(&aroon_result.up, overbought, i as u32) {
			1 // Buy signal: Aroon Up crosses over overbought
		} else if crossed_under(&aroon_result.down, oversold, i as u32) {
			-1 // Sell signal: Aroon Down crosses under oversold
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get Aroon strategy metadata for registry
pub fn aroon_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "aroon",
		"name": "Aroon Trend",
		"category": "trend",
		"default_timeframes": ["1h", "4h", "1d"],
		"description": "Generates buy signals when Aroon Up crosses over overbought level and sell signals when Aroon Down crosses under oversold level"
	})
}

/// Get Aroon strategy default parameters
pub fn aroon_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 14,
			"overbought": 70.0,
			"oversold": 30.0
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 5.0,
				"max": 30.0,
				"step": 1.0
			},
			{
				"param_name": "overbought",
				"min": 60.0,
				"max": 90.0,
				"step": 5.0
			},
			{
				"param_name": "oversold",
				"min": 10.0,
				"max": 40.0,
				"step": 5.0
			}
		]
	})
}
