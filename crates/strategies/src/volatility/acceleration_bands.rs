use crate::types::configs::AccelerationBandsConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};
use crate::{StrategyError, StrategyResult};

/// Acceleration Bands
///
/// Generates buy/sell signals based on volatility channel breakouts and mean reversion.
pub fn acceleration_bands_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<AccelerationBandsConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20);
	let multiplier = config.multiplier.unwrap_or(4.0);

	if !(2..=100).contains(&period) {
		return Err(StrategyError::Validation(
			"Acceleration Bands period must be between 2 and 100".into(),
		));
	}
	if !(0.1..=10.0).contains(&multiplier) {
		return Err(StrategyError::Validation(
			"Acceleration Bands multiplier must be between 0.1 and 10.0".into(),
		));
	}

	let data_len = closes.len();
	let ab =
		indicators_core::acceleration_bands(highs, lows, closes, Some(period), Some(multiplier))?;
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < period as usize {
			0
		} else {
			let closes_vec = closes;

			if crossed_under_series(closes_vec, &ab.lower, i as u32) {
				1 // Buy signal: price crosses under lower band
			} else if crossed_over_series(closes_vec, &ab.upper, i as u32) {
				-1 // Sell signal: price crosses over upper band
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}

pub fn acceleration_bands_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "accelerationBands",
		"name": "Acceleration Bands Strategy",
		"category": "volatility",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when price crosses under lower band and sell signals when price crosses over upper band"
	})
}

pub fn acceleration_bands_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 20,
			"multiplier": 4.0
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
