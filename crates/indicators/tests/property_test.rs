use proptest::prelude::*;

proptest! {
	#[test]
	fn test_sma_output_len_equals_input_len(values in proptest::collection::vec(-1e6f64..1e6, 1..100), period in 1usize..30) {
		let result = indicators_core::sma_internal(&values, period);
		assert_eq!(result.len(), values.len());
	}

	#[test]
	fn test_ema_output_len_equals_input_len(values in proptest::collection::vec(-1e6f64..1e6, 1..100), period in 1usize..30) {
		let result = indicators_core::ema_internal(&values, period);
		assert_eq!(result.len(), values.len());
	}

	#[test]
	fn test_sma_first_valid_index(values in proptest::collection::vec(1.0f64..1000.0, 5..50), period in 2usize..10) {
		let result = indicators_core::sma_internal(&values, period);
		// First (period-1) values should be NaN
		for r in result.iter().take((period - 1).min(result.len())) {
			assert!(r.is_nan());
		}
		// From period-1 onwards, values should be finite
		for r in result.iter().skip(period - 1) {
			assert!(r.is_finite());
		}
	}
}
