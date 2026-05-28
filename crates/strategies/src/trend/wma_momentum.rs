use crate::types::configs::WmaMomentumConfig;

/// WMA Momentum Trend Strategy
///
/// Generates buy signals when WMA is increasing
/// Generates sell signals when WMA is decreasing
///
/// @strategy_id wmaMomentum
/// @strategy_name WMA Momentum Trend
/// @category trend
/// @default_timeframes 15m,1h,4h
pub fn wma_momentum_strategy(
	closes: &[f64],
	config: Option<WmaMomentumConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14);

	// Validate parameters
	if !(2..=100).contains(&period) {
		return Err("WMA period must be between 2 and 100".to_string());
	}
	let data_len = closes.len();
	let min_periods = period as usize;
	if data_len < min_periods + 1 {
		return Err("Insufficient data for WMA Momentum strategy".to_string());
	}

	// Calculate WMA
	let wma_result = indicators_core::wma(closes, Some(period))?;

	// Generate signals based on WMA momentum (increasing/decreasing)
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_periods || i == 0 {
			0 // Not enough data
		} else if wma_result[i] > wma_result[i - 1] {
			1 // Buy signal: WMA increasing
		} else if wma_result[i] < wma_result[i - 1] {
			-1 // Sell signal: WMA decreasing
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get WMA Momentum strategy metadata for registry
pub fn wma_momentum_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "wmaMomentum",
		"name": "WMA Momentum Trend",
		"category": "trend",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when WMA is increasing and sell signals when WMA is decreasing"
	})
}

/// Get WMA Momentum strategy default parameters
pub fn wma_momentum_strategy_defaults() -> serde_json::Value {
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
