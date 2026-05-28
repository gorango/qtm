use crate::utils::arrays::validate_arrays_equal_length;

pub fn obv(closings: &[f64], volumes: &[f64]) -> Vec<f64> {
	validate_arrays_equal_length(&[closings, volumes]).unwrap();

	let len = closings.len();
	let mut result = vec![f64::NAN; len];

	let mut cumulative = 0.0;
	result[0] = cumulative;

	for i in 1..len {
		if closings[i] > closings[i - 1] {
			cumulative += volumes[i];
		} else if closings[i] < closings[i - 1] {
			cumulative -= volumes[i];
		}
		result[i] = cumulative;
	}

	result
}

pub fn on_balance_volume(closings: &[f64], volumes: &[f64]) -> Vec<f64> {
	obv(closings, volumes)
}
