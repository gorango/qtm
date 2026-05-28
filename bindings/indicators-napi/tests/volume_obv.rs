use indicators::{obv, on_balance_volume};

#[test]
fn test_obv_basic() {
	let closes = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = obv(closes.into(), volumes.into());

	assert_eq!(result.len(), 5);
	assert!(!result.is_empty());
}

#[test]
fn test_obv_values() {
	let closes = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = obv(closes.into(), volumes.into());

	assert_eq!(result[0], 0.0);
	assert_eq!(result[1], 110.0);
	assert_eq!(result[2], 30.0);
	assert_eq!(result[3], 150.0);
	assert_eq!(result[4], 60.0);
}

#[test]
fn test_on_balance_volume_alias() {
	let closes = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = on_balance_volume(closes.into(), volumes.into());

	assert_eq!(result.len(), 5);
	assert_eq!(result[0], 0.0);
	assert_eq!(result[1], 110.0);
}

#[test]
#[should_panic]
fn test_obv_mismatched_lengths() {
	let closes = vec![1.0, 2.0, 3.0];
	let volumes = vec![1.0, 2.0];
	let _result = obv(closes.into(), volumes.into());
}

#[test]
#[should_panic]
fn test_obv_empty_arrays() {
	let closes: Vec<f64> = vec![];
	let volumes: Vec<f64> = vec![];
	let _result = obv(closes.into(), volumes.into());
}

#[test]
fn test_obv_single_value() {
	let closes = vec![10.0];
	let volumes = vec![100.0];
	let result = obv(closes.into(), volumes.into());

	assert_eq!(result.len(), 1);
	assert_eq!(result[0], 0.0);
}

#[test]
fn test_obv_all_increasing() {
	let closes = vec![10.0, 11.0, 12.0, 13.0, 14.0];
	let volumes = vec![100.0, 100.0, 100.0, 100.0, 100.0];
	let result = obv(closes.into(), volumes.into());

	assert_eq!(result[0], 0.0);
	assert_eq!(result[1], 100.0);
	assert_eq!(result[2], 200.0);
	assert_eq!(result[3], 300.0);
	assert_eq!(result[4], 400.0);
}

#[test]
fn test_obv_all_decreasing() {
	let closes = vec![14.0, 13.0, 12.0, 11.0, 10.0];
	let volumes = vec![100.0, 100.0, 100.0, 100.0, 100.0];
	let result = obv(closes.into(), volumes.into());

	assert_eq!(result[0], 0.0);
	assert_eq!(result[1], -100.0);
	assert_eq!(result[2], -200.0);
	assert_eq!(result[3], -300.0);
	assert_eq!(result[4], -400.0);
}
