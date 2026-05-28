use crate::types::configs::EaseOfMovementConfig;
use crate::utils::signals::{crossed_over, crossed_under};

/// Ease of Movement Strategy
///
/// Generates buy signals when EOM crosses above zero
/// Generates sell signals when EOM crosses below zero
///
/// @strategy_id ease-of-movement
/// @strategy_name Ease of Movement
/// @category volume
/// @default_timeframes 15m,1h,4h
pub fn ease_of_movement_strategy(
	highs: &[f64],
	lows: &[f64],
	volumes: &[f64],
	config: Option<EaseOfMovementConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14);

	let data_len = highs.len();
	if highs.len() != lows.len() || highs.len() != volumes.len() {
		return Err("All input arrays must have equal length".to_string());
	}
	if !(5..=50).contains(&period) {
		return Err("Period must be between 5 and 50".to_string());
	}
	if data_len < (period as usize) + 1 {
		return Err("Insufficient data for Ease of Movement strategy".to_string());
	}

	let highs_vec: Vec<f64> = highs.to_vec();
	let lows_vec: Vec<f64> = lows.to_vec();
	let volumes_vec: Vec<f64> = volumes.to_vec();

	let eom_values = indicators_core::ease_of_movement(&highs_vec, &lows_vec, &volumes_vec, period);

	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < (period as usize) {
			0
		} else if crossed_over(&eom_values, 0.0, i as u32) {
			1
		} else if crossed_under(&eom_values, 0.0, i as u32) {
			-1
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get Ease of Movement strategy metadata for registry
pub fn ease_of_movement_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "ease-of-movement",
		"name": "Ease of Movement",
		"category": "volume",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when EOM crosses above zero, sell signals when EOM crosses below zero"
	})
}

/// Get Ease of Movement strategy default parameters
pub fn ease_of_movement_strategy_defaults() -> serde_json::Value {
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
