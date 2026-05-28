use crate::types::configs::ZScoreReversionConfig;

/// Z Score Reversion
///
/// Generates buy/sell signals based on volatility channel breakouts and mean reversion.
pub fn z_score_reversion_strategy(
	closes: &[f64],
	config: Option<ZScoreReversionConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20);
	let threshold = config.threshold.unwrap_or(2.0);

	if !(2..=100).contains(&period) {
		return Err("Z-Score Reversion period must be between 2 and 100".to_string());
	}
	if !(0.1..=10.0).contains(&threshold) {
		return Err("Z-Score Reversion threshold must be between 0.1 and 10.0".to_string());
	}

	let data_len = closes.len();
	let z_config = indicators_core::ZScoreConfig {
		period: Some(period),
	};
	let z_arr = indicators_core::z_score(closes, Some(z_config))?;
	let mut signals = Vec::with_capacity(data_len);

	for (i, &z) in z_arr.iter().enumerate().take(data_len) {
		let signal = if i < period as usize {
			0
		} else if z < -threshold {
			1
		} else if z > threshold {
			-1
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}

pub fn z_score_reversion_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "zScoreReversion",
		"name": "Z-Score Reversion Strategy",
		"category": "volatility",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when z-score is below negative threshold and sell signals when it exceeds positive threshold"
	})
}

pub fn z_score_reversion_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 20,
			"threshold": 2.0
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 5.0,
				"max": 50.0,
				"step": 1.0
			},
			{
				"param_name": "threshold",
				"min": 0.1,
				"max": 5.0,
				"step": 0.1
			}
		]
	})
}
