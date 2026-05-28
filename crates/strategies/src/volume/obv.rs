use crate::types::configs::OBVConfig;

/// OBV Divergence Strategy
///
/// Generates buy signals when price makes lower lows while OBV makes higher lows (bullish divergence)
/// Generates sell signals when price makes higher highs while OBV makes lower highs (bearish divergence)
///
/// @strategy_id obv
/// @strategy_name OBV Divergence
/// @category volume
/// @default_timeframes 15m,1h,4h
pub fn obv_strategy(
	closes: &[f64],
	volumes: &[f64],
	config: Option<OBVConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let lookback_period = config.lookback_period.unwrap_or(20) as usize;

	if closes.len() != volumes.len() {
		return Err("Closes and volumes must have equal length".to_string());
	}
	if !(10..=50).contains(&(lookback_period as u32)) {
		return Err("Lookback period must be between 10 and 50".to_string());
	}
	let data_len = closes.len();
	if data_len < 2 * lookback_period + 1 {
		return Err("Insufficient data for OBV strategy".to_string());
	}

	let closes_vec: Vec<f64> = closes.to_vec();
	let volumes_vec: Vec<f64> = volumes.to_vec();
	let obv_values = indicators_core::on_balance_volume(&closes_vec, &volumes_vec);

	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		let signal = if i < 2 * lookback_period {
			0
		} else {
			let current_start = i - lookback_period + 1;
			let previous_start = i - 2 * lookback_period + 1;

			let current_prices = &closes_vec[current_start..=i];
			let previous_prices = &closes_vec[previous_start..current_start];
			let current_obvs = &obv_values[current_start..=i];
			let previous_obvs = &obv_values[previous_start..current_start];

			let current_price_min = *current_prices
				.iter()
				.min_by(|a, b| a.partial_cmp(b).unwrap())
				.unwrap_or(&f64::INFINITY);
			let previous_price_min = *previous_prices
				.iter()
				.min_by(|a, b| a.partial_cmp(b).unwrap())
				.unwrap_or(&f64::INFINITY);
			let current_obv_min = *current_obvs
				.iter()
				.min_by(|a, b| a.partial_cmp(b).unwrap())
				.unwrap_or(&f64::INFINITY);
			let previous_obv_min = *previous_obvs
				.iter()
				.min_by(|a, b| a.partial_cmp(b).unwrap())
				.unwrap_or(&f64::INFINITY);

			let current_price_max = *current_prices
				.iter()
				.max_by(|a, b| a.partial_cmp(b).unwrap())
				.unwrap_or(&f64::NEG_INFINITY);
			let previous_price_max = *previous_prices
				.iter()
				.max_by(|a, b| a.partial_cmp(b).unwrap())
				.unwrap_or(&f64::NEG_INFINITY);
			let current_obv_max = *current_obvs
				.iter()
				.max_by(|a, b| a.partial_cmp(b).unwrap())
				.unwrap_or(&f64::NEG_INFINITY);
			let previous_obv_max = *previous_obvs
				.iter()
				.max_by(|a, b| a.partial_cmp(b).unwrap())
				.unwrap_or(&f64::NEG_INFINITY);

			let bullish_divergence =
				current_price_min < previous_price_min && current_obv_min > previous_obv_min;
			let bearish_divergence =
				current_price_max > previous_price_max && current_obv_max < previous_obv_max;

			if bullish_divergence {
				1
			} else if bearish_divergence {
				-1
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}

/// Get OBV strategy metadata for registry
pub fn obv_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "obv",
		"name": "OBV Divergence",
		"category": "volume",
		"default_timeframes": ["15m", "1h", "4h"],
		"description": "Generates buy signals on bullish divergence (price lower low, OBV higher low) and sell signals on bearish divergence (price higher high, OBV lower high)"
	})
}

/// Get OBV strategy default parameters
pub fn obv_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"lookback_period": 20
		},
		"optimization_bounds": [
			{
				"param_name": "lookback_period",
				"min": 10.0,
				"max": 50.0,
				"step": 1.0
			}
		]
	})
}
