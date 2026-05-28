pub fn rolling_sum(values: &[f64], period: usize) -> Vec<f64> {
	let len = values.len();
	if len < period || period == 0 {
		return vec![f64::NAN; len];
	}

	let mut result = vec![f64::NAN; len];
	let mut window_sum = 0.0;

	for i in 0..len {
		window_sum += values[i];

		if i >= period {
			window_sum -= values[i - period];
		}

		if i >= period - 1 {
			result[i] = window_sum;
		}
	}

	result
}

pub fn rolling_mean(values: &[f64], period: usize) -> Vec<f64> {
	let sums = rolling_sum(values, period);
	let len = sums.len();
	let mut result = vec![f64::NAN; len];

	for i in 0..len {
		if !sums[i].is_nan() && period > 0 {
			result[i] = sums[i] / period as f64;
		}
	}

	result
}

pub fn rolling_max(values: &[f64], period: usize) -> Vec<f64> {
	let len = values.len();
	if len < period || period == 0 {
		return vec![f64::NAN; len];
	}

	let mut result = vec![f64::NAN; len];

	for i in 0..len.saturating_sub(period).saturating_add(1) {
		if period == 0 {
			continue;
		}
		let window = &values[i..i + period];
		result[i + period - 1] = window.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
	}

	result
}

pub fn rolling_min(values: &[f64], period: usize) -> Vec<f64> {
	let len = values.len();
	if len < period || period == 0 {
		return vec![f64::NAN; len];
	}

	let mut result = vec![f64::NAN; len];

	for i in 0..len.saturating_sub(period).saturating_add(1) {
		if period == 0 {
			continue;
		}
		let window = &values[i..i + period];
		result[i + period - 1] = window.iter().fold(f64::INFINITY, |a, &b| a.min(b));
	}

	result
}

pub fn rolling_min_growing(values: &[f64], period: usize) -> Vec<f64> {
	let len = values.len();
	if len == 0 {
		return vec![];
	}

	let mut result = vec![0.0; len];

	for i in 0..len {
		let window_size = std::cmp::min(i + 1, period);
		let window = &values[i + 1 - window_size..=i];
		result[i] = window.iter().fold(f64::INFINITY, |a, &b| a.min(b));
	}

	result
}

pub fn rolling_max_growing(values: &[f64], period: usize) -> Vec<f64> {
	let len = values.len();
	if len == 0 {
		return vec![];
	}

	let mut result = vec![0.0; len];

	for i in 0..len {
		let window_size = std::cmp::min(i + 1, period);
		let window = &values[i + 1 - window_size..=i];
		result[i] = window.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
	}

	result
}
