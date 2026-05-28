use indicators::{volume_surge, vs};
use indicators_core::VolumeSurgeConfig;

#[test]
fn test_vs_with_config() {
	let volumes = vec![
		100.0, 110.0, 120.0, 130.0, 140.0, 150.0, 160.0, 170.0, 180.0, 190.0, 200.0, 210.0, 220.0,
		230.0, 240.0, 250.0, 260.0, 270.0, 280.0, 290.0,
	];
	let result = vs(
		volumes.into(),
		Some(VolumeSurgeConfig {
			period: Some(10),
			multiplier: Some(1.5),
		}),
	);

	assert_eq!(result.len(), 20);
}

#[test]
fn test_vs_default_config() {
	let volumes = vec![
		100.0, 110.0, 120.0, 130.0, 140.0, 150.0, 160.0, 170.0, 180.0, 190.0, 200.0, 210.0, 220.0,
		230.0, 240.0, 250.0, 260.0, 270.0, 280.0, 290.0,
	];
	let result = vs(volumes.into(), None);

	assert_eq!(result.len(), 20);
}

#[test]
fn test_vs_detect_surge() {
	let volumes = vec![
		100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 250.0,
	];
	let result = vs(
		volumes.into(),
		Some(VolumeSurgeConfig {
			period: Some(10),
			multiplier: Some(2.0),
		}),
	);

	assert_eq!(result.len(), 11);
	for &value in result.iter().take(10) {
		assert!(!value);
	}
	assert!(result[10]);
}

#[test]
fn test_vs_no_surge() {
	let volumes = vec![
		100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 150.0,
	];
	let result = vs(
		volumes.into(),
		Some(VolumeSurgeConfig {
			period: Some(10),
			multiplier: Some(2.0),
		}),
	);

	assert_eq!(result.len(), 11);
	for &value in &result {
		assert!(!value);
	}
}

#[test]
fn test_volume_surge_alias() {
	let volumes = vec![
		100.0, 110.0, 120.0, 130.0, 140.0, 150.0, 160.0, 170.0, 180.0, 190.0, 200.0,
	];
	let result = volume_surge(
		volumes.into(),
		Some(VolumeSurgeConfig {
			period: Some(10),
			multiplier: Some(1.5),
		}),
	);

	assert_eq!(result.len(), 11);
}

#[test]
#[should_panic]
fn test_vs_insufficient_data() {
	let volumes = vec![100.0, 110.0, 120.0, 130.0, 140.0];
	let _result = vs(
		volumes.into(),
		Some(VolumeSurgeConfig {
			period: Some(10),
			multiplier: Some(2.0),
		}),
	);
}

#[test]
#[should_panic]
fn test_vs_empty_arrays() {
	let volumes: Vec<f64> = vec![];
	let _result = vs(volumes.into(), None);
}

#[test]
fn test_vs_single_value() {
	let volumes = vec![100.0];
	let result = vs(
		volumes.into(),
		Some(VolumeSurgeConfig {
			period: Some(1),
			multiplier: Some(2.0),
		}),
	);

	assert_eq!(result.len(), 1);
	assert!(!result[0]);
}

#[test]
fn test_vs_multiple_surge() {
	let volumes = vec![
		100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 250.0, 100.0, 100.0,
		100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 300.0,
	];
	let result = vs(
		volumes.into(),
		Some(VolumeSurgeConfig {
			period: Some(10),
			multiplier: Some(2.0),
		}),
	);

	assert_eq!(result.len(), 22);
	assert!(result[10]);
	assert!(result[21]);
}

#[test]
fn test_vs_period_20() {
	let volumes = vec![
		100.0, 110.0, 120.0, 130.0, 140.0, 150.0, 160.0, 170.0, 180.0, 190.0, 200.0, 210.0, 220.0,
		230.0, 240.0, 250.0, 260.0, 270.0, 280.0, 290.0,
	];
	let result = vs(
		volumes.into(),
		Some(VolumeSurgeConfig {
			period: Some(20),
			multiplier: Some(2.0),
		}),
	);

	assert_eq!(result.len(), 20);
}

#[test]
fn test_vs_multiplier_3() {
	let volumes = vec![
		100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 350.0,
	];
	let result = vs(
		volumes.into(),
		Some(VolumeSurgeConfig {
			period: Some(10),
			multiplier: Some(3.0),
		}),
	);

	assert_eq!(result.len(), 11);
	assert!(!result[10]);
}
