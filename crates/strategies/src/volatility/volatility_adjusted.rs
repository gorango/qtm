use crate::types::configs::VolatilityAdjustedConfig;

/// Volatility Adjusted
///
/// Generates buy/sell signals based on volatility channel breakouts and mean reversion.
pub fn volatility_adjusted_strategy(
	closes: &[f64],
	config: Option<VolatilityAdjustedConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20);
	let target_volatility = config.target_volatility.unwrap_or(0.15);

	if !(2..=252).contains(&period) {
		return Err("Volatility Adjusted period must be between 2 and 252".to_string());
	}
	if !(0.01..=1.0).contains(&target_volatility) {
		return Err(
			"Volatility Adjusted target_volatility must be between 0.01 and 1.0".to_string(),
		);
	}

	let data_len = closes.len();
	let vol_arr = indicators_core::annualized_volatility(closes, Some(period))?;
	let mut signals = Vec::with_capacity(data_len);

	for (i, &vol) in vol_arr.iter().enumerate().take(data_len) {
		let signal = if i < period as usize {
			0
		} else if vol < target_volatility {
			1
		} else if vol > target_volatility {
			-1
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}

pub fn volatility_adjusted_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "volatilityAdjusted",
		"name": "Volatility Adjusted Strategy",
		"category": "volatility",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals when annualized volatility is below target and sell signals when it exceeds target"
	})
}

pub fn volatility_adjusted_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"period": 20,
			"targetVolatility": 0.15
		},
		"optimization_bounds": [
			{
				"param_name": "period",
				"min": 5.0,
				"max": 50.0,
				"step": 1.0
			},
			{
				"param_name": "targetVolatility",
				"min": 0.05,
				"max": 0.5,
				"step": 0.01
			}
		]
	})
}
