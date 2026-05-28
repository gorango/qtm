use crate::types::configs::AlmahmaDivergenceConfig;
use indicators_core::ALMAConfig;
use serde_json;

/// ALMA HMA Divergence Trend Strategy
///
/// Generates buy signals when ALMA diverges above HMA by threshold
/// Generates sell signals when ALMA diverges below HMA by threshold
///
/// @strategy_id almaHmaDivergence
/// @strategy_name ALMA HMA Divergence Trend
/// @category trend
/// @default_timeframes 15m,1h,4h
pub fn alma_hma_divergence_strategy(
	closes: &[f64],
	config: Option<AlmahmaDivergenceConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let fast_period = config.fast_period.unwrap_or(9);
	let slow_period = config.slow_period.unwrap_or(21);
	let offset = config.offset.unwrap_or(0.85);
	let divergence_threshold = config.divergence_threshold.unwrap_or(0.01);

	// Validate parameters
	if !(2..=50).contains(&fast_period) {
		return Err("ALMA fast period must be between 2 and 50".to_string());
	}
	if !(2..=100).contains(&slow_period) {
		return Err("HMA slow period must be between 2 and 100".to_string());
	}
	if !(0.0..=1.0).contains(&offset) {
		return Err("ALMA offset must be between 0 and 1".to_string());
	}
	if divergence_threshold <= 0.0 {
		return Err("Divergence threshold must be positive".to_string());
	}
	let data_len = closes.len();
	let min_periods = slow_period as usize;
	if data_len < min_periods {
		return Err("Insufficient data for ALMA HMA Divergence strategy".to_string());
	}

	// Calculate ALMA and HMA
	let closes_copy = closes;
	let alma_result = indicators_core::alma(
		closes_copy,
		Some(ALMAConfig {
			period: Some(fast_period),
			offset: Some(offset),
			sigma: Some(6.0),
		}),
	)?;
	let hma_result = indicators_core::hma(closes, Some(slow_period))?;

	// Generate signals based on divergence
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_periods {
			0 // Not enough data
		} else {
			let divergence = alma_result[i] - hma_result[i];
			if divergence > divergence_threshold {
				1 // Buy signal: ALMA significantly above HMA
			} else if divergence < -divergence_threshold {
				-1 // Sell signal: ALMA significantly below HMA
			} else {
				0 // Hold
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get ALMA HMA Divergence strategy metadata for registry
pub fn alma_hma_divergence_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "almaHmaDivergence",
		"name": "ALMA HMA Divergence Trend",
		"category": "trend",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when ALMA diverges above HMA by threshold and sell signals when ALMA diverges below HMA by threshold"
	})
}

/// Get ALMA HMA Divergence strategy default parameters
pub fn alma_hma_divergence_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"fast_period": 9,
			"slow_period": 21,
			"offset": 0.85,
			"divergence_threshold": 0.01
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
				"min": 10.0,
				"max": 50.0,
				"step": 1.0
			},
			{
				"param_name": "offset",
				"min": 0.5,
				"max": 1.0,
				"step": 0.05
			},
			{
				"param_name": "divergence_threshold",
				"min": 0.001,
				"max": 0.1,
				"step": 0.001
			}
		]
	})
}
