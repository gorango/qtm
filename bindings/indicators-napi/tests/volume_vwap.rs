use indicators::{volume_weighted_average_price, vwap};
use indicators_core::VWAPConfig;

#[test]
fn test_vwap_with_period() {
	let highs = vec![10.0, 12.0, 8.0, 11.0, 9.0];
	let lows = vec![8.0, 10.0, 6.0, 9.0, 7.0];
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = vwap(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(VWAPConfig {
			period: Some(2),
			price_source: Some("close".to_string()),
			anchored: Some(false),
			session_length: Some(0),
		}),
	);

	assert_eq!(result.len(), 5);
}

#[test]
fn test_vwap_values_period_2() {
	let highs = vec![10.0, 12.0, 8.0, 11.0, 9.0];
	let lows = vec![8.0, 10.0, 6.0, 9.0, 7.0];
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = vwap(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(VWAPConfig {
			period: Some(2),
			price_source: Some("close".to_string()),
			anchored: Some(false),
			session_length: Some(0),
		}),
	);

	assert!((result[0] - 9.0).abs() < 0.01);
	assert!((result[1] - 10.05).abs() < 0.01);
	assert!((result[2] - 9.32).abs() < 0.01);
	assert!((result[3] - 8.8).abs() < 0.01);
	assert!((result[4] - 9.14).abs() < 0.01);
}

#[test]
fn test_vwap_default_config() {
	let highs = vec![10.0, 12.0, 8.0, 11.0, 9.0];
	let lows = vec![8.0, 10.0, 6.0, 9.0, 7.0];
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = vwap(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(VWAPConfig {
			period: None,
			price_source: None,
			anchored: None,
			session_length: None,
		}),
	);

	assert_eq!(result.len(), 5);
	assert!((result[0] - 9.0).abs() < 0.01);
	assert!((result[1] - 10.05).abs() < 0.01);
	assert!((result[2] - 9.21).abs() < 0.01);
	assert!((result[3] - 9.44).abs() < 0.01);
	assert!((result[4] - 9.18).abs() < 0.01);
}

#[test]
fn test_vwap_hlc3_price_source() {
	let highs = vec![10.0, 12.0, 8.0, 11.0, 9.0];
	let lows = vec![8.0, 10.0, 6.0, 9.0, 7.0];
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = vwap(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(VWAPConfig {
			period: Some(14),
			price_source: Some("hlc3".to_string()),
			anchored: Some(false),
			session_length: Some(0),
		}),
	);

	assert_eq!(result.len(), 5);
	for value in result {
		assert!(value.is_nan() || value.is_finite());
	}
}

#[test]
fn test_volume_weighted_average_price_alias() {
	let highs = vec![10.0, 12.0, 8.0, 11.0, 9.0];
	let lows = vec![8.0, 10.0, 6.0, 9.0, 7.0];
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = volume_weighted_average_price(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(VWAPConfig {
			period: Some(2),
			price_source: Some("close".to_string()),
			anchored: Some(false),
			session_length: Some(0),
		}),
	);

	assert_eq!(result.len(), 5);
}

#[test]
#[should_panic]
fn test_vwap_mismatched_lengths() {
	let highs = vec![1.0, 2.0, 3.0];
	let lows = vec![1.0, 2.0];
	let closings = vec![1.0, 2.0, 3.0];
	let volumes = vec![1.0, 2.0, 3.0];
	let _result = vwap(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(VWAPConfig {
			period: Some(2),
			price_source: Some("close".to_string()),
			anchored: Some(false),
			session_length: Some(0),
		}),
	);
}

#[test]
fn test_vwap_empty_arrays() {
	let highs: Vec<f64> = vec![];
	let lows: Vec<f64> = vec![];
	let closings: Vec<f64> = vec![];
	let volumes: Vec<f64> = vec![];
	let result = vwap(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(VWAPConfig {
			period: Some(2),
			price_source: Some("close".to_string()),
			anchored: Some(false),
			session_length: Some(0),
		}),
	);

	assert_eq!(result.len(), 0);
}

#[test]
fn test_vwap_single_value() {
	let highs = vec![10.0];
	let lows = vec![8.0];
	let closings = vec![9.0];
	let volumes = vec![100.0];
	let result = vwap(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(VWAPConfig {
			period: Some(2),
			price_source: Some("close".to_string()),
			anchored: Some(false),
			session_length: Some(0),
		}),
	);

	assert_eq!(result.len(), 1);
	assert!(result[0].is_finite());
}

#[test]
fn test_vwap_zero_volume() {
	let highs = vec![10.0, 12.0, 8.0];
	let lows = vec![8.0, 10.0, 6.0];
	let closings = vec![9.0, 11.0, 7.0];
	let volumes = vec![100.0, 0.0, 100.0];
	let result = vwap(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(VWAPConfig {
			period: Some(2),
			price_source: Some("close".to_string()),
			anchored: Some(false),
			session_length: Some(0),
		}),
	);

	assert_eq!(result.len(), 3);
	for value in result {
		assert!(value.is_finite() || value.is_nan());
	}
}

#[test]
fn test_vwap_anchored_true() {
	let highs = vec![10.0, 12.0, 8.0, 11.0, 9.0];
	let lows = vec![8.0, 10.0, 6.0, 9.0, 7.0];
	let closings = vec![9.0, 11.0, 7.0, 10.0, 8.0];
	let volumes = vec![100.0, 110.0, 80.0, 120.0, 90.0];
	let result = vwap(
		highs.into(),
		lows.into(),
		closings.into(),
		volumes.into(),
		Some(VWAPConfig {
			period: Some(14),
			price_source: Some("close".to_string()),
			anchored: Some(true),
			session_length: Some(0),
		}),
	);

	assert_eq!(result.len(), 5);
	for value in result {
		assert!(value.is_finite());
	}
}
