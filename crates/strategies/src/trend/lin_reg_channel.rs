use crate::types::configs::LinRegChannelConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};
use crate::{StrategyError, StrategyResult};

/// Linear Regression Channel Trend Strategy
///
/// Generates buy signals when price crosses above the linear regression line
/// Generates sell signals when price crosses below the linear regression line
///
/// @strategy_id linRegChannel
/// @strategy_name Linear Regression Channel
/// @category trend
/// @default_timeframes 1h,4h,1d
pub fn lin_reg_channel_strategy(
	closes: &[f64],
	config: Option<LinRegChannelConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20);
	let offset = config.offset.unwrap_or(0.0);

	// Validate parameters
	if !(2..=200).contains(&period) {
		return Err(StrategyError::Validation(
			"Linear Regression Channel period must be between 2 and 200".into(),
		));
	}

	let data_len = closes.len();
	if data_len < period as usize {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for Linear Regression Channel strategy".into(),
		));
	}

	// Convert Float64Array to Vec<f64>
	let closes_vec: Vec<f64> = closes.to_vec();

	// Calculate linear regression values
	let linreg_config = indicators_core::LinRegConfig {
		period: Some(period),
		offset: Some(offset as u32),
	};
	let linreg_values = indicators_core::linreg(closes, Some(linreg_config))?;

	// Generate signals based on price crossing regression line
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		if i < (period - 1) as usize {
			signals.push(0);
			continue;
		}

		let signal = if crossed_over_series(&closes_vec, &linreg_values, i as u32) {
			1 // Buy signal
		} else if crossed_under_series(&closes_vec, &linreg_values, i as u32) {
			-1 // Sell signal
		} else {
			0 // Hold
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get Linear Regression Channel strategy metadata for registry
pub fn lin_reg_channel_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "linRegChannel",
		"name": "Linear Regression Channel",
		"category": "trend",
		"default_timeframes": ["1h", "4h", "1d"],
		"description": "Generates buy signals when price crosses above the linear regression line and sell signals when price crosses below"
	})
}

/// Get Linear Regression Channel strategy default parameters
pub fn lin_reg_channel_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 20,
			"offset": 0.0
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 5.0,
				"max": 50.0,
				"step": 1.0
			},
			{
				"param_name": "offset",
				"min": -2.0,
				"max": 2.0,
				"step": 0.1
			}
		]
	})
}
