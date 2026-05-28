use indicators::ce;

#[test]
fn test_chandelier_exit_without_config() {
	let highs = vec![10.0, 9.0, 12.0, 14.0, 12.0];
	let lows = vec![6.0, 7.0, 9.0, 12.0, 10.0];
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];

	let result = ce(highs.into(), lows.into(), closings.into(), None).unwrap();

	assert_eq!(result.long.len(), 5);
	assert_eq!(result.short.len(), 5);
}

#[test]
fn test_chandelier_exit_empty_arrays() {
	let highs: Vec<f64> = vec![];
	let lows: Vec<f64> = vec![];
	let closings: Vec<f64> = vec![];

	let result = ce(highs.into(), lows.into(), closings.into(), None);
	assert!(result.is_err());
}

#[test]
fn test_chandelier_exit_mismatched_lengths() {
	let highs = vec![1.0, 2.0];
	let lows = vec![1.0];
	let closings = vec![1.0];

	let result = ce(highs.into(), lows.into(), closings.into(), None);
	assert!(result.is_err());
}
