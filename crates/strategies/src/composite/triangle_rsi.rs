use crate::types::configs::TriangleRsiConfig;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

#[strategy(
    id = "triangle-rsi-breakout",
    name = "Triangle + RSI Breakout",
    category = "composite",
    default_timeframes = ["15m", "1h", "4h"],
    description = "Triangle pattern + RSI breakout",
    opt_params = r#"[{"param_name": "minPoints", "min": 3.0, "max": 8.0, "step": 1.0}, {"param_name": "slopeTolerance", "min": 0.005, "max": 0.05, "step": 0.005}, {"param_name": "rsiPeriod", "min": 7.0, "max": 21.0, "step": 1.0}, {"param_name": "oversold", "min": 20.0, "max": 40.0, "step": 1.0}, {"param_name": "overbought", "min": 60.0, "max": 80.0, "step": 1.0}]"#
)]
pub fn triangle_rsi_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<TriangleRsiConfig>,
) -> StrategyResult<Vec<i8>> {
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
		return Err(StrategyError::Validation(
			"Highs, lows, and closes arrays must have the same length".into(),
		));
	}
	let mut signals = Vec::with_capacity(data_len);

	let triangle = triangle_signals?;

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
