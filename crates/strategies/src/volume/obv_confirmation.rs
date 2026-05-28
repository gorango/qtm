use crate::types::configs::ObvConfirmationConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};

/// OBV Confirmation Strategy
///
/// Generates buy signals when OBV crosses above its SMA
/// Generates sell signals when OBV crosses below its SMA
///
/// @strategy_id obv-confirmation
/// @strategy_name OBV Confirmation
/// @category volume
/// @default_timeframes 15m,1h,4h
pub fn obv_confirmation_strategy(
	closes: &[f64],
	volumes: &[f64],
	config: Option<ObvConfirmationConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let obv_period = config.obv_period.unwrap_or(10);
	let price_period = config.price_period.unwrap_or(10);

	let data_len = closes.len();
	if closes.len() != volumes.len() {
		return Err("Closes and volumes must have equal length".to_string());
	}
	if !(5..=50).contains(&obv_period) {
		return Err("OBV period must be between 5 and 50".to_string());
	}
	if !(5..=50).contains(&price_period) {
		return Err("Price period must be between 5 and 50".to_string());
	}
	let min_required = std::cmp::max(obv_period, price_period) as usize;
	if data_len < min_required + 1 {
		return Err("Insufficient data for OBV Confirmation strategy".to_string());
	}

	let closes_vec: Vec<f64> = closes.to_vec();
	let volumes_vec: Vec<f64> = volumes.to_vec();

	let obv_values = indicators_core::on_balance_volume(&closes_vec, &volumes_vec);

	let obv_ma = indicators_core::sma(&obv_values, Some(obv_period))?;

	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < min_required {
			0
		} else if crossed_over_series(&obv_values, &obv_ma, i as u32) {
			1
		} else if crossed_under_series(&obv_values, &obv_ma, i as u32) {
			-1
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get OBV Confirmation strategy metadata for registry
pub fn obv_confirmation_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "obv-confirmation",
		"name": "OBV Confirmation",
		"category": "volume",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when OBV crosses above its SMA, sell signals when OBV crosses below its SMA"
	})
}

/// Get OBV Confirmation strategy default parameters
pub fn obv_confirmation_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"obv_period": 10,
			"price_period": 10
		},
		"optimization_bounds": [
			{
				"param_name": "obv_period",
				"min": 5.0,
				"max": 50.0,
				"step": 1.0
			},
			{
				"param_name": "price_period",
				"min": 5.0,
				"max": 50.0,
				"step": 1.0
			}
		]
	})
}
