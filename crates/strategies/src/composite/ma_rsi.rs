use crate::types::configs::MaRsiConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};

/// Ma Rsi
///
/// Buy when price above MA and RSI oversold. Sell when below MA and RSI overbought.
pub fn ma_rsi_strategy(closes: &[f64], config: Option<MaRsiConfig>) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let ma_period = config.ma_period.unwrap_or(20);
	let rsi_period = config.rsi_period.unwrap_or(14);
	let oversold = config.oversold.unwrap_or(30.0);
	let overbought = config.overbought.unwrap_or(70.0);

	let data_len = closes.len();
	let min_periods = ma_period.max(rsi_period) as usize;

	if data_len < min_periods {
		return Err(format!(
			"Insufficient data: MA + RSI requires at least {} data points, got {}",
			min_periods, data_len
		));
	}

	let closes_vec: Vec<f64> = closes.to_vec();
	let ma_values = indicators_core::sma(&closes_vec, Some(ma_period)).unwrap();

	let rsi_config = indicators_core::RSIConfig {
		period: Some(rsi_period),
	};
	let rsi_values = indicators_core::rsi(&closes_vec, Some(rsi_config));

	let mut signals = Vec::with_capacity(data_len);

	for (i, &rsi_value) in rsi_values.iter().enumerate().take(data_len) {
		let signal = if i < min_periods {
			0
		} else {
			let crossed_over_ma = crossed_over_series(&closes_vec, &ma_values, i as u32);
			let crossed_under_ma = crossed_under_series(&closes_vec, &ma_values, i as u32);

			if crossed_over_ma && rsi_value <= oversold {
				1
			} else if crossed_under_ma && rsi_value >= overbought {
				-1
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}

pub fn ma_rsi_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "ma-rsi-trend-following",
		"name": "MA + RSI Trend Following",
		"category": "composite",
		"description": "Combine MA trend + RSI momentum",
		"default_timeframes": ["15m", "1h", "4h"]
	})
}

pub fn ma_rsi_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"ma_period": 20,
			"rsi_period": 14,
			"oversold": 30.0,
			"overbought": 70.0
		},
		"optimization_bounds": [
			{
				"param_name": "ma_period",
				"min": 5.0,
				"max": 50.0,
				"step": 1.0
			},
			{
				"param_name": "rsi_period",
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
