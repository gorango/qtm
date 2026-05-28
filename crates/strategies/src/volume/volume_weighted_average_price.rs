use crate::types::configs::VolumeWeightedAveragePriceConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};
use crate::{StrategyError, StrategyResult};

/// Volume Weighted Average Price Trend Strategy
///
/// Generates buy signals when price crosses above VWAP
/// Generates sell signals when price crosses below VWAP
///
/// @strategy_id volumeWeightedAveragePrice
/// @strategy_name Volume Weighted Average Price Trend
/// @category volume
/// @default_timeframes 15m,1h,4h
pub fn volume_weighted_average_price_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	volumes: &[f64],
	config: Option<VolumeWeightedAveragePriceConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14);

	// Validate parameters
	if !(2..=100).contains(&period) {
		return Err(StrategyError::Validation(
			"VWAP period must be between 2 and 100".into(),
		));
	}
	let data_len = closes.len();
	let min_periods = period as usize;
	if data_len < min_periods {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for Volume Weighted Average Price strategy".into(),
		));
	}

	// Calculate VWAP
	let vwap_config = indicators_core::VWAPConfig {
		period: Some(period),
		anchored: Some(true),
		session_length: Some(1440),
		price_source: Some("hlc3".to_string()),
	};
	let vwap_result = indicators_core::vwap(highs, lows, closes, volumes, Some(vwap_config));

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);
	let _zero_line = vec![0.0; vwap_result.len()];

	for i in 0..data_len {
		let signal = if i < min_periods {
			0 // Not enough data
		} else if crossed_over_series(closes, &vwap_result, i as u32) {
			1 // Buy signal: price crosses above VWAP
		} else if crossed_under_series(closes, &vwap_result, i as u32) {
			-1 // Sell signal: price crosses below VWAP
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get Volume Weighted Average Price strategy metadata for registry
pub fn volume_weighted_average_price_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "volumeWeightedAveragePrice",
		"name": "Volume Weighted Average Price Trend",
		"category": "volume",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when price crosses above VWAP and sell signals when price crosses below VWAP"
	})
}

/// Get Volume Weighted Average Price strategy default parameters
pub fn volume_weighted_average_price_strategy_defaults() -> serde_json::Value {
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
