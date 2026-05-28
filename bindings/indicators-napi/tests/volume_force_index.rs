use indicators::{fi, force_index};
use indicators_core::FIConfig;

#[test]
fn test_fi_with_period_1() {
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = fi(
		closings.into(),
		volumes.into(),
		Some(FIConfig { period: Some(1) }),
	);

	assert_eq!(result.len(), 5);
}

#[test]
fn test_fi_values_period_1() {
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = fi(
		closings.into(),
		volumes.into(),
		Some(FIConfig { period: Some(1) }),
	);

	assert!((result[0] - 900.0).abs() < 0.01);
	assert!((result[1] - 220.0).abs() < 0.01);
	assert!((result[2] - -320.0).abs() < 0.01);
	assert!((result[3] - 360.0).abs() < 0.01);
	assert!((result[4] - -180.0).abs() < 0.01);
}

#[test]
fn test_fi_default_config() {
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = fi(closings.into(), volumes.into(), None);

	assert_eq!(result.len(), 5);
	for value in result {
		assert!(value.is_nan() || value.is_finite());
	}
}

#[test]
fn test_force_index_alias() {
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = force_index(
		closings.into(),
		volumes.into(),
		Some(FIConfig { period: Some(1) }),
	);

	assert_eq!(result.len(), 5);
}

#[test]
#[should_panic]
fn test_fi_mismatched_lengths() {
	let closings = vec![1.0, 2.0, 3.0];
	let volumes = vec![1.0, 2.0];
	let _result = fi(closings.into(), volumes.into(), None);
}

#[test]
fn test_fi_empty_arrays() {
	let closings: Vec<f64> = vec![];
	let volumes: Vec<f64> = vec![];
	let result = fi(closings.into(), volumes.into(), None);

	assert_eq!(result.len(), 0);
}

#[test]
fn test_fi_single_value() {
	let closings = vec![10.0];
	let volumes = vec![100.0];
	let result = fi(
		closings.into(),
		volumes.into(),
		Some(FIConfig { period: Some(1) }),
	);

	assert_eq!(result.len(), 1);
	assert!(result[0].is_finite());
}

#[test]
fn test_fi_period_2() {
	let closings = vec![10.0, 11.0, 12.0, 13.0, 14.0];
	let volumes = vec![100.0, 100.0, 100.0, 100.0, 100.0];
	let result = fi(
		closings.into(),
		volumes.into(),
		Some(FIConfig { period: Some(2) }),
	);

	assert_eq!(result.len(), 5);
	for value in result {
		assert!(value.is_nan() || value.is_finite());
	}
}

#[test]
fn test_fi_zero_volume() {
	let closings = vec![10.0, 11.0, 12.0];
	let volumes = vec![100.0, 0.0, 100.0];
	let result = fi(
		closings.into(),
		volumes.into(),
		Some(FIConfig { period: Some(1) }),
	);

	assert_eq!(result.len(), 3);
	assert!(result[0].is_finite());
	assert_eq!(result[1], 0.0);
	assert!(result[2].is_finite());
}

#[test]
fn test_fi_period_3() {
	let closings = vec![10.0, 11.0, 12.0, 13.0, 14.0];
	let volumes = vec![100.0, 100.0, 100.0, 100.0, 100.0];
	let result = fi(
		closings.into(),
		volumes.into(),
		Some(FIConfig { period: Some(3) }),
	);

	assert_eq!(result.len(), 5);
	for value in result {
		assert!(value.is_nan() || value.is_finite());
	}
}
