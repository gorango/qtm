use indicators::kc;

#[test]
fn test_keltner_channel_with_config() {
	let highs = vec![10.0, 9.0, 12.0, 14.0, 12.0];
	let lows = vec![6.0, 7.0, 9.0, 12.0, 10.0];
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];

	let result = kc(highs.into(), lows.into(), closings.into(), Some(14)).unwrap();

	assert_eq!(result.middle.len(), 5);
	assert_eq!(result.upper.len(), 5);
	assert_eq!(result.lower.len(), 5);

	for value in &result.middle {
		assert!(value.is_nan() || value.is_finite());
	}

	for value in &result.upper {
		assert!(value.is_nan() || value.is_finite());
	}

	for value in &result.lower {
		assert!(value.is_nan() || value.is_finite());
	}
}

#[test]
fn test_keltner_channel_without_config() {
	let highs = vec![10.0, 9.0, 12.0, 14.0, 12.0];
	let lows = vec![6.0, 7.0, 9.0, 12.0, 10.0];
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];

	let result = kc(highs.into(), lows.into(), closings.into(), None).unwrap();

	assert_eq!(result.middle.len(), 5);
	assert_eq!(result.upper.len(), 5);
	assert_eq!(result.lower.len(), 5);

	for value in &result.middle {
		assert!(value.is_nan() || value.is_finite());
	}

	for value in &result.upper {
		assert!(value.is_nan() || value.is_finite());
	}

	for value in &result.lower {
		assert!(value.is_nan() || value.is_finite());
	}
}

#[test]
fn test_keltner_channel_mismatched_lengths() {
	let highs = vec![1.0, 2.0];
	let lows = vec![1.0];
	let closings = vec![1.0];

	let result = kc(highs.into(), lows.into(), closings.into(), None);
	assert!(result.is_err());
}
