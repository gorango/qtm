use crate::utils::arrays::validate_arrays_equal_length;

pub fn negative_volume_index(closings: &[f64], volumes: &[f64], start: Option<f64>) -> Vec<f64> {
	validate_arrays_equal_length(&[closings, volumes]).unwrap();

	let len = closings.len();
	let mut result = vec![f64::NAN; len];
	let start_val = start.unwrap_or(1000.0);

	result[0] = start_val;

	for i in 1..len {
		if volumes[i - 1] < volumes[i] {
			result[i] = result[i - 1];
		} else {
			let prev_nvi = result[i - 1];
			let price_change = (closings[i] - closings[i - 1]) / closings[i - 1];
			result[i] = prev_nvi + price_change * prev_nvi;
		}
	}

	result
}

pub fn nvi(closings: &[f64], volumes: &[f64], start: Option<f64>) -> Vec<f64> {
	negative_volume_index(closings, volumes, start)
}
