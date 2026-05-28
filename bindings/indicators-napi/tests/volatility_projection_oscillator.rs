use indicators::po;

#[test]
fn test_projection_oscillator_empty_arrays() {
	let highs: Vec<f64> = vec![];
	let lows: Vec<f64> = vec![];
	let closings: Vec<f64> = vec![];

	let result = po(highs.into(), lows.into(), closings.into(), None, None);
	assert!(result.is_err());
}

#[test]
fn test_projection_oscillator_mismatched_lengths() {
	let highs = vec![1.0, 2.0];
	let lows = vec![1.0];
	let closings = vec![1.0];

	let result = po(highs.into(), lows.into(), closings.into(), None, None);
	assert!(result.is_err());
}
