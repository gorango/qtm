use crate::types::configs::BuyAndHoldConfig;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Buy and Hold Strategy
///
/// Buy on first bar, hold forever
#[strategy(
	id = "buy_and_hold",
	name = "Buy and Hold",
	category = "special",
	default_timeframes = ["1d", "1w", "1M"],
	description = "Buy on first bar and hold position forever"
)]
pub fn buy_and_hold_strategy(
	closes: &[f64],
	config: Option<BuyAndHoldConfig>,
) -> StrategyResult<Vec<i8>> {
	let _config = config.unwrap_or_default();

	let data_len = closes.len();
	if data_len == 0 {
		return Err(StrategyError::Validation(
			"Input arrays cannot be empty".into(),
		));
	}

	// Generate signals: buy on first bar, hold forever
	let mut signals = vec![0; data_len];
	if data_len > 0 {
		signals[0] = 1; // Buy on first bar
	}

	Ok(signals)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty_input_returns_error() {
		let closes: Vec<f64> = vec![];
		let result = buy_and_hold_strategy(&closes, None);
		assert!(result.is_err());
		assert!(result
			.err()
			.unwrap()
			.to_string()
			.contains("Input arrays cannot be empty"));
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
			assert_eq!(result[i], 0, "signal[{i}] should be 0");
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
