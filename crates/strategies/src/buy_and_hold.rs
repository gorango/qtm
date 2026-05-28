use crate::types::configs::BuyAndHoldConfig;

/// Buy and Hold Strategy
///
/// Buy on first bar, hold forever
///
/// @strategy_id buyAndHold
/// @strategy_name Buy and Hold
/// @category special
/// @default_timeframes 1d,1w,1M
pub fn buy_and_hold_strategy(
	closes: &[f64],
	config: Option<BuyAndHoldConfig>,
) -> Result<Vec<i8>, String> {
	let _config = config.unwrap_or_default();

	let data_len = closes.len();
	if data_len == 0 {
		return Err("Input arrays cannot be empty".to_string());
	}

	// Generate signals: buy on first bar, hold forever
	let mut signals = vec![0; data_len];
	if data_len > 0 {
		signals[0] = 1; // Buy on first bar
	}

	Ok(signals)
}

/// Get Buy and Hold strategy metadata for registry
pub fn buy_and_hold_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "buyAndHold",
		"name": "Buy and Hold",
		"category": "special",
		"default_timeframes": ["1d", "1w", "1M"],
		"description": "Buy on first bar and hold position forever"
	})
}

/// Get Buy and Hold strategy default parameters
pub fn buy_and_hold_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {},
		"optimization_bounds": []
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty_input_returns_error() {
		let closes: Vec<f64> = vec![];
		let result = buy_and_hold_strategy(&closes, None);
		assert!(result.is_err());
		assert_eq!(result.err().unwrap(), "Input arrays cannot be empty");
	}

	#[test]
	fn single_element_signal_is_one() {
		let closes = vec![100.0];
		let result = buy_and_hold_strategy(&closes, None).unwrap();
		assert_eq!(result.len(), 1);
		assert_eq!(result[0], 1);
	}

	#[test]
	fn only_first_element_is_one_rest_are_zero() {
		let closes = vec![100.0, 101.0, 102.0, 103.0, 104.0];
		let result = buy_and_hold_strategy(&closes, None).unwrap();
		assert_eq!(result.len(), 5);
		assert_eq!(result[0], 1);
		for i in 1..5 {
			assert_eq!(result[i], 0, "signal[{}] should be 0", i);
		}
	}

	#[test]
	fn config_default_works() {
		let config = BuyAndHoldConfig::default();
		let result = buy_and_hold_strategy(&[100.0, 101.0], Some(config)).unwrap();
		assert_eq!(result[0], 1);
		assert_eq!(result[1], 0);
	}
}
