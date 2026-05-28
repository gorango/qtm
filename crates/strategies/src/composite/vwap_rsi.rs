use crate::types::configs::VwapRsiConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};

/// Vwap Rsi
///
/// Buy when price above VWAP and RSI oversold. Sell when below VWAP and RSI overbought.
pub fn vwap_rsi_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	volumes: &[f64],
	config: Option<VwapRsiConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let vwap_period = config.vwap_period.unwrap_or(14);
	let rsi_period = config.rsi_period.unwrap_or(14);
	let oversold = config.oversold.unwrap_or(30.0);
	let overbought = config.overbought.unwrap_or(70.0);

	let data_len = closes.len();
	if highs.len() != data_len || lows.len() != data_len || volumes.len() != data_len {
		return Err(
			"Highs, lows, closes, and volumes arrays must have the same length".to_string(),
		);
	}
	let min_periods = vwap_period.max(rsi_period) as usize;

	if data_len < min_periods {
		return Err(format!(
			"Insufficient data: VWAP + RSI requires at least {} data points, got {}",
			min_periods, data_len
		));
	}

	let vwap_config = indicators_core::VWAPConfig {
		price_source: None,
		anchored: None,
		session_length: None,
		period: Some(vwap_period),
	};
	let highs_vec: Vec<f64> = highs.to_vec();
	let lows_vec: Vec<f64> = lows.to_vec();
	let closes_vec: Vec<f64> = closes.to_vec();
	let volumes_vec: Vec<f64> = volumes.to_vec();
	let vwap_values = indicators_core::vwap(
		&highs_vec,
		&lows_vec,
		&closes_vec,
		&volumes_vec,
		Some(vwap_config),
	);

	let rsi_config = indicators_core::RSIConfig {
		period: Some(rsi_period),
	};
	let rsi_values = indicators_core::rsi(&closes_vec, Some(rsi_config));

	let mut signals = Vec::with_capacity(data_len);

	for (i, &rsi_value) in rsi_values.iter().enumerate().take(data_len) {
		let signal = if i < min_periods {
			0
		} else {
			let crossed_over_vwap = crossed_over_series(&closes_vec, &vwap_values, i as u32);
			let crossed_under_vwap = crossed_under_series(&closes_vec, &vwap_values, i as u32);

			if crossed_over_vwap && rsi_value <= oversold {
				1
			} else if crossed_under_vwap && rsi_value >= overbought {
				-1
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}

pub fn vwap_rsi_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "vwap-rsi-breakout",
		"name": "VWAP + RSI Breakout",
		"category": "composite",
		"description": "Combine VWAP + RSI breakout",
		"default_timeframes": ["15m", "1h", "4h"]
	})
}

pub fn vwap_rsi_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"vwap_period": 14,
			"rsi_period": 14,
			"oversold": 30.0,
			"overbought": 70.0
		},
		"optimization_bounds": [
			{
				"param_name": "vwap_period",
				"min": 5.0,
				"max": 50.0,
				"step": 1.0
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
