use crate::types::configs::WmaConfirmationConfig;

/// WMA Confirmation Trend Strategy
///
/// Generates buy signals when WMA slope is above threshold
/// Generates sell signals when WMA slope is below negative threshold
///
/// @strategy_id wmaConfirmation
/// @strategy_name WMA Confirmation Trend
/// @category trend
/// @default_timeframes 15m,1h,4h
pub fn wma_confirmation_strategy(
	closes: &[f64],
	config: Option<WmaConfirmationConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20);
	let threshold = config.threshold.unwrap_or(0.001);

	// Validate parameters
	if !(2..=100).contains(&period) {
		return Err("WMA period must be between 2 and 100".to_string());
	}
	if threshold <= 0.0 {
		return Err("WMA threshold must be positive".to_string());
	}
	let data_len = closes.len();
	let min_periods = period as usize;
	if data_len < min_periods + 1 {
		return Err("Insufficient data for WMA Confirmation strategy".to_string());
	}

	// Calculate WMA
	let wma_result = indicators_core::wma(closes, Some(period))?;

	// Generate signals based on WMA slope
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_periods || i == 0 {
			0 // Not enough data
		} else {
			let slope = wma_result[i] - wma_result[i - 1];
			if slope > threshold {
				1 // Buy signal: positive slope above threshold
			} else if slope < -threshold {
				-1 // Sell signal: negative slope below threshold
			} else {
				0 // Hold
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get WMA Confirmation strategy metadata for registry
pub fn wma_confirmation_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "wmaConfirmation",
		"name": "WMA Confirmation Trend",
		"category": "trend",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when WMA slope is above threshold and sell signals when WMA slope is below negative threshold"
	})
}

/// Get WMA Confirmation strategy default parameters
pub fn wma_confirmation_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 20,
			"threshold": 0.001
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 5.0,
				"max": 50.0,
				"step": 1.0
			},
			{
				"param_name": "threshold",
				"min": 0.0001,
				"max": 0.01,
				"step": 0.0001
			}
		]
	})
}
