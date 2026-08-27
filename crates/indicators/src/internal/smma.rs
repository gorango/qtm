/// SMMA kernel — Wilder's smoothing. `O(n)`, no validation.
pub fn smma_internal(values: &[f64], period: usize) -> Vec<f64> {
	let len = values.len();
	if len < period {
		return vec![f64::NAN; len];
	}

	let mut result = vec![f64::NAN; len];

	for i in (period - 1)..len {
		if i == period - 1 {
			let mut sum = 0.0;
			for &val in values.iter().take(period) {
				sum += val;
			}
			result[i] = sum / period as f64;
		} else {
			result[i] = (result[i - 1] * (period - 1) as f64 + values[i]) / period as f64;
		}
	}

	result
}
