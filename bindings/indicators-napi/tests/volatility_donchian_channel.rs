use indicators::dc;

#[test]
fn test_dochian_channel_empty_array() {
	let closings: Vec<f64> = vec![];
	let result = dc(closings.into(), None);
	assert!(result.is_err());
}
