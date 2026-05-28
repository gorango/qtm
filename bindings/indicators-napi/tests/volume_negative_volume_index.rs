use indicators::{negative_volume_index, nvi};

#[test]
fn test_nvi_with_start() {
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = nvi(closings.into(), volumes.into(), Some(500.0));

	assert_eq!(result.len(), 5);
}

#[test]
fn test_nvi_values_with_start() {
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = nvi(closings.into(), volumes.into(), Some(500.0));

	assert!((result[0] - 500.0).abs() < 0.01);
	assert!((result[1] - 500.0).abs() < 0.01);
	assert!((result[2] - 318.18).abs() < 0.01);
	assert!((result[3] - 318.18).abs() < 0.01);
	assert!((result[4] - 254.55).abs() < 0.01);
}

#[test]
fn test_nvi_default_start() {
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = nvi(closings.into(), volumes.into(), None);

	assert_eq!(result.len(), 5);
	assert!((result[0] - 1000.0).abs() < 0.01);
	assert!((result[1] - 1000.0).abs() < 0.01);
	assert!((result[2] - 636.36).abs() < 0.01);
	assert!((result[3] - 636.36).abs() < 0.01);
	assert!((result[4] - 509.09).abs() < 0.01);
}

#[test]
fn test_negative_volume_index_alias() {
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = negative_volume_index(closings.into(), volumes.into(), Some(500.0));

	assert_eq!(result.len(), 5);
}

#[test]
#[should_panic]
fn test_nvi_mismatched_lengths() {
	let closings = vec![1.0, 2.0, 3.0];
	let volumes = vec![1.0, 2.0];
	let _result = nvi(closings.into(), volumes.into(), Some(1000.0));
}

#[test]
#[should_panic]
fn test_nvi_empty_arrays() {
	let closings: Vec<f64> = vec![];
	let volumes: Vec<f64> = vec![];
	let _result = nvi(closings.into(), volumes.into(), Some(1000.0));
}

#[test]
fn test_nvi_single_value() {
	let closings = vec![10.0];
	let volumes = vec![100.0];
	let result = nvi(closings.into(), volumes.into(), Some(1000.0));

	assert_eq!(result.len(), 1);
	assert!((result[0] - 1000.0).abs() < 0.01);
}

#[test]
fn test_nvi_all_increasing_volume() {
	let closings = vec![10.0, 11.0, 12.0, 13.0, 14.0];
	let volumes = vec![100.0, 110.0, 120.0, 130.0, 140.0];
	let result = nvi(closings.into(), volumes.into(), Some(1000.0));

	assert_eq!(result.len(), 5);
	assert_eq!(result[0], 1000.0);
	assert_eq!(result[1], 1000.0);
	assert_eq!(result[2], 1000.0);
	assert_eq!(result[3], 1000.0);
	assert_eq!(result[4], 1000.0);
}

#[test]
fn test_nvi_custom_start() {
	let closings = vec![10.0, 11.0, 9.0];
	let volumes = vec![100.0, 90.0, 80.0];
	let result = nvi(closings.into(), volumes.into(), Some(100.0));

	assert_eq!(result.len(), 3);
	assert_eq!(result[0], 100.0);
	assert!(result[1] > result[0]);
	assert!(result[2] < result[1]);
}

#[test]
fn test_nvi_all_decreasing_volume() {
	let closings = vec![10.0, 11.0, 12.0, 13.0, 14.0];
	let volumes = vec![140.0, 130.0, 120.0, 110.0, 100.0];
	let result = nvi(closings.into(), volumes.into(), Some(1000.0));

	assert_eq!(result[0], 1000.0);
	assert!((result[1] - 1100.0).abs() < 0.01);
	assert!((result[2] - 1200.0).abs() < 0.01);
	assert!((result[3] - 1300.0).abs() < 0.01);
	assert!((result[4] - 1400.0).abs() < 0.01);
}
