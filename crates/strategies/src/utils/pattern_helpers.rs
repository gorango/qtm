/// Find local peaks in a data series
/// Returns indices of peaks found
pub fn find_peaks(values: &[f64], lookaround: usize) -> Vec<usize> {
	let mut peaks = Vec::new();

	if values.len() < 2 * lookaround + 1 {
		return peaks;
	}

	for i in lookaround..values.len().saturating_sub(lookaround) {
		let current_value = values[i];
		let mut is_peak = true;

		for j in 1..=lookaround {
			let left_idx = i.saturating_sub(j);
			let right_idx = (i + j).min(values.len() - 1);

			if left_idx < i && values[left_idx] >= current_value {
				is_peak = false;
				break;
			}
			if right_idx > i && values[right_idx] >= current_value {
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

/// Find local troughs (local minima) in a data series
/// Returns indices of troughs found
pub fn find_troughs(values: &[f64], lookaround: usize) -> Vec<usize> {
	let mut troughs = Vec::new();

	if values.len() < 2 * lookaround + 1 {
		return troughs;
	}

	for i in lookaround..values.len().saturating_sub(lookaround) {
		let current_value = values[i];
		let mut is_trough = true;

		for j in 1..=lookaround {
			let left_idx = i.saturating_sub(j);
			let right_idx = (i + j).min(values.len() - 1);

			if left_idx < i && values[left_idx] <= current_value {
				is_trough = false;
				break;
			}
			if right_idx > i && values[right_idx] <= current_value {
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

/// Linear regression result
#[derive(Debug, Clone)]
pub struct LinearRegressionResult {
	pub slope: f64,
	pub intercept: f64,
}

/// Perform linear regression on a set of points
/// Returns slope and intercept of the best-fit line
pub fn linear_regression(points: &[(f64, f64)]) -> Option<LinearRegressionResult> {
	if points.len() < 2 {
		return None;
	}

	let n = points.len() as f64;
	let mut sum_x = 0.0;
	let mut sum_y = 0.0;
	let mut sum_xy = 0.0;
	let mut sum_xx = 0.0;

	for &(x, y) in points {
		sum_x += x;
		sum_y += y;
		sum_xy += x * y;
		sum_xx += x * x;
	}

	let denominator = n * sum_xx - sum_x * sum_x;

	if denominator.abs() > 1e-10 {
		let slope = (n * sum_xy - sum_x * sum_y) / denominator;
		let intercept = (sum_y - slope * sum_x) / n;
		Some(LinearRegressionResult { slope, intercept })
	} else {
		None
	}
}
