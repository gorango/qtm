use crate::types::configs::HeadAndShouldersConfig;
use crate::StrategyResult;

/// Head and Shoulders Reversal Strategy
///
/// Detects head and shoulders (bearish) and inverse head and shoulders (bullish) patterns
/// Regular Head & Shoulders: Left shoulder - head - right shoulder formation (bearish)
/// Inverse Head & Shoulders: Inverted formation (bullish)
///
/// @strategy_id head-and-shoulders-reversal
/// @strategy_name Head and Shoulders Reversal Strategy
/// @category patterns
/// @default_timeframes 15m,1h,4h
pub fn head_and_shoulders_strategy(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<HeadAndShouldersConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let min_distance = config.min_distance.unwrap_or(5) as usize;
	let tolerance = config.tolerance.unwrap_or(0.02);
	let deviation = config.deviation.unwrap_or(0.005);
	let min_data_length = config.min_data_length.unwrap_or(15) as usize;

	let data_len = highs.len();

	if data_len < min_data_length {
		return Ok(vec![0; data_len]);
	}

	let signals = indicators_core::head_and_shoulders(
		opens,
		highs,
		lows,
		closes,
		Some(min_distance as u32),
		Some(tolerance),
		Some(deviation),
	)?;

	let mut result = Vec::with_capacity(data_len);
	for &s in signals.iter().take(data_len) {
		let signal = if s < -0.5 {
			1 // Inverse head & shoulders (bullish)
		} else if s > 0.5 {
			-1 // Regular head & shoulders (bearish)
		} else {
			0
		};
		result.push(signal);
	}

	Ok(result)
}

/// Get Head and Shoulders strategy metadata for registry
pub fn head_and_shoulders_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "head-and-shoulders-reversal",
		"name": "Head and Shoulders Reversal Strategy",
		"category": "patterns",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Detects head and shoulders (bearish) and inverse head and shoulders (bullish) patterns"
	})
}

/// Get Head and Shoulders strategy default parameters
pub fn head_and_shoulders_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"minDistance": 5,
			"tolerance": 0.02,
			"deviation": 0.005,
			"minDataLength": 15
		},
		"optimization_bounds": [
			{
				"param_name": "minDistance",
				"min": 3,
				"max": 20,
				"step": 1
			},
			{
				"param_name": "tolerance",
				"min": 0.0,
				"max": 0.1,
				"step": 0.005
			},
			{
				"param_name": "deviation",
				"min": 0.0,
				"max": 0.1,
				"step": 0.005
			},
			{
				"param_name": "minDataLength",
				"min": 10,
				"max": 50,
				"step": 5
			}
		]
	})
}
