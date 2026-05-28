use indicators::bb;

#[test]
fn test_bollinger_bands_without_config() {
	let closings = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0];

	let result = bb(closings.into(), None).unwrap();

	assert_eq!(result.upper.len(), 10);
	assert_eq!(result.middle.len(), 10);
	assert_eq!(result.lower.len(), 10);
}

#[test]
fn test_bollinger_bands_empty_array() {
	let closings: Vec<f64> = vec![];
	let result = bb(closings.into(), None).unwrap();

	assert_eq!(result.upper.len(), 0);
	assert_eq!(result.middle.len(), 0);
	assert_eq!(result.lower.len(), 0);
}
