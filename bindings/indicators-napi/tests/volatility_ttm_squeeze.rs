use indicators::ttm_squeeze;

#[test]
fn test_ttm_squeeze_insufficient_data() {
	let highs = vec![100.0];
	let lows = vec![95.0];
	let closings = vec![100.0];

	let result = ttm_squeeze(
		highs.into(),
		lows.into(),
		closings.into(),
		Some(20),
		Some(2.0),
		Some(20),
	)
	.unwrap();

	assert_eq!(result.in_squeeze.len(), 1);
	assert!(!result.in_squeeze[0]);
	assert!(result.breakout[0].is_none());
}
