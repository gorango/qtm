/// Rolling std kernel — population std over `period` bars. `O(n*period)`, no validation.
pub fn std_dev_internal(values: &[f64], period: usize) -> Vec<f64> {
	let len = values.len();
	if period == 0 || len < period {
		return vec![f64::NAN; len];
	}

	let mut result = vec![f64::NAN; len];

	let start_idx = period - 1;
	for i in start_idx..len {
		let window_start = i - (period - 1);
		let window = &values[window_start..=i];
		let mut mean = 0.0;
		let mut m2 = 0.0;

		for (idx, &x) in window.iter().enumerate() {
			let count = idx + 1;
			let count_f = count as f64;
			let delta = x - mean;
			mean += delta / count_f;
			let delta2 = x - mean;
			m2 += delta * delta2;
		}

		if !mean.is_nan() {
			let variance = m2 / period as f64;
			result[i] = variance.sqrt();
		}
	}

	result
}
