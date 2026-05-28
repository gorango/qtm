use crate::types::configs::ChaikinMoneyFlowConfig;
use crate::utils::signals::{crossed_over, crossed_under};

/// Chaikin Money Flow Strategy
///
/// Generates buy signals when CMF crosses above zero
/// Generates sell signals when CMF crosses below zero
///
/// @strategy_id chaikin-money-flow
/// @strategy_name Chaikin Money Flow
/// @category volume
/// @default_timeframes 15m,1h,4h
pub fn chaikin_money_flow_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	volumes: &[f64],
	config: Option<ChaikinMoneyFlowConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20);

	let data_len = closes.len();
	if closes.len() != highs.len() || closes.len() != lows.len() || closes.len() != volumes.len() {
		return Err("All input arrays must have equal length".to_string());
	}
	if !(5..=50).contains(&period) {
		return Err("Period must be between 5 and 50".to_string());
	}
	if data_len < (period as usize) + 1 {
		return Err("Insufficient data for Chaikin Money Flow strategy".to_string());
	}

	let closes_vec: Vec<f64> = closes.to_vec();
	let highs_vec: Vec<f64> = highs.to_vec();
	let lows_vec: Vec<f64> = lows.to_vec();
	let volumes_vec: Vec<f64> = volumes.to_vec();

	let cmf_values = indicators_core::chaikin_money_flow(
		&highs_vec,
		&lows_vec,
		&closes_vec,
		&volumes_vec,
		period,
	);

	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < (period as usize) {
			0
		} else if crossed_over(&cmf_values, 0.0, i as u32) {
			1
		} else if crossed_under(&cmf_values, 0.0, i as u32) {
			-1
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get Chaikin Money Flow strategy metadata for registry
pub fn chaikin_money_flow_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "chaikin-money-flow",
		"name": "Chaikin Money Flow",
		"category": "volume",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when CMF crosses above zero, sell signals when CMF crosses below zero"
	})
}

/// Get Chaikin Money Flow strategy default parameters
pub fn chaikin_money_flow_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 20
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
