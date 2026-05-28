use crate::types::configs::DoubleTopBottomConfig;

/// Double Top/Bottom Reversal Strategy
///
/// Detects double top (bearish) and double bottom (bullish) reversal patterns
/// Double Top: Two peaks at similar price levels followed by breakdown
/// Double Bottom: Two troughs at similar price levels followed by breakout
///
/// @strategy_id double-top-bottom-reversal
/// @strategy_name Double Top/Bottom Reversal Strategy
/// @category patterns
/// @default_timeframes 15m,1h,4h
pub fn double_top_bottom_strategy(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<DoubleTopBottomConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let min_distance = config.min_distance.unwrap_or(10) as usize;
	let tolerance = config.tolerance.unwrap_or(0.03);
	let lookaround = config.lookaround.unwrap_or(2);

	let data_len = highs.len();

	if data_len < min_distance * 2 {
		return Ok(vec![0; data_len]);
	}

	let opens_vec = opens;
	let highs_vec = highs;
	let lows_vec = lows;
	let closes_vec = closes;

	let bullish_signals = indicators_core::double_bottom(
		opens_vec,
		highs_vec,
		lows_vec,
		closes_vec,
		Some(tolerance),
		Some(min_distance as u32),
		Some(lookaround),
	)?;

	let bearish_signals = indicators_core::double_top(
		opens_vec,
		highs_vec,
		lows_vec,
		closes_vec,
		Some(tolerance),
		Some(min_distance as u32),
		Some(lookaround),
	)?;

	let mut result = Vec::with_capacity(data_len);
	for i in 0..data_len {
		let signal = if bullish_signals[i] > 0.5 {
			1 // Bullish double bottom
		} else if bearish_signals[i] < -0.5 {
			-1 // Bearish double top
		} else {
			0
		};
		result.push(signal);
	}

	Ok(result)
}

/// Get Double Top/Bottom strategy metadata for registry
pub fn double_top_bottom_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "double-top-bottom-reversal",
		"name": "Double Top/Bottom Reversal Strategy",
		"category": "patterns",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Detects double top (bearish) and double bottom (bullish) reversal patterns"
	})
}

/// Get Double Top/Bottom strategy default parameters
pub fn double_top_bottom_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"minDistance": 10,
			"tolerance": 0.03,
			"lookaround": 2
		},
		"optimization_bounds": [
			{
				"param_name": "minDistance",
				"min": 5,
				"max": 50,
				"step": 1
			},
			{
				"param_name": "tolerance",
				"min": 0.0,
				"max": 0.1,
				"step": 0.005
			},
			{
				"param_name": "lookaround",
				"min": 1,
				"max": 5,
				"step": 1
			}
		]
	})
}
