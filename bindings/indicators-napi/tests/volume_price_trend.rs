use indicators::{volume_price_trend, vpt};

#[test]
fn test_vpt_basic() {
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = vpt(closings.into(), volumes.into());

	assert_eq!(result.len(), 5);
}

#[test]
fn test_vpt_values() {
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = vpt(closings.into(), volumes.into());

	assert!((result[0] - 0.0).abs() < 0.01);
	assert!((result[1] - 24.44).abs() < 0.01);
	assert!((result[2] - -4.65).abs() < 0.01);
	assert!((result[3] - 46.78).abs() < 0.01);
	assert!((result[4] - 28.78).abs() < 0.01);
}

#[test]
fn test_volume_price_trend_alias() {
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = volume_price_trend(closings.into(), volumes.into());

	assert_eq!(result.len(), 5);
}

#[test]
#[should_panic]
fn test_vpt_mismatched_lengths() {
	let closings = vec![1.0, 2.0, 3.0];
	let volumes = vec![1.0, 2.0];
	let _result = vpt(closings.into(), volumes.into());
}

#[test]
#[should_panic]
fn test_vpt_empty_arrays() {
	let closings: Vec<f64> = vec![];
	let volumes: Vec<f64> = vec![];
	let _result = vpt(closings.into(), volumes.into());
}

#[test]
fn test_vpt_single_value() {
	let closings = vec![10.0];
	let volumes = vec![100.0];
	let result = vpt(closings.into(), volumes.into());

	assert_eq!(result.len(), 1);
	assert_eq!(result[0], 0.0);
}

#[test]
fn test_vpt_all_increasing() {
	let closings = vec![10.0, 11.0, 12.0, 13.0, 14.0];
	let volumes = vec![100.0, 100.0, 100.0, 100.0, 100.0];
	let result = vpt(closings.into(), volumes.into());

	assert_eq!(result[0], 0.0);
	assert!(result[1] > 0.0);
	assert!(result[2] > result[1]);
	assert!(result[3] > result[2]);
	assert!(result[4] > result[3]);
}

#[test]
fn test_vpt_all_decreasing() {
	let closings = vec![14.0, 13.0, 12.0, 11.0, 10.0];
	let volumes = vec![100.0, 100.0, 100.0, 100.0, 100.0];
	let result = vpt(closings.into(), volumes.into());

	assert_eq!(result[0], 0.0);
	assert!(result[1] < 0.0);
	assert!(result[2] < result[1]);
	assert!(result[3] < result[2]);
	assert!(result[4] < result[3]);
}

#[test]
fn test_vpt_constant_price() {
	let closings = vec![10.0, 10.0, 10.0, 10.0, 10.0];
	let volumes = vec![100.0, 100.0, 100.0, 100.0, 100.0];
	let result = vpt(closings.into(), volumes.into());

	assert_eq!(result[0], 0.0);
	assert_eq!(result[1], 0.0);
	assert_eq!(result[2], 0.0);
	assert_eq!(result[3], 0.0);
	assert_eq!(result[4], 0.0);
}
