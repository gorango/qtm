use crate::types::configs::SuperTrendConfig;

/// Super Trend Trend Strategy
///
/// Generates signals on trend direction changes
/// Buy when trend changes to up, sell when trend changes to down
///
/// @strategy_id superTrend
/// @strategy_name Super Trend Trend
/// @category trend
/// @default_timeframes 15m,1h,4h
pub fn super_trend_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<SuperTrendConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(3);
	let multiplier = config.multiplier.unwrap_or(3.0);

	// Validate parameters
	if !(2..=100).contains(&period) {
		return Err("SuperTrend period must be between 2 and 100".to_string());
	}
	let data_len = highs.len();
	let min_periods = period as usize;
	if data_len < min_periods {
		return Err("Insufficient data for SuperTrend strategy".to_string());
	}

	// Calculate SuperTrend
	let supertrend_result =
		indicators_core::super_trend(highs, lows, closes, Some(period), Some(multiplier))?;

	// Generate signals on direction changes
	let mut signals = Vec::with_capacity(data_len);
	let mut prev_direction = 0;

	for i in 0..data_len {
		let current_direction = supertrend_result.direction[i];
		let signal = if i < min_periods {
			0 // Not enough data
		} else if prev_direction == -1 && current_direction == 1 {
			1 // Buy signal: trend changed to up
		} else if prev_direction == 1 && current_direction == -1 {
			-1 // Sell signal: trend changed to down
		} else {
			0 // Hold: no change
		};
		signals.push(signal);
		prev_direction = current_direction;
	}

	Ok(signals)
}

/// Get Super Trend strategy metadata for registry
pub fn super_trend_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "superTrend",
		"name": "Super Trend Trend",
		"category": "trend",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when trend changes to up and sell signals when trend changes to down"
	})
}

/// Get Super Trend strategy default parameters
pub fn super_trend_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 3,
			"multiplier": 3.0
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 2.0,
				"max": 20.0,
				"step": 1.0
			},
			{
				"param_name": "multiplier",
				"min": 1.0,
				"max": 5.0,
				"step": 0.5
			}
		]
	})
}
