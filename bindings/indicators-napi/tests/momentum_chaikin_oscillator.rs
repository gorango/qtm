use indicators::chaikin_oscillator;
use indicators_core::ChaikinOscillatorConfig;

#[test]
fn test_chaikin_oscillator_with_config() {
	let highs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0];
	let lows = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
	let closings = vec![5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0];
	let volumes = vec![100.0, 200.0, 300.0, 400.0, 500.0, 600.0, 700.0, 800.0];

	let result = chaikin_oscillator(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(ChaikinOscillatorConfig {
			fast_period: Some(2),
			slow_period: Some(5),
		}),
	);

	assert_eq!(result.ad_result.len(), 8);
	assert_eq!(result.cmo_result.len(), 8);

	for value in &result.ad_result {
		assert!(value.is_finite());
	}

	for value in &result.cmo_result {
		assert!(value.is_nan() || value.is_finite());
	}
}

#[test]
fn test_chaikin_oscillator_without_config() {
	let highs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0];
	let lows = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
	let closings = vec![5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0];
	let volumes = vec![100.0, 200.0, 300.0, 400.0, 500.0, 600.0, 700.0, 800.0];

	let result = chaikin_oscillator(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		None,
	);

	assert_eq!(result.ad_result.len(), 8);
	assert_eq!(result.cmo_result.len(), 8);

	for value in &result.ad_result {
		assert!(value.is_finite());
	}

	for value in &result.cmo_result {
		assert!(value.is_nan() || value.is_finite());
	}
}
