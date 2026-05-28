use indicators::ulcer_index;

#[test]
fn test_ulcer_index_empty_array() {
	let closings: Vec<f64> = vec![];
	let result = ulcer_index(closings.into(), None);
	assert!(result.is_err());
}
