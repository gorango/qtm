use crate::types::configs::VwmaConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};
use crate::{StrategyError, StrategyResult};

/// VWMA Trend Strategy
///
/// Generates buy signals when price crosses above VWMA
/// Generates sell signals when price crosses below VWMA
///
/// @strategy_id vwma
/// @strategy_name VWMA Trend
/// @category trend
/// @default_timeframes 15m,1h,4h
pub fn vwma_strategy(
	closes: &[f64],
	volumes: &[f64],
	config: Option<VwmaConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20);

	// Validate parameters
	if !(2..=200).contains(&period) {
		return Err(StrategyError::Validation(
			"VWMA period must be between 2 and 200".into(),
		));
	}
	let data_len = closes.len();
	let min_periods = period as usize;
	if data_len < min_periods {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for VWMA strategy".into(),
		));
	}

	// Convert for later use
	let closes_vec: Vec<f64> = closes.to_vec();

	// Calculate VWMA
	let vwma_result = indicators_core::vwma(closes, volumes, Some(period))?;

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_periods {
			0 // Not enough data
		} else if crossed_over_series(&closes_vec, &vwma_result, i as u32) {
			1 // Buy signal: price crosses above VWMA
		} else if crossed_under_series(&closes_vec, &vwma_result, i as u32) {
			-1 // Sell signal: price crosses below VWMA
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get VWMA strategy metadata for registry
pub fn vwma_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "vwma",
		"name": "VWMA Trend",
		"category": "trend",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when price crosses above VWMA and sell signals when price crosses below VWMA"
	})
}

/// Get VWMA strategy default parameters
pub fn vwma_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 20
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 10.0,
				"max": 50.0,
				"step": 1.0
			}
		]
	})
}
