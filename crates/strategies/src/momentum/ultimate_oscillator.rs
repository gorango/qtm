use crate::types::configs::UltimateOscillatorConfig;
use crate::utils::signals::{crossed_over, crossed_under};

/// Ultimate Oscillator Momentum Strategy
///
/// Generates buy signals when UO crosses above oversold level
/// Generates sell signals when UO crosses below overbought level
///
/// @strategy_id ultimateOscillator
/// @strategy_name Ultimate Oscillator Momentum Strategy
/// @category momentum
/// @default_timeframes 15m,1h,4h
pub fn ultimate_oscillator_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<UltimateOscillatorConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let period1 = config.period1.unwrap_or(7);
	let period2 = config.period2.unwrap_or(14);
	let period3 = config.period3.unwrap_or(28);
	let oversold = config.oversold.unwrap_or(30.0);
	let overbought = config.overbought.unwrap_or(70.0);

	// Validate parameters
	if !(2..=100).contains(&period1)
		|| !(2..=100).contains(&period2)
		|| !(2..=100).contains(&period3)
	{
		return Err("Ultimate Oscillator periods must be between 2 and 100".to_string());
	}
	if period1 >= period2 || period2 >= period3 {
		return Err("Ultimate Oscillator periods must be in ascending order".to_string());
	}
	if !(0.0..=100.0).contains(&oversold) || !(0.0..=100.0).contains(&overbought) {
		return Err("Ultimate Oscillator thresholds must be between 0 and 100".to_string());
	}
	if oversold >= overbought {
		return Err("Ultimate Oscillator oversold must be less than overbought".to_string());
	}

	let min_period = period1.max(period2).max(period3);
	let data_len = highs.len();
	if data_len < (min_period as usize) + 1 {
		return Err("Insufficient data for Ultimate Oscillator strategy".to_string());
	}
	if lows.len() != data_len || closes.len() != data_len {
		return Err("All price arrays must have the same length".to_string());
	}

	// Calculate Ultimate Oscillator values
	let uo_config = indicators_core::UltimateOscillatorConfig {
		period1: Some(period1),
		period2: Some(period2),
		period3: Some(period3),
	};
	let uo_values = indicators_core::ultimate_oscillator(highs, lows, closes, Some(uo_config));

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if crossed_over(&uo_values, oversold, i as u32) {
			1 // Buy signal
		} else if crossed_under(&uo_values, overbought, i as u32) {
			-1 // Sell signal
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get Ultimate Oscillator strategy metadata for registry
pub fn ultimate_oscillator_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "ultimateOscillator",
		"name": "Ultimate Oscillator Momentum Strategy",
		"category": "momentum",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when UO crosses above oversold level and sell signals when UO crosses below overbought level"
	})
}

/// Get Ultimate Oscillator strategy default parameters
pub fn ultimate_oscillator_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period1": 7,
			"period2": 14,
			"period3": 28,
			"oversold": 30.0,
			"overbought": 70.0
		},
		"optimization_bounds": [
			{
				"param_name": "period1",
				"min": 5.0,
				"max": 10.0,
				"step": 1.0
			},
			{
				"param_name": "period2",
				"min": 10.0,
				"max": 20.0,
				"step": 1.0
			},
			{
				"param_name": "period3",
				"min": 20.0,
				"max": 40.0,
				"step": 1.0
			},
			{
				"param_name": "oversold",
				"min": 10.0,
				"max": 40.0,
				"step": 5.0
			},
			{
				"param_name": "overbought",
				"min": 60.0,
				"max": 90.0,
				"step": 5.0
			}
		]
	})
}
