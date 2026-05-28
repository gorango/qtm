use crate::types::configs::FibonacciRetracementConfig;
use crate::{StrategyError, StrategyResult};

/// Fibonacci Retracement Trend Strategy
///
/// Generates buy signals when price is above Fibonacci level
/// Generates sell signals when price is below Fibonacci level
///
/// @strategy_id fibonacciRetracement
/// @strategy_name Fibonacci Retracement Trend
/// @category trend
/// @default_timeframes 15m,1h,4h
pub fn fibonacci_retracement_strategy(
	closes: &[f64],
	config: Option<FibonacciRetracementConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(50);
	let fib_level = config.fib_level.unwrap_or(0.618);

	// Validate parameters
	if !(10..=200).contains(&period) {
		return Err(StrategyError::Validation(
			"Fibonacci period must be between 10 and 200".into(),
		));
	}
	if !(0.0..=1.0).contains(&fib_level) {
		return Err(StrategyError::Validation(
			"Fibonacci level must be between 0 and 1".into(),
		));
	}
	let data_len = closes.len();
	let min_periods = period as usize;
	if data_len < min_periods {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for Fibonacci Retracement strategy".into(),
		));
	}

	// Use constant Fibonacci level for now
	let fib_level_result = vec![fib_level; data_len];

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_periods {
			0 // Not enough data
		} else if closes[i] > fib_level_result[i] {
			1 // Buy signal: price above Fibonacci level
		} else if closes[i] < fib_level_result[i] {
			-1 // Sell signal: price below Fibonacci level
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get Fibonacci Retracement strategy metadata for registry
pub fn fibonacci_retracement_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "fibonacciRetracement",
		"name": "Fibonacci Retracement Trend",
		"category": "trend",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when price is above Fibonacci level and sell signals when price is below Fibonacci level"
	})
}

/// Get Fibonacci Retracement strategy default parameters
pub fn fibonacci_retracement_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 50,
			"fib_level": 0.618
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 20.0,
				"max": 100.0,
				"step": 5.0
			},
			{
				"param_name": "fib_level",
				"min": 0.236,
				"max": 0.786,
				"step": 0.05
			}
		]
	})
}
