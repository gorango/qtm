use crate::types::configs::VortexConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};
use indicators_core::VortexResult;

/// Vortex Trend Strategy
///
/// Generates buy signals when VI+ crosses above VI-
/// Generates sell signals when VI+ crosses below VI-
///
/// @strategy_id vortex
/// @strategy_name Vortex Trend
/// @category trend
/// @default_timeframes 15m,1h,4h
pub fn vortex_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<VortexConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14);

	// Validate parameters
	if !(2..=100).contains(&period) {
		return Err("Vortex period must be between 2 and 100".to_string());
	}
	let data_len = closes.len();
	let min_periods = period as usize;
	if data_len < min_periods {
		return Err("Insufficient data for Vortex strategy".to_string());
	}

	// Calculate Vortex
	let vortex_result: VortexResult = indicators_core::vortex(highs, lows, closes, Some(period))?;
	let vi_plus = vortex_result.plus;
	let vi_minus = vortex_result.minus;

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_periods {
			0 // Not enough data
		} else if crossed_over_series(&vi_plus, &vi_minus, i as u32) {
			1 // Buy signal: VI+ crosses above VI-
		} else if crossed_under_series(&vi_plus, &vi_minus, i as u32) {
			-1 // Sell signal: VI+ crosses below VI-
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get Vortex strategy metadata for registry
pub fn vortex_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "vortex",
		"name": "Vortex Trend",
		"category": "trend",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when VI+ crosses above VI- and sell signals when VI+ crosses below VI-"
	})
}

/// Get Vortex strategy default parameters
pub fn vortex_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 14
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 5.0,
				"max": 50.0,
				"step": 1.0
			}
		]
	})
}
