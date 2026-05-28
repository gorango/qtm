use indicators::awesome_oscillator;
use indicators_core::AwesomeOscillatorConfig;

#[test]
fn test_awesome_oscillator_with_config() {
	let highs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0];
	let lows = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

	let result = awesome_oscillator(
		highs.into(),
		lows.into(),
		Some(AwesomeOscillatorConfig {
			fast_period: Some(5),
			slow_period: Some(34),
		}),
	)
	.unwrap();

	assert_eq!(result.len(), 8);
	for value in &result {
		assert!(value.is_nan() || value.is_finite());
	}
}

#[test]
fn test_awesome_oscillator_without_config() {
	let highs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0];
	let lows = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

	let result = awesome_oscillator(highs.into(), lows.into(), None).unwrap();

	assert_eq!(result.len(), 8);
	for value in &result {
		assert!(value.is_nan() || value.is_finite());
	}
}

#[test]
fn test_awesome_oscillator_mismatched_lengths() {
	let highs = vec![1.0, 2.0, 3.0];
	let lows = vec![1.0, 2.0];

	let result = awesome_oscillator(highs.into(), lows.into(), None);
	assert!(result.is_err());
}

#[test]
fn test_awesome_oscillator_empty_arrays() {
	let highs: Vec<f64> = vec![];
	let lows: Vec<f64> = vec![];

	let result = awesome_oscillator(highs.into(), lows.into(), None).unwrap();
	assert_eq!(result.len(), 0);
}
