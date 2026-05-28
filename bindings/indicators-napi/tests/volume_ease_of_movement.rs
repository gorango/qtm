use indicators::{ease_of_movement, emv};

#[test]
fn test_emv_with_period() {
	let highs = vec![10.0, 9.0, 12.0, 14.0, 12.0];
	let lows = vec![6.0, 7.0, 9.0, 12.0, 10.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = emv(highs.into(), lows.into(), volumes.into(), Some(20));

	assert_eq!(result.len(), 5);
}

#[test]
fn test_emv_values() {
	let highs = vec![10.0, 9.0, 12.0, 14.0, 12.0];
	let lows = vec![6.0, 7.0, 9.0, 12.0, 10.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = emv(highs.into(), lows.into(), volumes.into(), Some(20));

	assert_eq!(result.len(), 5);
	for value in result {
		assert!(value.is_finite() || value.is_nan());
	}
}

#[test]
fn test_ease_of_movement_alias() {
	let highs = vec![10.0, 9.0, 12.0, 14.0, 12.0];
	let lows = vec![6.0, 7.0, 9.0, 12.0, 10.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = ease_of_movement(highs.into(), lows.into(), volumes.into(), Some(20));

	assert_eq!(result.len(), 5);
}

#[test]
#[should_panic]
fn test_emv_mismatched_lengths() {
	let highs = vec![1.0, 2.0, 3.0];
	let lows = vec![1.0, 2.0];
	let volumes = vec![1.0, 2.0, 3.0];
	let _result = emv(highs.into(), lows.into(), volumes.into(), Some(20));
}

#[test]
fn test_emv_empty_arrays() {
	let highs: Vec<f64> = vec![];
	let lows: Vec<f64> = vec![];
	let volumes: Vec<f64> = vec![];
	let result = emv(highs.into(), lows.into(), volumes.into(), Some(20));

	assert_eq!(result.len(), 0);
}

#[test]
fn test_emv_single_value() {
	let highs = vec![10.0];
	let lows = vec![6.0];
	let volumes = vec![100.0];
	let result = emv(highs.into(), lows.into(), volumes.into(), Some(20));

	assert_eq!(result.len(), 1);
	assert!(result[0].is_finite() || result[0].is_nan());
}

#[test]
fn test_emv_zero_range() {
	let highs = vec![10.0, 10.0, 10.0];
	let lows = vec![10.0, 10.0, 10.0];
	let volumes = vec![100.0, 100.0, 100.0];
	let result = emv(highs.into(), lows.into(), volumes.into(), Some(20));

	assert_eq!(result.len(), 3);
	for value in result {
		assert!(value.is_finite() || value.is_nan());
	}
}

#[test]
fn test_emv_period_5() {
	let highs = vec![10.0, 9.0, 12.0, 14.0, 12.0];
	let lows = vec![6.0, 7.0, 9.0, 12.0, 10.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = emv(highs.into(), lows.into(), volumes.into(), Some(5));

	assert_eq!(result.len(), 5);
	for value in result {
		assert!(value.is_finite() || value.is_nan());
	}
}

#[test]
fn test_emv_all_same_midpoint() {
	let highs = vec![10.0, 10.0, 10.0];
	let lows = vec![6.0, 6.0, 6.0];
	let volumes = vec![100.0, 100.0, 100.0];
	let result = emv(highs.into(), lows.into(), volumes.into(), Some(20));

	assert_eq!(result.len(), 3);
	assert!(result[0].is_finite() || result[0].is_nan());
	assert!(result[1].is_finite() || result[1].is_nan());
	assert!(result[2].is_finite() || result[2].is_nan());
}
