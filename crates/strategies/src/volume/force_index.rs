use crate::types::configs::ForceIndexConfig;
use crate::utils::signals::{crossed_over, crossed_under};

/// Force Index Strategy
///
/// Generates buy signals when Force Index crosses above oversold threshold
/// Generates sell signals when Force Index crosses below overbought threshold
///
/// @strategy_id force-index
/// @strategy_name Force Index
/// @category volume
/// @default_timeframes 15m,1h,4h
pub fn force_index_strategy(
	closes: &[f64],
	volumes: &[f64],
	config: Option<ForceIndexConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(13);
	let oversold = config.oversold.unwrap_or(-0.1);
	let overbought = config.overbought.unwrap_or(0.1);

	let data_len = closes.len();
	if closes.len() != volumes.len() {
		return Err("Closes and volumes must have equal length".to_string());
	}
	if !(5..=50).contains(&period) {
		return Err("Period must be between 5 and 50".to_string());
	}
	if !(overbought..=1.0).contains(&oversold) && !(-1.0..=0.0).contains(&oversold) {
		return Err("Oversold threshold must be between -1 and 1".to_string());
	}
	if !(overbought..=1.0).contains(&overbought) && !(-1.0..=0.0).contains(&overbought) {
		return Err("Overbought threshold must be between -1 and 1".to_string());
	}
	if data_len < (period as usize) + 1 {
		return Err("Insufficient data for Force Index strategy".to_string());
	}

	let closes_vec: Vec<f64> = closes.to_vec();
	let volumes_vec: Vec<f64> = volumes.to_vec();

	let fi_config = indicators_core::FIConfig {
		period: Some(period),
	};
	let fi_values = indicators_core::force_index(&closes_vec, &volumes_vec, Some(fi_config));

	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < (period as usize) {
			0
		} else if crossed_over(&fi_values, oversold, i as u32) {
			1
		} else if crossed_under(&fi_values, overbought, i as u32) {
			-1
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get Force Index strategy metadata for registry
pub fn force_index_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "force-index",
		"name": "Force Index",
		"category": "volume",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when Force Index crosses above oversold threshold, sell signals when Force Index crosses below overbought threshold"
	})
}

/// Get Force Index strategy default parameters
pub fn force_index_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 13,
			"oversold": -0.1,
			"overbought": 0.1
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
				"min": -0.5,
				"max": -0.01,
				"step": 0.01
			},
			{
				"param_name": "overbought",
				"min": 0.01,
				"max": 0.5,
				"step": 0.01
			}
		]
	})
}
