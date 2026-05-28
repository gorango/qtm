use indicators::{chaikin_money_flow, cmf};

#[test]
fn test_cmf_with_period() {
	let highs = vec![10.0, 9.0, 12.0, 14.0, 12.0];
	let lows = vec![6.0, 7.0, 9.0, 12.0, 10.0];
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = cmf(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(14),
	);

	assert_eq!(result.len(), 5);
}

#[test]
fn test_cmf_values() {
	let highs = vec![10.0, 9.0, 12.0, 14.0, 12.0];
	let lows = vec![6.0, 7.0, 9.0, 12.0, 10.0];
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = cmf(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(14),
	);

	assert!((result[0] - 0.5).abs() < 0.01);
	assert!((result[1] - 1.81).abs() < 0.01);
	assert!((result[2] - 0.67).abs() < 0.01);
	assert!((result[3] - -0.41).abs() < 0.01);
	assert!((result[4] - -0.87).abs() < 0.01);
}

#[test]
fn test_chaikin_money_flow_alias() {
	let highs = vec![10.0, 9.0, 12.0, 14.0, 12.0];
	let lows = vec![6.0, 7.0, 9.0, 12.0, 10.0];
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = chaikin_money_flow(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(14),
	);

	assert_eq!(result.len(), 5);
}

#[test]
#[should_panic]
fn test_cmf_mismatched_lengths() {
	let highs = vec![1.0, 2.0, 3.0];
	let lows = vec![1.0, 2.0];
	let closings = vec![1.0, 2.0, 3.0];
	let volumes = vec![1.0, 2.0, 3.0];
	let _result = cmf(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(14),
	);
}

#[test]
fn test_cmf_empty_arrays() {
	let highs: Vec<f64> = vec![];
	let lows: Vec<f64> = vec![];
	let closings: Vec<f64> = vec![];
	let volumes: Vec<f64> = vec![];
	let result = cmf(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(14),
	);

	assert_eq!(result.len(), 0);
}

#[test]
fn test_cmf_single_value() {
	let highs = vec![10.0];
	let lows = vec![6.0];
	let closings = vec![9.0];
	let volumes = vec![100.0];
	let result = cmf(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(14),
	);

	assert_eq!(result.len(), 1);
	assert!(result[0].is_finite());
}

#[test]
fn test_cmf_zero_volume() {
	let highs = vec![10.0, 9.0];
	let lows = vec![6.0, 7.0];
	let closings = vec![9.0, 11.0];
	let volumes = vec![100.0, 0.0];
	let result = cmf(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(14),
	);

	assert_eq!(result.len(), 2);
	assert!(result[0].is_finite());
	assert!(result[1].is_finite());
}

#[test]
fn test_cmf_period_14() {
	let highs = vec![10.0, 9.0, 12.0, 14.0, 12.0];
	let lows = vec![6.0, 7.0, 9.0, 12.0, 10.0];
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = cmf(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(14),
	);

	assert_eq!(result.len(), 5);
	for value in result {
		assert!(value.is_finite() || value.is_nan());
	}
}
