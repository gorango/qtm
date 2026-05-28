use crate::types::configs::RSIConfig;

/// Obv Rsi
///
/// Buy when OBV/volume diverges bullishly with RSI oversold. Sell on bearish divergence.
pub fn obv_rsi_strategy(
	closes: &[f64],
	volumes: &[f64],
	rsi_config: Option<RSIConfig>,
) -> Result<Vec<i8>, String> {
	let rsi_cfg = rsi_config.unwrap_or_default();

	let period = rsi_cfg.period.unwrap_or(14);
	let oversold = rsi_cfg.oversold.unwrap_or(30.0);
	let overbought = rsi_cfg.overbought.unwrap_or(70.0);

	let data_len = closes.len();
	if volumes.len() != data_len {
		return Err("Closes and volumes arrays must have the same length".to_string());
	}
	let min_data_length = (period + 1) as usize;

	if data_len < min_data_length {
		return Err(format!(
			"Insufficient data: OBV + RSI requires at least {} data points, got {}",
			min_data_length, data_len
		));
	}

	let rsi_config = indicators_core::RSIConfig {
		period: Some(period),
	};

	let rsi_values = indicators_core::rsi(closes, Some(rsi_config));
	let obv_values = indicators_core::on_balance_volume(closes, volumes);

	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i == 0 {
			0
		} else {
			let obv_increasing = obv_values[i] > obv_values[i - 1];
			let obv_decreasing = obv_values[i] < obv_values[i - 1];
			let rsi_value = rsi_values[i];

			if obv_increasing && rsi_value <= oversold {
				1
			} else if obv_decreasing && rsi_value >= overbought {
				-1
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}

pub fn obv_rsi_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "obv-rsi-volume-confirmation",
		"name": "OBV + RSI Volume Confirmation",
		"category": "composite",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Combine OBV + RSI confirmation"
	})
}

pub fn obv_rsi_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 14,
			"oversold": 30.0,
			"overbought": 70.0
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 5.0,
				"max": 30.0,
				"step": 1.0
			},
			{
				"param_name": "oversold",
				"min": 10.0,
				"max": 40.0,
				"step": 5.0
			},
			{
				"param_name": "overbought",
				"min": 60.0,
				"max": 90.0,
				"step": 5.0
			}
		]
	})
}
