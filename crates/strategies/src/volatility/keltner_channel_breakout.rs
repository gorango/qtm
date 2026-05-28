use crate::types::configs::KeltnerChannelConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};
use crate::{StrategyError, StrategyResult};

/// Keltner Channel Breakout
///
/// Generates buy/sell signals based on volatility channel breakouts and mean reversion.
pub fn keltner_channel_breakout_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<KeltnerChannelConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20);

	if !(2..=100).contains(&period) {
		return Err(StrategyError::Validation(
			"Keltner Channel period must be between 2 and 100".into(),
		));
	}

	let data_len = closes.len();
	let kc = indicators_core::keltner_channel(highs, lows, closes, Some(period))?;
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < period as usize {
			0
		} else {
			let closes_vec = closes;

			if crossed_over_series(closes_vec, &kc.upper, i as u32) {
				1 // Buy signal: price crosses over upper band (breakout)
			} else if crossed_under_series(closes_vec, &kc.lower, i as u32) {
				-1 // Sell signal: price crosses under lower band (breakdown)
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}

pub fn keltner_channel_breakout_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "keltnerChannelBreakout",
		"name": "Keltner Channel Breakout Strategy",
		"category": "volatility",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when price crosses over upper Keltner channel and sell signals when price crosses under lower channel"
	})
}

pub fn keltner_channel_breakout_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 20
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 5.0,
				"max": 50.0,
				"step": 1.0
			}
		]
	})
}
