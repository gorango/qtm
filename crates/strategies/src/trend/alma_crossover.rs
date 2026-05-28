use crate::types::configs::AlmacrossoverConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};
use crate::{StrategyError, StrategyResult};

/// ALMA Crossover Trend Strategy
///
/// Generates buy signals when fast ALMA crosses above slow ALMA
/// Generates sell signals when fast ALMA crosses below slow ALMA
///
/// @strategy_id almaCrossover
/// @strategy_name ALMA Crossover Trend
/// @category trend
/// @default_timeframes 15m,1h,4h
pub fn alma_crossover_strategy(
	closes: &[f64],
	config: Option<AlmacrossoverConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let fast_period = config.fast_period.unwrap_or(9);
	let slow_period = config.slow_period.unwrap_or(21);
	let offset = config.offset.unwrap_or(0.85);

	// Validate parameters
	if !(2..=100).contains(&fast_period) {
		return Err(StrategyError::Validation(
			"ALMA fast period must be between 2 and 100".into(),
		));
	}
	if !(2..=200).contains(&slow_period) {
		return Err(StrategyError::Validation(
			"ALMA slow period must be between 2 and 200".into(),
		));
	}
	let data_len = closes.len();
	let min_periods = slow_period as usize;
	if data_len < min_periods {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for ALMA Crossover strategy".into(),
		));
	}

	// Calculate ALMA lines
	let closes_vec: Vec<f64> = closes.to_vec();
	let fast_alma_config = indicators_core::ALMAConfig {
		period: Some(fast_period),
		offset: Some(offset),
		sigma: Some(6.0),
	};
	let fast_alma = indicators_core::alma(&closes_vec, Some(fast_alma_config))?;

	let slow_alma_config = indicators_core::ALMAConfig {
		period: Some(slow_period),
		offset: Some(offset),
		sigma: Some(6.0),
	};
	let slow_alma = indicators_core::alma(&closes_vec, Some(slow_alma_config))?;

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_periods {
			0 // Not enough data
		} else if crossed_over_series(&fast_alma, &slow_alma, i as u32) {
			1 // Buy signal: fast ALMA crosses above slow ALMA
		} else if crossed_under_series(&fast_alma, &slow_alma, i as u32) {
			-1 // Sell signal: fast ALMA crosses below slow ALMA
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get ALMA Crossover strategy metadata for registry
pub fn alma_crossover_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "almaCrossover",
		"name": "ALMA Crossover Trend",
		"category": "trend",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when fast ALMA crosses above slow ALMA and sell signals when fast ALMA crosses below slow ALMA"
	})
}

/// Get ALMA Crossover strategy default parameters
pub fn alma_crossover_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"fast_period": 9,
			"slow_period": 21,
			"offset": 0.85
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
				"min": 15.0,
				"max": 50.0,
				"step": 1.0
			},
			{
				"param_name": "offset",
				"min": 0.5,
				"max": 1.0,
				"step": 0.05
			}
		]
	})
}
