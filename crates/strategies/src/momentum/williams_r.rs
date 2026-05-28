use crate::types::configs::WilliamsRConfig;
use crate::utils::signals::{crossed_over, crossed_under};

/// Williams %R Momentum Strategy
///
/// Generates buy signals when Williams %R crosses above oversold level
/// Generates sell signals when Williams %R crosses below overbought level
///
/// @strategy_id williamsR
/// @strategy_name Williams %R Momentum Strategy
/// @category momentum
/// @default_timeframes 15m,1h,4h
pub fn williams_r_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<WilliamsRConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14);
	let oversold = config.oversold.unwrap_or(-80.0);
	let overbought = config.overbought.unwrap_or(-20.0);

	// Validate parameters
	if !(2..=100).contains(&period) {
		return Err("Williams %R period must be between 2 and 100".to_string());
	}
	if !(-100.0..=0.0).contains(&oversold) || !(-100.0..=0.0).contains(&overbought) {
		return Err("Williams %R thresholds must be between -100 and 0".to_string());
	}
	if oversold >= overbought {
		return Err("Williams %R oversold must be less than overbought".to_string());
	}

	let data_len = highs.len();
	if data_len < (period as usize) + 1 {
		return Err("Insufficient data for Williams %R strategy".to_string());
	}
	if lows.len() != data_len || closes.len() != data_len {
		return Err("All price arrays must have the same length".to_string());
	}

	// Calculate Williams %R values
	let williams_config = indicators_core::WilliamsRConfig {
		period: Some(period),
	};
	let williams_values = indicators_core::williams_r(highs, lows, closes, Some(williams_config))?;

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if crossed_over(&williams_values, oversold, i as u32) {
			1 // Buy signal
		} else if crossed_under(&williams_values, overbought, i as u32) {
			-1 // Sell signal
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get Williams %R strategy metadata for registry
pub fn williams_r_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "williamsR",
		"name": "Williams %R Momentum Strategy",
		"category": "momentum",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when Williams %R crosses above oversold level and sell signals when Williams %R crosses below overbought level"
	})
}

/// Get Williams %R strategy default parameters
pub fn williams_r_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 14,
			"oversold": -80.0,
			"overbought": -20.0
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 5.0,
				"max": 20.0,
				"step": 1.0
			},
			{
				"param_name": "oversold",
				"min": -90.0,
				"max": -70.0,
				"step": 1.0
			},
			{
				"param_name": "overbought",
				"min": -30.0,
				"max": -10.0,
				"step": 1.0
			}
		]
	})
}
