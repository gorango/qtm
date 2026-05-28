use crate::types::configs::MaCrossoverConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};

/// Moving Average Crossover Strategy
///
/// Generates buy signals when fast MA crosses above slow MA
/// Generates sell signals when fast MA crosses below slow MA
///
/// @strategy_id ma-crossover
/// @strategy_name Moving Average Crossover Information
/// @category trend
/// @default_timeframes 1h,4h,1d
pub fn ma_crossover_strategy(
	closes: &[f64],
	config: Option<MaCrossoverConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let fast_period = config.fast_period.unwrap_or(5);
	let slow_period = config.slow_period.unwrap_or(20);

	// Validate parameters
	if !(2..=100).contains(&fast_period) {
		return Err("Fast MA period must be between 2 and 100".to_string());
	}
	if !(2..=200).contains(&slow_period) {
		return Err("Slow MA period must be between 2 and 200".to_string());
	}
	if closes.len() < slow_period as usize {
		return Err("Insufficient data for MA Crossover strategy".to_string());
	}

	// Calculate moving averages using the NAPI functions
	let closes_vec: Vec<f64> = closes.to_vec();
	let fast_ma_result = indicators_core::sma(&closes_vec, Some(fast_period))?;
	let slow_ma_result = indicators_core::sma(&closes_vec, Some(slow_period))?;

	// Generate signals
	let mut signals = Vec::with_capacity(closes.len());

	for i in 0..closes.len() {
		let signal = if i < slow_period as usize {
			0 // Not enough data
		} else if crossed_over_series(&fast_ma_result, &slow_ma_result, i as u32) {
			1 // Buy signal
		} else if crossed_under_series(&fast_ma_result, &slow_ma_result, i as u32) {
			-1 // Sell signal
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get MA Crossover strategy metadata for registry
pub fn ma_crossover_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "ma-crossover",
		"name": "Moving Average Crossover Information",
		"category": "trend",
		"default_timeframes": ["1h", "4h", "1d"],
		"description": "Generates buy signals when fast MA crosses above slow MA and sell signals when fast MA crosses below slow MA"
	})
}

/// Get MA Crossover strategy default parameters
pub fn ma_crossover_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"fast_period": 5,
			"slow_period": 20
		},
		"optimization_bounds": [
			{
				"param_name": "fast_period",
				"min": 3.0,
				"max": 10.0,
				"step": 1.0
			},
			{
				"param_name": "slow_period",
				"min": 10.0,
				"max": 50.0,
				"step": 1.0
			}
		]
	})
}
