use indicators::volume_profile;

#[test]
fn test_volume_profile_basic() {
	let highs = vec![105.0, 108.0, 110.0, 112.0, 115.0];
	let lows = vec![95.0, 98.0, 100.0, 105.0, 108.0];
	let volumes = vec![1000.0, 800.0, 1200.0, 900.0, 1100.0];
	let result = volume_profile(highs.into(), lows.into(), volumes.into(), Some(10));

	assert_eq!(result.price_levels.len(), 10);
	assert_eq!(result.volumes.len(), 10);
	assert!(!result.price_levels.is_empty());
}

#[test]
fn test_volume_profile_properties() {
	let highs = vec![105.0, 108.0, 110.0, 112.0, 115.0];
	let lows = vec![95.0, 98.0, 100.0, 105.0, 108.0];
	let volumes = vec![1000.0, 800.0, 1200.0, 900.0, 1100.0];
	let result = volume_profile(highs.into(), lows.into(), volumes.into(), Some(10));

	assert!(result.point_of_control >= 95.0);
	assert!(result.point_of_control <= 115.0);
	assert!(result.high_volume_node >= 95.0);
	assert!(result.high_volume_node <= 115.0);
	assert!(result.low_volume_node >= 95.0);
	assert!(result.low_volume_node <= 115.0);
}

#[test]
fn test_volume_profile_single_price_level() {
	let highs = vec![100.0];
	let lows = vec![100.0];
	let volumes = vec![1000.0];
	let result = volume_profile(highs.into(), lows.into(), volumes.into(), Some(5));

	assert_eq!(result.price_levels.len(), 1);
	assert_eq!(result.volumes.len(), 1);
	assert_eq!(result.point_of_control, 100.0);
	assert_eq!(result.high_volume_node, 100.0);
	assert_eq!(result.low_volume_node, 100.0);
}

#[test]
fn test_volume_profile_empty_data() {
	let highs: Vec<f64> = vec![];
	let lows: Vec<f64> = vec![];
	let volumes: Vec<f64> = vec![];
	let result = volume_profile(highs.into(), lows.into(), volumes.into(), Some(10));

	assert_eq!(result.price_levels.len(), 0);
	assert_eq!(result.volumes.len(), 0);
	assert_eq!(result.point_of_control, 0.0);
	assert_eq!(result.high_volume_node, 0.0);
	assert_eq!(result.low_volume_node, 0.0);
}

#[test]
fn test_volume_profile_point_of_control() {
	let highs = vec![102.0, 103.0, 104.0];
	let lows = vec![100.0, 101.0, 102.0];
	let volumes = vec![500.0, 1500.0, 800.0];
	let result = volume_profile(highs.into(), lows.into(), volumes.into(), Some(5));

	assert!((result.point_of_control - 102.0).abs() < 1.0);
}

#[test]
fn test_volume_profile_default_bins() {
	let highs = vec![105.0, 108.0, 110.0, 112.0, 115.0];
	let lows = vec![95.0, 98.0, 100.0, 105.0, 108.0];
	let volumes = vec![1000.0, 800.0, 1200.0, 900.0, 1100.0];
	let result = volume_profile(highs.into(), lows.into(), volumes.into(), None);

	assert_eq!(result.price_levels.len(), 50);
	assert_eq!(result.volumes.len(), 50);
}

#[test]
fn test_volume_profile_single_value() {
	let highs = vec![10.0];
	let lows = vec![5.0];
	let volumes = vec![1000.0];
	let result = volume_profile(highs.into(), lows.into(), volumes.into(), Some(5));

	assert_eq!(result.price_levels.len(), 5);
	assert_eq!(result.volumes.len(), 5);
	assert!(result.point_of_control >= 5.0);
	assert!(result.point_of_control <= 10.0);
}

#[test]
fn test_volume_profile_high_volume_at_level() {
	let highs = vec![102.0, 103.0, 104.0];
	let lows = vec![100.0, 101.0, 102.0];
	let volumes = vec![500.0, 1500.0, 800.0];
	let result = volume_profile(highs.into(), lows.into(), volumes.into(), Some(5));

	assert!(result.high_volume_node >= 100.0);
	assert!(result.high_volume_node <= 104.0);
}

#[test]
fn test_volume_profile_low_volume_at_level() {
	let highs = vec![102.0, 103.0, 104.0];
	let lows = vec![100.0, 101.0, 102.0];
	let volumes = vec![500.0, 1500.0, 800.0];
	let result = volume_profile(highs.into(), lows.into(), volumes.into(), Some(5));

	assert!(result.low_volume_node >= 100.0);
	assert!(result.low_volume_node <= 104.0);
}
