use crate::types::configs::BalanceOfPowerConfig;
use crate::utils::signals::{crossed_over, crossed_under};

/// Balance of Power Trend Strategy
///
/// Generates buy signals when BOP crosses over zero
/// Generates sell signals when BOP crosses under zero
///
/// @strategy_id balanceOfPower
/// @strategy_name Balance of Power Trend
/// @category trend
/// @default_timeframes 15m,1h,4h
pub fn balance_of_power_strategy(
	openings: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<BalanceOfPowerConfig>,
) -> Result<Vec<i8>, String> {
	let _config = config.unwrap_or_default(); // Period not used in current implementation

	let data_len = openings.len();
	if data_len == 0 {
		return Err("Input arrays cannot be empty".to_string());
	}

	// Calculate Balance of Power
	let bop_result = indicators_core::balance_of_power(openings, highs, lows, closes)?;

	// Generate signals
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if crossed_over(&bop_result, 0.0, i as u32) {
			1 // Buy signal: BOP crosses over 0
		} else if crossed_under(&bop_result, 0.0, i as u32) {
			-1 // Sell signal: BOP crosses under 0
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get Balance of Power strategy metadata for registry
pub fn balance_of_power_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "balanceOfPower",
		"name": "Balance of Power Trend",
		"category": "trend",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when Balance of Power crosses over zero and sell signals when Balance of Power crosses under zero"
	})
}

/// Get Balance of Power strategy default parameters
pub fn balance_of_power_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 14
		},
		"optimization_bounds": []
	})
}
