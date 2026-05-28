use crate::types::configs::MfiObvConfig;
use crate::{StrategyError, StrategyResult};

/// Mfi Obv
///
/// Buy when MFI oversold and OBV confirms accumulation. Sell on distribution.
pub fn mfi_obv_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	volumes: &[f64],
	config: Option<MfiObvConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let mfi_period = config.mfi_period.unwrap_or(14);
	let oversold = config.oversold.unwrap_or(20.0);
	let overbought = config.overbought.unwrap_or(80.0);

	let data_len = closes.len();
	if highs.len() != data_len || lows.len() != data_len || volumes.len() != data_len {
		return Err(StrategyError::Validation(
			"Highs, lows, closes, and volumes arrays must have the same length".into(),
		));
	}
	let min_data_length = (mfi_period + 1) as usize;

	if data_len < min_data_length {
		return Err(StrategyError::InsufficientData(format!(
			"Insufficient data: MFI + OBV requires at least {} data points, got {}",
			min_data_length, data_len
		)));
	}

	let mfi_config = indicators_core::MFIConfig {
		price_source: None,
		period: Some(mfi_period),
	};

	let mfi_values = indicators_core::mfi(highs, lows, closes, volumes, Some(mfi_config));
	let obv_values = indicators_core::on_balance_volume(closes, volumes);

	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i == 0 {
			0
		} else {
			let obv_increasing = obv_values[i] > obv_values[i - 1];
			let obv_decreasing = obv_values[i] < obv_values[i - 1];
			let mfi_value = mfi_values[i];
			let prev_mfi = mfi_values[i - 1];

			// Trigger: MFI exiting oversold zone AND OBV confirms direction
			if obv_increasing && prev_mfi <= oversold && mfi_value > oversold {
				1
			} else if obv_decreasing && prev_mfi >= overbought && mfi_value < overbought {
				-1
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}

pub fn mfi_obv_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "mfi-obv-volume-flow",
		"name": "MFI + OBV Volume Flow",
		"category": "composite",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Combines MFI (Money Flow Index) + OBV volume confirmation"
	})
}

pub fn mfi_obv_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"mfi_period": 14,
			"oversold": 20.0,
			"overbought": 80.0
		},
		"optimization_bounds": [
			{
				"param_name": "mfi_period",
				"min": 5.0,
				"max": 30.0,
				"step": 1.0
			},
			{
				"param_name": "oversold",
				"min": 10.0,
				"max": 30.0,
				"step": 5.0
			},
			{
				"param_name": "overbought",
				"min": 70.0,
				"max": 90.0,
				"step": 5.0
			}
		]
	})
}
