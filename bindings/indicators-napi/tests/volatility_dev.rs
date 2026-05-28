use indicators::dev;
use indicators_core::MeanAbsoluteDeviationConfig;

#[test]
fn test_dev_without_config() {
	let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

	let result = dev(values.into(), None).unwrap();

	assert_eq!(result.len(), 10);
}

#[test]
fn test_dev_empty_array() {
	let values: Vec<f64> = vec![];
	let result = dev(values.into(), None).unwrap();
	assert_eq!(result.len(), 0);
}

#[test]
fn test_dev_array_shorter_than_period() {
	let short_values = vec![1.0, 2.0, 3.0];
	let result = dev(
		short_values.into(),
		Some(MeanAbsoluteDeviationConfig { period: Some(5) }),
	)
	.unwrap();

	assert_eq!(result.len(), 3);
	for value in &result {
		assert!(value.is_nan());
	}
}
