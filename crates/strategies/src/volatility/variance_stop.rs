use crate::types::configs::VarianceStopConfig;
use crate::{StrategyError, StrategyResult};

/// Variance Stop
///
/// Generates buy/sell signals based on volatility channel breakouts and mean reversion.
pub fn variance_stop_strategy(
	closes: &[f64],
	config: Option<VarianceStopConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14);
	let multiplier = config.multiplier.unwrap_or(2.0);

	if !(2..=100).contains(&period) {
		return Err(StrategyError::Validation(
			"Variance Stop period must be between 2 and 100".into(),
		));
	}
	if !(0.1..=10.0).contains(&multiplier) {
		return Err(StrategyError::Validation(
			"Variance Stop multiplier must be between 0.1 and 10.0".into(),
		));
	}

	let data_len = closes.len();
	let variance_config = indicators_core::VarianceConfig {
		period: Some(period),
	};
	let var_arr = indicators_core::rolling_variance(closes, Some(variance_config))?;
	let mut signals = Vec::with_capacity(data_len);

	for (i, &var_value) in var_arr.iter().enumerate().take(data_len) {
		let signal = if i < period as usize {
			0
		} else if var_value < multiplier {
			1
		} else if var_value > multiplier {
			-1
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}

pub fn variance_stop_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "varianceStop",
		"name": "Variance Stop Strategy",
		"category": "volatility",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when variance is below multiplier and sell signals when it exceeds multiplier"
	})
}

pub fn variance_stop_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 14,
			"multiplier": 2.0
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 5.0,
				"max": 50.0,
				"step": 1.0
			},
			{
				"param_name": "multiplier",
				"min": 1.0,
				"max": 5.0,
				"step": 0.1
			}
		]
	})
}
