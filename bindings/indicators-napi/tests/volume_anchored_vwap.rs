use indicators::anchored_vwap;

#[test]
fn test_anchored_vwap_basic() {
	let highs = vec![100.0, 108.0, 105.0, 104.0];
	let lows = vec![90.0, 92.0, 93.0, 91.0];
	let closes = vec![98.0, 100.0, 96.0, 101.0];
	let volumes = vec![100.0, 200.0, 300.0, 400.0];
	let result = anchored_vwap(
		highs.into(),
		lows.into(),
		closes.into(),
		volumes.into(),
		Some(1),
	);

	assert_eq!(result.len(), 4);
}

#[test]
fn test_anchored_vwap_from_start() {
	let highs = vec![100.0, 108.0, 105.0, 104.0];
	let lows = vec![90.0, 92.0, 93.0, 91.0];
	let closes = vec![98.0, 100.0, 96.0, 101.0];
	let volumes = vec![100.0, 200.0, 300.0, 400.0];
	let result = anchored_vwap(
		highs.into(),
		lows.into(),
		closes.into(),
		volumes.into(),
		Some(0),
	);

	assert!((result[0] - 96.0).abs() < 0.1);
	assert!((result[1] - 98.67).abs() < 0.1);
}

#[test]
fn test_anchored_vwap_from_second_bar() {
	let highs = vec![100.0, 108.0, 105.0, 104.0];
	let lows = vec![90.0, 92.0, 93.0, 91.0];
	let closes = vec![98.0, 100.0, 96.0, 101.0];
	let volumes = vec![100.0, 200.0, 300.0, 400.0];
	let result = anchored_vwap(
		highs.into(),
		lows.into(),
		closes.into(),
		volumes.into(),
		Some(1),
	);

	assert!(result[0].is_nan());
	assert!((result[1] - 100.0).abs() < 0.1);
	assert!((result[2] - 98.8).abs() < 0.1);
}

#[test]
fn test_anchored_vwap_out_of_bounds() {
	let highs = vec![105.0];
	let lows = vec![95.0];
	let closes = vec![102.0];
	let volumes = vec![1000.0];
	let result = anchored_vwap(
		highs.into(),
		lows.into(),
		closes.into(),
		volumes.into(),
		Some(10),
	);

	assert_eq!(result.len(), 1);
	assert!(result[0].is_nan());
}

#[test]
fn test_anchored_vwap_zero_volume() {
	let highs = vec![100.0, 108.0, 105.0, 104.0];
	let lows = vec![90.0, 92.0, 93.0, 91.0];
	let closes = vec![98.0, 100.0, 96.0, 101.0];
	let volumes = vec![100.0, 0.0, 200.0, 300.0];
	let result = anchored_vwap(
		highs.into(),
		lows.into(),
		closes.into(),
		volumes.into(),
		Some(0),
	);

	assert!((result[0] - 96.0).abs() < 0.1);
	assert!((result[1] - 96.0).abs() < 0.1);
	assert!((result[2] - 97.33).abs() < 0.1);
}

#[test]
#[should_panic]
fn test_anchored_vwap_mismatched_lengths() {
	let highs = vec![1.0, 2.0];
	let lows = vec![1.0];
	let closes = vec![1.0, 2.0];
	let volumes = vec![1.0, 2.0];
	let _result = anchored_vwap(
		highs.into(),
		lows.into(),
		closes.into(),
		volumes.into(),
		Some(0),
	);
}

#[test]
fn test_anchored_vwap_empty_arrays() {
	let highs: Vec<f64> = vec![];
	let lows: Vec<f64> = vec![];
	let closes: Vec<f64> = vec![];
	let volumes: Vec<f64> = vec![];
	let result = anchored_vwap(
		highs.into(),
		lows.into(),
		closes.into(),
		volumes.into(),
		Some(0),
	);

	assert_eq!(result.len(), 0);
}

#[test]
fn test_anchored_vwap_single_value() {
	let highs = vec![100.0];
	let lows = vec![90.0];
	let closes = vec![98.0];
	let volumes = vec![100.0];
	let result = anchored_vwap(
		highs.into(),
		lows.into(),
		closes.into(),
		volumes.into(),
		Some(0),
	);

	assert_eq!(result.len(), 1);
	assert!((result[0] - 96.0).abs() < 0.01);
}
