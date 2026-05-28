use indicators::{accumulation_distribution, ad};

#[test]
fn test_ad_basic() {
	let highs = vec![10.0, 9.0, 12.0, 14.0, 12.0];
	let lows = vec![6.0, 7.0, 9.0, 12.0, 10.0];
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 200.0, 300.0, 400.0, 500.0];
	let result = ad(highs.into(), lows.into(), closings.into(), volumes.into());

	assert_eq!(result.len(), 5);
	assert!(!result.is_empty());
}

#[test]
fn test_ad_values() {
	let highs = vec![10.0, 9.0, 12.0, 14.0, 12.0];
	let lows = vec![6.0, 7.0, 9.0, 12.0, 10.0];
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 200.0, 300.0, 400.0, 500.0];
	let result = ad(highs.into(), lows.into(), closings.into(), volumes.into());

	assert!((result[0] - 50.0).abs() < 0.01);
	assert!((result[1] - 650.0).abs() < 0.01);
	assert!((result[2] - -50.0).abs() < 0.01);
	assert!((result[3] - -1250.0).abs() < 0.01);
	assert!((result[4] - -2750.0).abs() < 0.01);
}

#[test]
fn test_accumulation_distribution_alias() {
	let highs = vec![10.0, 9.0, 12.0, 14.0, 12.0];
	let lows = vec![6.0, 7.0, 9.0, 12.0, 10.0];
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 200.0, 300.0, 400.0, 500.0];
	let result =
		accumulation_distribution(highs.into(), lows.into(), closings.into(), volumes.into());

	assert_eq!(result.len(), 5);
}

#[test]
#[should_panic]
fn test_ad_mismatched_lengths() {
	let highs = vec![1.0, 2.0, 3.0];
	let lows = vec![1.0, 2.0];
	let closings = vec![1.0, 2.0, 3.0];
	let volumes = vec![1.0, 2.0, 3.0];
	let _result = ad(highs.into(), lows.into(), closings.into(), volumes.into());
}

#[test]
fn test_ad_empty_arrays() {
	let highs: Vec<f64> = vec![];
	let lows: Vec<f64> = vec![];
	let closings: Vec<f64> = vec![];
	let volumes: Vec<f64> = vec![];
	let result = ad(highs.into(), lows.into(), closings.into(), volumes.into());

	assert_eq!(result.len(), 0);
}

#[test]
fn test_ad_zero_range() {
	let highs = vec![10.0, 10.0, 10.0];
	let lows = vec![10.0, 10.0, 10.0];
	let closings = vec![10.0, 10.0, 10.0];
	let volumes = vec![100.0, 100.0, 100.0];
	let result = ad(highs.into(), lows.into(), closings.into(), volumes.into());

	assert_eq!(result.len(), 3);
	for value in result {
		assert!(value.is_finite());
	}
}

#[test]
fn test_ad_single_value() {
	let highs = vec![10.0];
	let lows = vec![6.0];
	let closings = vec![9.0];
	let volumes = vec![100.0];
	let result = ad(highs.into(), lows.into(), closings.into(), volumes.into());

	assert_eq!(result.len(), 1);
	assert!((result[0] - 50.0).abs() < 0.01);
}
