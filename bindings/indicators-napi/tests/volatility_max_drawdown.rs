use indicators::max_drawdown;

#[test]
fn test_max_drawdown_empty_and_single_element() {
	let prices_empty: Vec<f64> = vec![];
	let result_empty = max_drawdown(prices_empty.into(), 3).unwrap();
	assert_eq!(result_empty.len(), 0);

	let prices_single = vec![100.0];
	let result_single = max_drawdown(prices_single.into(), 3).unwrap();
	assert_eq!(result_single.len(), 1);
	assert_eq!(result_single[0], 0.0);
}
