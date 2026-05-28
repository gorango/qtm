use crate::types::configs::RocConfig;
use crate::utils::signals::{crossed_over, crossed_under};

/// ROC Momentum Strategy
///
/// Generates buy signals when ROC crosses above oversold level
/// Generates sell signals when ROC crosses below overbought level
///
/// @strategy_id roc
/// @strategy_name ROC Momentum Strategy
/// @category momentum
/// @default_timeframes 15m,1h,4h
pub fn roc_strategy(closes: &[f64], config: Option<RocConfig>) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14);
	let oversold = config.oversold.unwrap_or(-10.0);
	let overbought = config.overbought.unwrap_or(10.0);

	// Validate parameters
	if !(2..=100).contains(&period) {
		return Err("ROC period must be between 2 and 100".to_string());
	}
	if oversold >= overbought {
		return Err("ROC oversold must be less than overbought".to_string());
	}

	let data_len = closes.len();
	if data_len < (period as usize) + 1 {
		return Err("Insufficient data for ROC strategy".to_string());
	}

	// Calculate ROC values
	let roc_config = indicators_core::PriceRateOfChangeConfig {
		period: Some(period),
	};
	let roc_values = indicators_core::price_rate_of_change(closes, Some(roc_config))?;

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if crossed_over(&roc_values, oversold, i as u32) {
			1 // Buy signal
		} else if crossed_under(&roc_values, overbought, i as u32) {
			-1 // Sell signal
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get ROC strategy metadata for registry
pub fn roc_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "roc",
		"name": "ROC Momentum Strategy",
		"category": "momentum",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when ROC crosses above oversold level and sell signals when ROC crosses below overbought level"
	})
}

/// Get ROC strategy default parameters
pub fn roc_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 14,
			"oversold": -10.0,
			"overbought": 10.0
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 5.0,
				"max": 50.0,
				"step": 1.0
			},
			{
				"param_name": "oversold",
				"min": -50.0,
				"max": -5.0,
				"step": 5.0
			},
			{
				"param_name": "overbought",
				"min": 5.0,
				"max": 50.0,
				"step": 5.0
			}
		]
	})
}
