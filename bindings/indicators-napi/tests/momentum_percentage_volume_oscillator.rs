use indicators::percentage_volume_oscillator;
use indicators_core::PercentageVolumeOscillatorConfig;

#[test]
fn test_pvo_with_config() {
	let volumes = vec![
		6954.0, 4511.0, 4474.0, 4126.0, 4572.0, 3936.0, 3192.0, 3090.0, 3476.0, 3852.0, 3107.0,
		3604.0, 4145.0, 5192.0, 3560.0, 3961.0, 4322.0, 3901.0, 3392.0, 4278.0, 4212.0, 4428.0,
		3846.0, 3824.0, 4142.0, 4964.0, 4683.0, 4630.0, 4746.0, 4254.0, 4197.0, 4236.0, 3877.0,
		4474.0, 3943.0, 3969.0, 3876.0, 3760.0, 4061.0, 3930.0, 3833.0, 3678.0, 3197.0, 3509.0,
		3634.0, 3273.0, 3451.0, 3452.0, 3453.0, 4054.0, 4137.0, 3906.0, 3833.0, 3828.0, 3782.0,
		3665.0, 4239.0, 3696.0, 3577.0, 3573.0, 4014.0, 3962.0, 3961.0, 6681.0, 4174.0, 5002.0,
		4331.0, 4757.0, 3877.0, 4008.0, 4220.0, 6237.0, 5506.0, 4558.0, 4062.0, 4409.0, 4679.0,
		4594.0, 3941.0, 5070.0, 3814.0, 4007.0, 3871.0, 3596.0, 3478.0, 3363.0, 3466.0, 4164.0,
		4490.0, 3662.0,
	];

	let result = percentage_volume_oscillator(
		volumes.into(),
		Some(PercentageVolumeOscillatorConfig {
			fast_period: Some(6),
			slow_period: Some(13),
			signal_period: Some(7),
		}),
	);

	assert_eq!(result.pvo_result.len(), 90);
	assert_eq!(result.signal_period.len(), 90);
	assert_eq!(result.histogram.len(), 90);

	for value in &result.pvo_result {
		assert!(value.is_nan() || value.is_finite());
	}

	for value in &result.signal_period {
		assert!(value.is_nan() || value.is_finite());
	}

	for value in &result.histogram {
		assert!(value.is_nan() || value.is_finite());
	}
}

#[test]
fn test_pvo_without_config() {
	let volumes = vec![
		6954.0, 4511.0, 4474.0, 4126.0, 4572.0, 3936.0, 3192.0, 3090.0, 3476.0, 3852.0, 3107.0,
		3604.0, 4145.0, 5192.0, 3560.0, 3961.0, 4322.0, 3901.0, 3392.0, 4278.0, 4212.0, 4428.0,
		3846.0, 3824.0, 4142.0, 4964.0, 4683.0, 4630.0, 4746.0, 4254.0, 4197.0, 4236.0, 3877.0,
		4474.0, 3943.0, 3969.0, 3876.0, 3760.0, 4061.0, 3930.0, 3833.0, 3678.0, 3197.0, 3509.0,
		3634.0, 3273.0, 3451.0, 3452.0, 3453.0, 4054.0, 4137.0, 3906.0, 3833.0, 3828.0, 3782.0,
		3665.0, 4239.0, 3696.0, 3577.0, 3573.0, 4014.0, 3962.0, 3961.0, 6681.0, 4174.0, 5002.0,
		4331.0, 4757.0, 3877.0, 4008.0, 4220.0, 6237.0, 5506.0, 4558.0, 4062.0, 4409.0, 4679.0,
		4594.0, 3941.0, 5070.0, 3814.0, 4007.0, 3871.0, 3596.0, 3478.0, 3363.0, 3466.0, 4164.0,
		4490.0, 3662.0,
	];

	let result = percentage_volume_oscillator(volumes.into(), None);

	assert_eq!(result.pvo_result.len(), 90);
	assert_eq!(result.signal_period.len(), 90);
	assert_eq!(result.histogram.len(), 90);

	for value in &result.pvo_result {
		assert!(value.is_nan() || value.is_finite());
	}

	for value in &result.signal_period {
		assert!(value.is_nan() || value.is_finite());
	}

	for value in &result.histogram {
		assert!(value.is_nan() || value.is_finite());
	}
}
