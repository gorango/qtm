use indicators::{bb, macd, rsi, sma};
use indicators_core::{BBConfig, MACDConfig, RSIConfig};

#[test]
fn test_shared_sma_computation() {
	let closes = vec![
		102.0, 106.0, 110.0, 113.0, 116.0, 118.0, 120.0, 123.0, 126.0, 128.0,
	];
	let period = 4;
	let expected = vec![
		f64::NAN,
		f64::NAN,
		f64::NAN,
		107.75,
		111.25,
		114.25,
		116.75,
		119.25,
		121.75,
		124.25,
	];

	let result = sma(closes.into(), Some(period)).unwrap();

	assert_eq!(result.len(), expected.len());

	for (i, &value) in result.iter().enumerate() {
		if expected[i].is_nan() {
			assert!(value.is_nan());
		} else {
			assert!((value - expected[i]).abs() < 0.01);
		}
	}
}

#[test]
fn test_shared_rsi_values() {
	let closes = vec![
		102.0, 106.0, 110.0, 113.0, 116.0, 118.0, 120.0, 123.0, 126.0, 128.0,
	];
	let period = 14;

	let result = rsi(
		closes.clone().into(),
		Some(RSIConfig {
			period: Some(period),
		}),
	);

	assert!(result.len() == closes.len());
}

#[test]
fn test_shared_macd_values() {
	let closes = vec![
		102.0, 106.0, 110.0, 113.0, 116.0, 118.0, 120.0, 123.0, 126.0, 128.0,
	];
	let fast_period = 12;
	let slow_period = 26;
	let signal_period = 9;

	let result = macd(
		closes.clone().into(),
		Some(MACDConfig {
			fast_period: Some(fast_period),
			slow_period: Some(slow_period),
			signal_period: Some(signal_period),
		}),
	);

	let result = result.unwrap();
	assert_eq!(result.macd.len(), closes.len());
	assert_eq!(result.signal.len(), closes.len());
	assert_eq!(result.histogram.len(), closes.len());
}

#[test]
fn test_shared_bollinger_bands() {
	let closes = vec![
		102.0, 106.0, 110.0, 113.0, 116.0, 118.0, 120.0, 123.0, 126.0, 128.0,
	];
	let period = 20;
	let std_dev = 2.0;

	let result = bb(
		closes.clone().into(),
		Some(BBConfig {
			period: Some(period),
			std_dev: Some(std_dev),
		}),
	)
	.unwrap();

	assert!(!result.upper.is_empty());
	assert!(!result.middle.is_empty());
	assert!(!result.lower.is_empty());
	assert_eq!(result.upper.len(), closes.len());
	assert_eq!(result.middle.len(), closes.len());
	assert_eq!(result.lower.len(), closes.len());
}
