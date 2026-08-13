use crate::internal::moving_sum::moving_sum_internal;
use crate::utils::arrays::validate_arrays_equal_length;

pub fn volume_price_trend(closings: &[f64], volumes: &[f64]) -> Vec<f64> {
	if validate_arrays_equal_length(&[closings, volumes]).is_err() {
		return vec![];
	}

	let len = closings.len();
	let mut vpt_values = vec![f64::NAN; len];

	vpt_values[0] = 0.0;

	for i in 1..len {
		let prev_close = closings[i - 1];
		let c = closings[i];
		let volume = volumes[i];
		let change = (c - prev_close) / prev_close;
		vpt_values[i] = volume * change;
	}

	moving_sum_internal(&vpt_values, len)
}

pub fn vpt(closings: &[f64], volumes: &[f64]) -> Vec<f64> {
	volume_price_trend(closings, volumes)
}
