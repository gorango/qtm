use crate::utils::math::mean;

pub fn variance(values: &[f64], period: usize) -> Vec<f64> {
	let len = values.len();
	if len < period || period == 0 {
		return vec![f64::NAN; len];
	}

	let mut result = vec![f64::NAN; len];

	for i in period - 1..len {
		let window = &values[i - (period - 1)..=i];
		let m = mean(window);

		let sum_sq_diff = window
			.iter()
			.map(|&x| {
				let diff = x - m;
				diff * diff
			})
			.sum::<f64>();

		result[i] = sum_sq_diff / period as f64;
	}

	result
}

pub fn standard_deviation(values: &[f64], period: usize) -> Vec<f64> {
	let vars = variance(values, period);
	vars.iter()
		.map(|&v| if v.is_nan() { f64::NAN } else { v.sqrt() })
		.collect()
}

pub fn population_std(values: &[f64]) -> f64 {
	let m = mean(values);
	if m.is_nan() {
		return f64::NAN;
	}

	let sum_sq_diff = values
		.iter()
		.map(|&x| {
			let diff = x - m;
			diff * diff
		})
		.sum::<f64>();

	let variance = sum_sq_diff / values.len() as f64;
	variance.sqrt()
}

pub fn sample_std(values: &[f64]) -> f64 {
	let m = mean(values);
	if m.is_nan() || values.len() <= 1 {
		return f64::NAN;
	}

	let sum_sq_diff = values
		.iter()
		.map(|&x| {
			let diff = x - m;
			diff * diff
		})
		.sum::<f64>();

	let variance = sum_sq_diff / (values.len() - 1) as f64;
	variance.sqrt()
}
