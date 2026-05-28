use crate::types::configs::NegativeVolumeIndexConfig;
use crate::utils::signals::{crossed_over, crossed_under};
use crate::{StrategyError, StrategyResult};

/// Negative Volume Index Strategy
///
/// Generates buy signals when NVI crosses above its SMA
/// Generates sell signals when NVI crosses below its SMA
///
/// @strategy_id negative-volume-index
/// @strategy_name Negative Volume Index
/// @category volume
/// @default_timeframes 15m,1h,4h
pub fn negative_volume_index_strategy(
	closes: &[f64],
	volumes: &[f64],
	config: Option<NegativeVolumeIndexConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14);
	let start = config.start.unwrap_or(1000.0);

	let data_len = closes.len();
	if closes.len() != volumes.len() {
		return Err(StrategyError::Validation(
			"Closes and volumes must have equal length".into(),
		));
	}
	if !(5..=50).contains(&period) {
		return Err(StrategyError::Validation(
			"Period must be between 5 and 50".into(),
		));
	}
	if data_len < 1 {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for Negative Volume Index strategy".into(),
		));
	}

	let closes_vec: Vec<f64> = closes.to_vec();
	let volumes_vec: Vec<f64> = volumes.to_vec();

	let nvi_values = indicators_core::negative_volume_index(&closes_vec, &volumes_vec, Some(start));

	let _start_level = vec![start; data_len];
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < 1 {
			0
		} else if crossed_over(&nvi_values, start, i as u32) {
			1
		} else if crossed_under(&nvi_values, start, i as u32) {
			-1
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get Negative Volume Index strategy metadata for registry
pub fn negative_volume_index_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "negative-volume-index",
		"name": "Negative Volume Index",
		"category": "volume",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when NVI crosses above start level, sell signals when NVI crosses below start level"
	})
}

/// Get Negative Volume Index strategy default parameters
pub fn negative_volume_index_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 14,
			"start": 1000.0
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 5.0,
				"max": 50.0,
				"step": 1.0
			},
			{
				"param_name": "start",
				"min": 100.0,
				"max": 10000.0,
				"step": 100.0
			}
		]
	})
}
