use crate::signals::{crossed_over_series, crossed_under_series};
use crate::types::configs::BollingerBandsConfig;
use crate::{StrategyError, StrategyResult};

/// Bollinger Bands Breakout
///
/// Generates buy/sell signals based on volatility channel breakouts and mean reversion.
pub fn bollinger_bands_breakout_strategy(
	closes: &[f64],
	config: Option<BollingerBandsConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20);
	let std_dev = config.std_dev.unwrap_or(2.0);

	if !(2..=100).contains(&period) {
		return Err(StrategyError::Validation(
			"Bollinger Bands Breaknout period must be between 2 and 100".into(),
		));
	}
	if !(0.1..=5.0).contains(&std_dev) {
		return Err(StrategyError::Validation(
			"Bollinger Bands Breaknout std_dev must be between 0.1 and 5.0".into(),
		));
	}

	let data_len = closes.len();
	let bb_config = indicators_core::BBConfig {
		period: Some(period),
		std_dev: Some(std_dev),
	};
	let bb = indicators_core::bollinger_bands(closes, Some(bb_config))?;

	let mut signals = Vec::with_capacity(data_len);
	let closes_vec = closes; // Need vec for indexing if not using slices yet

	for i in 0..data_len {
		let signal = if i < period as usize {
			0
		} else {
			// 1. Buy when price crosses OVER the Upper Band (Breakout)
			if crossed_over_series(closes_vec, &bb.upper, i as u32) {
				1
			}
			// 2. Sell when price crosses UNDER the Lower Band (Breakdown)
			else if crossed_under_series(closes_vec, &bb.lower, i as u32) {
				-1
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}

pub fn bollinger_bands_breakout_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "bollingerBands",
		"name": "Bollinger Bands Strategy",
		"category": "volatility",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when price crosses above upper band and sell signals when price crosses below lower band"
	})
}

pub fn bollinger_bands_breakout_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 20,
			"stdDev": 2.0
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 5.0,
				"max": 50.0,
				"step": 1.0
			},
			{
				"param_name": "stdDev",
				"min": 1.0,
				"max": 3.0,
				"step": 0.1
			}
		]
	})
}
