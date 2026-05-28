use indicators::rsi;
use indicators_core::RSIConfig;

#[test]
fn test_rsi_with_config() {
	let closings = vec![10.0, 9.0, 11.0, 10.0, 12.0];

	let result = rsi(closings.into(), Some(RSIConfig { period: Some(2) }));

	assert_eq!(result.len(), 5);
}

#[test]
fn test_rsi_without_config() {
	let closings = vec![10.0, 9.0, 11.0, 10.0, 12.0];

	let result = rsi(closings.into(), None);

	assert_eq!(result.len(), 5);
}

#[test]
fn test_rsi_empty_array() {
	let closings: Vec<f64> = vec![];
	let result = rsi(closings.into(), None);
	assert_eq!(result.len(), 0);
}

#[test]
fn test_rsi_values_between_0_and_100() {
	let closings = vec![10.0, 9.0, 11.0, 10.0, 12.0];
	let result = rsi(closings.into(), Some(RSIConfig { period: Some(2) }));

	for &value in &result {
		if !value.is_nan() {
			assert!(value >= 0.0);
			assert!(value <= 100.0);
		}
	}
}
