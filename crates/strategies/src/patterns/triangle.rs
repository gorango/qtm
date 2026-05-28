use crate::types::configs::TriangleConfig;

/// Triangle Breakout Strategy
///
/// Detects triangle patterns and generates breakout signals
/// Ascending Triangle: Flat resistance line, rising support (bullish breakout)
/// Descending Triangle: Falling resistance line, flat support (bearish breakout)
/// Symmetrical Triangle: Converging lines (direction depends on breakout)
///
/// @strategy_id triangle-breakout
/// @strategy_name Triangle Breakout Strategy
/// @category patterns
/// @default_timeframes 15m,1h,4h
pub fn triangle_strategy(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<TriangleConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let min_points = config.min_points.unwrap_or(4) as usize;
	let slope_tolerance = config.slope_tolerance.unwrap_or(0.01);
	let min_data_length = config.min_data_length.unwrap_or(20) as usize;
	let angle_tolerance = config.angle_tolerance.unwrap_or(0.001);

	let data_len = highs.len();

	if data_len < min_data_length {
		return Ok(vec![0; data_len]);
	}

	let signals = indicators_core::triangles(
		opens,
		highs,
		lows,
		closes,
		Some(min_points as u32),
		Some(slope_tolerance),
		Some(angle_tolerance),
	)?;

	let mut result = Vec::with_capacity(data_len);
	for &s in signals.iter().take(data_len) {
		let signal = if s > 0.5 {
			1 // Bullish breakout
		} else if s < -0.5 {
			-1 // Bearish breakout
		} else {
			0
		};
		result.push(signal);
	}

	Ok(result)
}

/// Get Triangle strategy metadata for registry
pub fn triangle_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "triangle-breakout",
		"name": "Triangle Breakout Strategy",
		"category": "patterns",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Detects triangle patterns and generates breakout signals"
	})
}

/// Get Triangle strategy default parameters
pub fn triangle_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"minPoints": 4,
			"slopeTolerance": 0.01,
			"minDataLength": 20,
			"angleTolerance": 0.001
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
				"max": 0.1,
				"step": 0.005
			},
			{
				"param_name": "minDataLength",
				"min": 15,
				"max": 50,
				"step": 5
			},
			{
				"param_name": "angleTolerance",
				"min": 0.0001,
				"max": 0.01,
				"step": 0.0001
			}
		]
	})
}
