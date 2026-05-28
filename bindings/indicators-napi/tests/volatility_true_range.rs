use indicators::tr;

#[test]
fn test_true_range_computation() {
	let highs = vec![10.0, 9.0, 12.0, 14.0, 12.0];
	let lows = vec![6.0, 7.0, 9.0, 12.0, 10.0];
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let expected = [4.0, 2.0, 3.0, 7.0, 2.0];

	let result = tr(highs.into(), lows.into(), closings.into()).unwrap();

	assert_eq!(result.tr_line.len(), 5);

	for (&actual, &expected_val) in result.tr_line.iter().zip(expected.iter()) {
		assert!((actual - expected_val).abs() < 0.01);
	}
}

#[test]
fn test_true_range_mismatched_lengths() {
	let highs = vec![1.0, 2.0, 3.0];
	let lows = vec![1.0, 2.0];
	let closings = vec![1.0, 2.0, 3.0];

	let result = tr(highs.into(), lows.into(), closings.into());
	assert!(result.is_err());
}

#[test]
fn test_true_range_empty_arrays() {
	let highs: Vec<f64> = vec![];
	let lows: Vec<f64> = vec![];
	let closings: Vec<f64> = vec![];

	let result = tr(highs.into(), lows.into(), closings.into()).unwrap();

	assert_eq!(result.tr_line.len(), 0);
}
