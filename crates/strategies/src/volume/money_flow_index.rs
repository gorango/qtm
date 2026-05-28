use crate::types::configs::MoneyFlowIndexConfig;
use crate::utils::signals::{crossed_over, crossed_under};

/// Money Flow Index Strategy
///
/// Generates buy signals when MFI crosses below oversold level
/// Generates sell signals when MFI crosses above overbought level
///
/// @strategy_id money-flow-index
/// @strategy_name Money Flow Index
/// @category volume
/// @default_timeframes 15m,1h,4h
pub fn money_flow_index_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	volumes: &[f64],
	config: Option<MoneyFlowIndexConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14);
	let oversold = config.oversold.unwrap_or(20.0);
	let overbought = config.overbought.unwrap_or(80.0);

	let data_len = closes.len();
	if closes.len() != highs.len() || closes.len() != lows.len() || closes.len() != volumes.len() {
		return Err("All input arrays must have equal length".to_string());
	}
	if !(5..=50).contains(&period) {
		return Err("Period must be between 5 and 50".to_string());
	}
	if !(0.0..=100.0).contains(&oversold) || !(0.0..=100.0).contains(&overbought) {
		return Err("MFI thresholds must be between 0 and 100".to_string());
	}
	if data_len < (period as usize) + 1 {
		return Err("Insufficient data for Money Flow Index strategy".to_string());
	}

	let highs_vec: Vec<f64> = highs.to_vec();
	let lows_vec: Vec<f64> = lows.to_vec();
	let closes_vec: Vec<f64> = closes.to_vec();
	let volumes_vec: Vec<f64> = volumes.to_vec();

	let mfi_values =
		indicators_core::money_flow_index(&highs_vec, &lows_vec, &closes_vec, &volumes_vec, None);

	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < (period as usize) {
			0
		} else if crossed_under(&mfi_values, oversold, i as u32) {
			1
		} else if crossed_over(&mfi_values, overbought, i as u32) {
			-1
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get Money Flow Index strategy metadata for registry
pub fn money_flow_index_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "money-flow-index",
		"name": "Money Flow Index",
		"category": "volume",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when MFI crosses below oversold level, sell signals when MFI crosses above overbought level"
	})
}

/// Get Money Flow Index strategy default parameters
pub fn money_flow_index_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 14,
			"oversold": 20.0,
			"overbought": 80.0
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
				"min": 10.0,
				"max": 40.0,
				"step": 1.0
			},
			{
				"param_name": "overbought",
				"min": 60.0,
				"max": 90.0,
				"step": 1.0
			}
		]
	})
}
