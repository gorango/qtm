use crate::types::configs::ADXConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};
use serde_json;

/// ADX Trend Strategy
///
/// Generates buy signals when ADX crosses above trend threshold in bullish direction (+DI > -DI)
/// Generates sell signals when ADX crosses above trend threshold in bearish direction (-DI > +DI)
/// or when ADX crosses below trend threshold (ranging market)
///
/// @strategy_id adx
/// @strategy_name ADX Trend
/// @category trend
/// @default_timeframes 1h,4h,1d
pub fn adx_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<ADXConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14);
	let trend_threshold = config.trend_threshold.unwrap_or(25.0);

	// Validate parameters
	if !(2..=100).contains(&period) {
		return Err("ADX period must be between 2 and 100".to_string());
	}
	let data_len = highs.len();
	if data_len != lows.len() || data_len != closes.len() {
		return Err("Highs, lows, and closes arrays must have the same length".to_string());
	}
	let min_periods = (period * 3) as usize; // ADX needs more periods
	if data_len < min_periods {
		return Err("Insufficient data for ADX strategy".to_string());
	}

	// Calculate ADX
	let adx_config = indicators_core::ADXConfig {
		period: Some(period),
	};
	let adx_result = indicators_core::adx(highs, lows, closes, Some(adx_config))?;

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);
	let threshold_line = vec![trend_threshold; adx_result.adx.len()];

	for i in 0..data_len {
		let signal = if i < min_periods {
			0 // Not enough data
		} else if crossed_over_series(&adx_result.adx, &threshold_line, i as u32)
			&& adx_result.plus_di[i] > adx_result.minus_di[i]
		{
			1 // Buy signal: ADX crosses above threshold in bullish trend
		} else if (crossed_over_series(&adx_result.adx, &threshold_line, i as u32)
			&& adx_result.minus_di[i] > adx_result.plus_di[i])
			|| crossed_under_series(&adx_result.adx, &threshold_line, i as u32)
		{
			-1 // Sell signal: ADX crosses above threshold in bearish trend or below threshold (ranging market)
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get ADX strategy metadata for registry
pub fn adx_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "adx",
		"name": "ADX Trend",
		"category": "trend",
		"default_timeframes": ["1h", "4h", "1d"],
		"description": "Generates buy signals when ADX crosses above trend threshold in bullish trends (+DI > -DI), sell signals in bearish trends (-DI > +DI) or ranging markets"
	})
}

/// Get ADX strategy default parameters
pub fn adx_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 14,
			"trend_threshold": 25.0
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 5.0,
				"max": 30.0,
				"step": 1.0
			},
			{
				"param_name": "trend_threshold",
				"min": 15.0,
				"max": 35.0,
				"step": 1.0
			}
		]
	})
}
