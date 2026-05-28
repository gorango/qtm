pub fn moving_sum_internal(values: &[f64], period: usize) -> Vec<f64> {
	let len = values.len();
	let mut result = Vec::with_capacity(len);
	let mut sum = 0.0;

	for i in 0..len {
		sum += values[i];

		if i >= period {
			sum -= values[i - period];
		}

		result.push(sum);
	}

	result
}
