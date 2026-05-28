use indicators::{mfi, money_flow_index};
use indicators_core::MFIConfig;

#[test]
fn test_mfi_with_period_2() {
	let highs = vec![10.0, 9.0, 12.0, 14.0, 12.0];
	let lows = vec![6.0, 7.0, 9.0, 12.0, 10.0];
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = mfi(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(MFIConfig {
			period: Some(2),
			price_source: Some("typical".to_string()),
		}),
	);

	assert_eq!(result.len(), 5);
}

#[test]
fn test_mfi_values_period_2() {
	let highs = vec![10.0, 9.0, 12.0, 14.0, 12.0];
	let lows = vec![6.0, 7.0, 9.0, 12.0, 10.0];
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = mfi(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(MFIConfig {
			period: Some(2),
			price_source: Some("typical".to_string()),
		}),
	);

	assert!((result[0] - 100.0).abs() < 0.01);
	assert!((result[1] - 100.0).abs() < 0.01);
	assert!((result[2] - 100.0).abs() < 0.01);
	assert!((result[3] - 100.0).abs() < 0.01);
	assert!((result[4] - 61.54).abs() < 0.01);
}

#[test]
fn test_mfi_default_config() {
	let highs = vec![10.0, 9.0, 12.0, 14.0, 12.0];
	let lows = vec![6.0, 7.0, 9.0, 12.0, 10.0];
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = mfi(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(MFIConfig {
			period: None,
			price_source: None,
		}),
	);

	assert_eq!(result.len(), 5);
	assert!((result[4] - 81.67).abs() < 0.01);
}

#[test]
fn test_money_flow_index_alias() {
	let highs = vec![10.0, 9.0, 12.0, 14.0, 12.0];
	let lows = vec![6.0, 7.0, 9.0, 12.0, 10.0];
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = money_flow_index(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(MFIConfig {
			period: Some(2),
			price_source: None,
		}),
	);

	assert_eq!(result.len(), 5);
}

#[test]
#[should_panic]
fn test_mfi_mismatched_lengths() {
	let highs = vec![1.0, 2.0, 3.0];
	let lows = vec![1.0, 2.0];
	let closings = vec![1.0, 2.0, 3.0];
	let volumes = vec![1.0, 2.0, 3.0];
	let _result = mfi(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(MFIConfig {
			period: Some(14),
			price_source: None,
		}),
	);
}

#[test]
fn test_mfi_empty_arrays() {
	let highs: Vec<f64> = vec![];
	let lows: Vec<f64> = vec![];
	let closings: Vec<f64> = vec![];
	let volumes: Vec<f64> = vec![];
	let result = mfi(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(MFIConfig {
			period: Some(14),
			price_source: None,
		}),
	);

	assert_eq!(result.len(), 0);
}

#[test]
fn test_mfi_single_value() {
	let highs = vec![10.0];
	let lows = vec![6.0];
	let closings = vec![9.0];
	let volumes = vec![100.0];
	let result = mfi(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(MFIConfig {
			period: Some(14),
			price_source: None,
		}),
	);

	assert_eq!(result.len(), 1);
	assert!(result[0].is_finite());
}

#[test]
fn test_mfi_zero_volume() {
	let highs = vec![10.0, 9.0, 12.0];
	let lows = vec![6.0, 7.0, 9.0];
	let closings = vec![9.0, 11.0, 7.0];
	let volumes = vec![100.0, 0.0, 100.0];
	let result = mfi(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(MFIConfig {
			period: Some(14),
			price_source: None,
		}),
	);

	assert_eq!(result.len(), 3);
	for value in result {
		assert!(value.is_finite() || value.is_infinite());
	}
}

#[test]
fn test_mfi_period_14() {
	let highs = vec![10.0, 9.0, 12.0, 14.0, 12.0];
	let lows = vec![6.0, 7.0, 9.0, 12.0, 10.0];
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = mfi(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(MFIConfig {
			period: Some(14),
			price_source: None,
		}),
	);

	assert_eq!(result.len(), 5);
	for value in result {
		assert!(value.is_finite() || value.is_infinite());
	}
}
