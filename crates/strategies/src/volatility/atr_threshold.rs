use crate::types::configs::AtrThresholdConfig;

/// Atr Threshold
///
/// Generates buy/sell signals based on volatility channel breakouts and mean reversion.
pub fn atr_threshold_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<AtrThresholdConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14);
	let multiplier = config.multiplier.unwrap_or(2.0);

	if !(2..=100).contains(&period) {
		return Err("ATR Threshold period must be between 2 and 100".to_string());
	}
	if !(0.1..=10.0).contains(&multiplier) {
		return Err("ATR Threshold multiplier must be between 0.1 and 10.0".to_string());
	}

	let data_len = closes.len();
	let highs_vec = highs;
	let lows_vec = lows;
	let atr_config = indicators_core::ATRConfig {
		period: Some(period),
	};
	let atr = indicators_core::average_true_range(highs, lows, closes, Some(atr_config))?;
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < period as usize {
			0
		} else {
			let current_atr = atr.atr_line[i];
			let range = highs_vec[i] - lows_vec[i];
			let atr_threshold = current_atr * multiplier;

			if range < atr_threshold {
				1
			} else if range > atr_threshold {
				-1
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}

pub fn atr_threshold_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "atrThreshold",
		"name": "ATR Threshold Strategy",
		"category": "volatility",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates signals based on ATR threshold comparison with price range"
	})
}

pub fn atr_threshold_strategy_defaults() -> serde_json::Value {
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
