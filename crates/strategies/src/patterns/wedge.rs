use crate::types::configs::WedgeConfig;
use serde_json;

/// Wedge Breakout Strategy
///
/// Detects wedge patterns where both lines slope in the same direction
/// Rising Wedge: Both lines slope upward (bearish breakdown)
/// Falling Wedge: Both lines slope downward (bullish breakout)
/// Breakouts typically occur opposite to the wedge direction
///
/// @strategy_id wedge-breakout
/// @strategy_name Wedge Breakout Strategy
/// @category patterns
/// @default_timeframes 15m,1h,4h
pub fn wedge_strategy(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<WedgeConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let min_points = config.min_points.unwrap_or(4) as usize;
	let slope_tolerance = config.slope_tolerance.unwrap_or(0.0001);
	let min_data_length = config.min_data_length.unwrap_or(20) as usize;

	let data_len = highs.len();

	if data_len < min_data_length {
		return Ok(vec![0; data_len]);
	}

	let signals = indicators_core::wedges(
		opens,
		highs,
		lows,
		closes,
		Some(min_points as u32),
		Some(slope_tolerance),
	)?;

	let mut result = Vec::with_capacity(data_len);
	for &s in signals.iter().take(data_len) {
		let signal = if s > 0.5 {
			1 // Bullish breakout (from falling wedge)
		} else if s < -0.5 {
			-1 // Bearish breakdown (from rising wedge)
		} else {
			0
		};
		result.push(signal);
	}

	Ok(result)
}

/// Get Wedge strategy metadata for registry
pub fn wedge_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "wedge-breakout",
		"name": "Wedge Breakout Strategy",
		"category": "patterns",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Detects wedge patterns and generates breakout signals opposite to wedge direction"
	})
}

/// Get Wedge strategy default parameters
pub fn wedge_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"minPoints": 4,
			"slopeTolerance": 0.0001,
			"minDataLength": 20
		},
		"optimization_bounds": [
			{
				"param_name": "minPoints",
				"min": 3,
				"max": 10,
				"step": 1
			},
			{
				"param_name": "slopeTolerance",
				"min": 0.0,
				"max": 0.01,
				"step": 0.0001
			},
			{
				"param_name": "minDataLength",
				"min": 15,
				"max": 50,
				"step": 5
			}
		]
	})
}
