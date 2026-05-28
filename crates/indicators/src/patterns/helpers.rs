pub fn find_peaks(values: &[f64], lookaround: u32) -> Vec<u32> {
	find_peaks_internal(values, lookaround as usize)
		.iter()
		.map(|&x| x as u32)
		.collect()
}

pub fn find_troughs(values: &[f64], lookaround: u32) -> Vec<u32> {
	find_troughs_internal(values, lookaround as usize)
		.iter()
		.map(|&x| x as u32)
		.collect()
}

pub fn linear_regression(points: Vec<f64>) -> Vec<f64> {
	linear_regression_internal(&points)
}

pub fn find_peaks_internal(values: &[f64], lookaround: usize) -> Vec<usize> {
	let mut peaks = Vec::new();

	if values.len() < 2 * lookaround + 1 {
		return peaks;
	}

	for i in lookaround..values.len() - lookaround {
		let current_value = values[i];
		let mut is_peak = true;

		for j in 1..=lookaround {
			if values[i - j] >= current_value || values[i + j] >= current_value {
				is_peak = false;
				break;
			}
		}

		if is_peak {
			peaks.push(i);
		}
	}

	peaks
}

pub fn find_troughs_internal(values: &[f64], lookaround: usize) -> Vec<usize> {
	let mut troughs = Vec::new();

	if values.len() < 2 * lookaround + 1 {
		return troughs;
	}

	for i in lookaround..values.len() - lookaround {
		let current_value = values[i];
		let mut is_trough = true;

		for j in 1..=lookaround {
			if values[i - j] <= current_value || values[i + j] <= current_value {
				is_trough = false;
				break;
			}
		}

		if is_trough {
			troughs.push(i);
		}
	}

	troughs
}

pub fn linear_regression_internal(points: &[f64]) -> Vec<f64> {
	if points.len() < 4 || !points.len().is_multiple_of(2) {
		return vec![0.0, 0.0];
	}

	let n = points.len() / 2;
	let mut sum_x = 0.0;
	let mut sum_y = 0.0;
	let mut sum_xy = 0.0;
	let mut sum_xx = 0.0;

	for i in 0..n {
		let x = points[2 * i];
		let y = points[2 * i + 1];
		sum_x += x;
		sum_y += y;
		sum_xy += x * y;
		sum_xx += x * x;
	}

	let n_f64 = n as f64;
	let denominator = n_f64 * sum_xx - sum_x * sum_x;

	let slope = if denominator.abs() > 1e-10 {
		(n_f64 * sum_xy - sum_x * sum_y) / denominator
	} else {
		0.0
	};

	let intercept = (sum_y - slope * sum_x) / n_f64;

	vec![slope, intercept]
}

pub fn zig_zag_filter(values: &[f64], deviation: f64) -> Vec<f64> {
	zig_zag_filter_internal(values, deviation)
}

pub fn zig_zag_filter_internal(values: &[f64], deviation: f64) -> Vec<f64> {
	if values.is_empty() {
		return Vec::new();
	}

	let mut filtered = vec![0.0; values.len()];
	filtered[0] = values[0];

	let mut last_pivot = 0;

	for i in 1..values.len() {
		let change = ((values[i] - values[last_pivot]) / values[last_pivot]).abs();
		if change >= deviation {
			filtered[i] = values[i];
			last_pivot = i;
		} else {
			filtered[i] = values[last_pivot];
		}
	}

	filtered
}
