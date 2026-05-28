use crate::types::configs::MadReversionConfig;
use crate::{StrategyError, StrategyResult};

/// Mad Reversion
///
/// Generates buy/sell signals based on volatility channel breakouts and mean reversion.
pub fn mad_reversion_strategy(
	closes: &[f64],
	config: Option<MadReversionConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20);
	let deviation_multiplier = config.deviation_multiplier.unwrap_or(2.0);

	if !(2..=100).contains(&period) {
		return Err(StrategyError::Validation(
			"MAD Reversion period must be between 2 and 100".into(),
		));
	}
	if !(0.1..=10.0).contains(&deviation_multiplier) {
		return Err(StrategyError::Validation(
			"MAD Reversion deviation_multiplier must be between 0.1 and 10.0".into(),
		));
	}

	let data_len = closes.len();
	let closes_vec = closes;
	let mstd_config = indicators_core::MSTDConfig {
		period: Some(period),
	};
	let std = indicators_core::moving_standard_deviation(closes_vec, Some(mstd_config))?;
	let mean = indicators_core::sma(closes_vec, Some(period))?;
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < period as usize {
			0
		} else {
			let mean_val = mean[i];
			let std_val = std[i];
			let close = closes[i];
			let lower = mean_val - deviation_multiplier * std_val;
			let upper = mean_val + deviation_multiplier * std_val;

			if close < lower {
				1
			} else if close > upper {
				-1
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}

pub fn mad_reversion_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "madReversion",
		"name": "MAD Reversion Strategy",
		"category": "volatility",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when price falls below mean minus deviation and sell signals when price exceeds mean plus deviation"
	})
}

pub fn mad_reversion_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 20,
			"deviationMultiplier": 2.0
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 5.0,
				"max": 50.0,
				"step": 1.0
			},
			{
				"param_name": "deviationMultiplier",
				"min": 1.0,
				"max": 3.0,
				"step": 0.1
			}
		]
	})
}
