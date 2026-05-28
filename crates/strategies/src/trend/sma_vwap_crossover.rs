use crate::types::configs::SmaVwapCrossoverConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};

/// SMA-VWAP Crossover Strategy
///
/// Generates buy signals when SMA crosses above VWAP
/// Generates sell signals when SMA crosses below VWAP
///
/// @strategy_id smaVwapCrossover
/// @strategy_name Sma Vwap Crossover
/// @category trend
/// @default_timeframes 15m,1h,4h
pub fn sma_vwap_crossover_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	volumes: &[f64],
	config: Option<SmaVwapCrossoverConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let sma_period = config.sma_period.unwrap_or(3);
	let vwap_period = config.vwap_period.unwrap_or(14);
	let price_source = config.price_source.unwrap_or("hlc3".to_string());
	let anchored = config.anchored.unwrap_or(true);
	let session_length = config.session_length.unwrap_or(1440);

	// Validate parameters
	if !(2..=100).contains(&sma_period) {
		return Err("SMA period must be between 2 and 100".to_string());
	}
	if !(2..=200).contains(&vwap_period) {
		return Err("VWAP period must be between 2 and 200".to_string());
	}
	let min_period = sma_period.max(vwap_period) as usize;
	let data_len = highs.len();
	if data_len < min_period
		|| lows.len() < min_period
		|| closes.len() < min_period
		|| volumes.len() < min_period
	{
		return Err("Insufficient data for SMA-VWAP crossover strategy".to_string());
	}

	// Calculate SMA
	let closes_vec: Vec<f64> = closes.to_vec();
	let sma_values = indicators_core::sma(&closes_vec, Some(sma_period))?;

	// Calculate VWAP
	let vwap_config = indicators_core::VWAPConfig {
		period: Some(vwap_period),
		price_source: Some(price_source),
		anchored: Some(anchored),
		session_length: Some(session_length),
	};
	let vwap_values = indicators_core::vwap(highs, lows, closes, volumes, Some(vwap_config));

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_period {
			0 // Not enough data
		} else if crossed_over_series(&sma_values, &vwap_values, i as u32) {
			1 // Buy signal
		} else if crossed_under_series(&sma_values, &vwap_values, i as u32) {
			-1 // Sell signal
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get SMA-VWAP Crossover strategy metadata for registry
pub fn sma_vwap_crossover_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "smaVwapCrossover",
		"name": "Sma Vwap Crossover",
		"category": "trend",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when SMA crosses above VWAP and sell signals when SMA crosses below VWAP"
	})
}

/// Get SMA-VWAP Crossover strategy default parameters
pub fn sma_vwap_crossover_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"sma_period": 3,
			"vwap_period": 14,
			"price_source": "hlc3",
			"anchored": true,
			"session_length": 1440
		},
		"optimization_bounds": [
			{
				"param_name": "sma_period",
				"min": 2.0,
				"max": 10.0,
				"step": 1.0
			},
			{
				"param_name": "vwap_period",
				"min": 5.0,
				"max": 50.0,
				"step": 1.0
			},
			{
				"param_name": "anchored",
				"min": 0.0,
				"max": 1.0,
				"step": 1.0
			},
			{
				"param_name": "session_length",
				"min": 60.0,
				"max": 1440.0,
				"step": 60.0
			}
		]
	})
}
