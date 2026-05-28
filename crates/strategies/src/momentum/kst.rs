use crate::types::configs::KSTConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};
use serde_json;

/// KST Trend Strategy
///
/// Generates buy signals when KST crosses above signal line
/// Generates sell signals when KST crosses below signal line
///
/// @strategy_id kst
/// @strategy_name KST Trend
/// @category momentum
/// @default_timeframes 1h,4h,1d
pub fn kst_strategy(closes: &[f64], config: Option<KSTConfig>) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let roc1_period = config.roc1_period.unwrap_or(10);
	let roc2_period = config.roc2_period.unwrap_or(15);
	let roc3_period = config.roc3_period.unwrap_or(20);
	let roc4_period = config.roc4_period.unwrap_or(30);
	let signal_period = config.signal_period.unwrap_or(9);

	// Validate parameters
	for period in &[
		roc1_period,
		roc2_period,
		roc3_period,
		roc4_period,
		signal_period,
	] {
		if !(2..=100).contains(period) {
			return Err("KST periods must be between 2 and 100".to_string());
		}
	}
	let data_len = closes.len();
	let min_periods = (roc4_period + signal_period) as usize; // Rough estimate
	if data_len < min_periods {
		return Err("Insufficient data for KST strategy".to_string());
	}

	// Calculate KST
	let kst_config = indicators_core::KSTConfig {
		roc1_period: Some(roc1_period),
		roc2_period: Some(roc2_period),
		roc3_period: Some(roc3_period),
		roc4_period: Some(roc4_period),
		sma1_period: Some(10), // Default SMAs
		sma2_period: Some(10),
		sma3_period: Some(10),
		sma4_period: Some(15),
		signal_period: Some(signal_period),
	};
	let kst_result = indicators_core::kst(closes, Some(kst_config));

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_periods {
			0 // Not enough data
		} else if crossed_over_series(&kst_result.kst, &kst_result.signal, i as u32) {
			1 // Buy signal: KST crosses above signal
		} else if crossed_under_series(&kst_result.kst, &kst_result.signal, i as u32) {
			-1 // Sell signal: KST crosses below signal
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get KST strategy metadata for registry
pub fn kst_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "kst",
		"name": "KST Trend",
		"category": "momentum",
		"default_timeframes": ["1h", "4h", "1d"],
		"description": "Generates buy signals when KST crosses above signal line and sell signals when KST crosses below signal line"
	})
}

/// Get KST strategy default parameters
pub fn kst_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"roc1_period": 10,
			"roc2_period": 15,
			"roc3_period": 20,
			"roc4_period": 30,
			"signal_period": 9
		},
		"optimization_bounds": [
			{
				"param_name": "roc1_period",
				"min": 5.0,
				"max": 15.0,
				"step": 1.0
			},
			{
				"param_name": "roc2_period",
				"min": 10.0,
				"max": 20.0,
				"step": 1.0
			},
			{
				"param_name": "roc3_period",
				"min": 15.0,
				"max": 25.0,
				"step": 1.0
			},
			{
				"param_name": "roc4_period",
				"min": 20.0,
				"max": 40.0,
				"step": 1.0
			},
			{
				"param_name": "signal_period",
				"min": 5.0,
				"max": 15.0,
				"step": 1.0
			}
		]
	})
}
