use crate::types::configs::DonchianTurtleConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};

/// Donchian Reversion
///
/// Generates buy/sell signals based on volatility channel breakouts and mean reversion.
pub fn donchian_reversion_strategy(
	closes: &[f64],
	config: Option<DonchianTurtleConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20);

	if !(2..=100).contains(&period) {
		return Err("Donchian Reversion period must be between 2 and 100".to_string());
	}

	let data_len = closes.len();
	let dc = indicators_core::donchian_channel(closes, Some(period))?;
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < period as usize {
			0
		} else {
			let closes_vec = closes;

			if crossed_over_series(closes_vec, &dc.lower, i as u32) {
				1 // Buy signal: price crosses over lower band (return to mean)
			} else if crossed_under_series(closes_vec, &dc.upper, i as u32) {
				-1 // Sell signal: price crosses under upper band (return to mean)
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}

pub fn donchian_reversion_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "donchianReversion",
		"name": "Donchian Mean Reversion Strategy",
		"category": "volatility",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Counter-trend strategy: generates buy signals when price crosses over lower channel and sell signals when price crosses under upper channel"
	})
}

pub fn donchian_reversion_strategy_defaults() -> serde_json::Value {
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
