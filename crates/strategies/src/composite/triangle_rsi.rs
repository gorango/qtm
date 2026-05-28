use crate::types::configs::TriangleRsiConfig;

/// Triangle Rsi
///
/// Buy on triangle breakout with RSI momentum confirmation. Sell on breakdown.
pub fn triangle_rsi_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<TriangleRsiConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let min_points = config.min_points.unwrap_or(4);
	let slope_tolerance = config.slope_tolerance.unwrap_or(0.01);
	let rsi_period = config.rsi_period.unwrap_or(14);
	let oversold = config.oversold.unwrap_or(30.0);
	let overbought = config.overbought.unwrap_or(70.0);

	let convergence_tolerance = 0.001;

	let highs_vec: Vec<f64> = highs.to_vec();
	let lows_vec: Vec<f64> = lows.to_vec();
	let closes_vec: Vec<f64> = closes.to_vec();
	let opens_vec: Vec<f64> = closes_vec.clone();
	let triangle_signals = indicators_core::triangles(
		&opens_vec,
		&highs_vec,
		&lows_vec,
		&closes_vec,
		Some(min_points),
		Some(slope_tolerance),
		Some(convergence_tolerance),
	);

	let rsi_config = indicators_core::RSIConfig {
		period: Some(rsi_period),
	};
	let rsi_values = indicators_core::rsi(&closes_vec, Some(rsi_config));

	let data_len = closes.len();
	if highs.len() != data_len || lows.len() != data_len {
		return Err("Highs, lows, and closes arrays must have the same length".to_string());
	}
	let mut signals = Vec::with_capacity(data_len);

	let triangle = triangle_signals.as_ref().unwrap();

	for i in 0..data_len {
		let signal = if i < rsi_period as usize {
			0
		} else if triangle[i] == 1.0 && rsi_values[i] <= oversold {
			1
		} else if triangle[i] == -1.0 && rsi_values[i] >= overbought {
			-1
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}

pub fn triangle_rsi_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "triangle-rsi-breakout",
		"name": "Triangle + RSI Breakout",
		"category": "composite",
		"description": "Triangle pattern + RSI breakout",
		"default_timeframes": ["15m", "1h", "4h"]
	})
}

pub fn triangle_rsi_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"min_points": 4,
			"slope_tolerance": 0.01,
			"rsi_period": 14,
			"oversold": 30.0,
			"overbought": 70.0
		},
		"optimization_bounds": [
			{
				"param_name": "min_points",
				"min": 3.0,
				"max": 8.0,
				"step": 1.0
			},
			{
				"param_name": "slope_tolerance",
				"min": 0.005,
				"max": 0.05,
				"step": 0.005
			},
			{
				"param_name": "rsi_period",
				"min": 7.0,
				"max": 21.0,
				"step": 1.0
			},
			{
				"param_name": "oversold",
				"min": 20.0,
				"max": 40.0,
				"step": 1.0
			},
			{
				"param_name": "overbought",
				"min": 60.0,
				"max": 80.0,
				"step": 1.0
			}
		]
	})
}
