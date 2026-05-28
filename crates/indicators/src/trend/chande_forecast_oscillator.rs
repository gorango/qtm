use std::collections::VecDeque;

fn linear_regression_least_squares(x: &[f64], y: &[f64]) -> Vec<f64> {
	let n = x.len();
	if n == 0 || n != y.len() {
		return y.to_vec();
	}

	let mut sum_x = 0.0;
	let mut sum_y = 0.0;
	let mut sum_xy = 0.0;
	let mut sum_xx = 0.0;

	for i in 0..n {
		sum_x += x[i];
		sum_y += y[i];
		sum_xy += x[i] * y[i];
		sum_xx += x[i] * x[i];
	}

	let denominator = n as f64 * sum_xx - sum_x * sum_x;
	if denominator.abs() <= 1e-10 {
		return y.to_vec();
	}

	let m = (n as f64 * sum_xy - sum_x * sum_y) / denominator;
	let b = (sum_y - m * sum_x) / n as f64;

	x.iter().map(|&xi| m * xi + b).collect()
}

fn moving_linear_regression_least_squares(period: usize, x: &[f64], y: &[f64]) -> Vec<f64> {
	let len = y.len();
	let mut result = vec![0.0; len];
	let mut window_y: VecDeque<f64> = VecDeque::new();
	let mut sum_x = 0.0;
	let mut sum_y = 0.0;
	let mut sum_xy = 0.0;
	let mut sum_xx = 0.0;

	for i in 0..len {
		let yi = y[i];
		let xi = x[i];
		window_y.push_back(yi);
		sum_x += xi;
		sum_y += yi;
		sum_xy += xi * yi;
		sum_xx += xi * xi;

		let n = window_y.len() as f64;
		if n >= 2.0 {
			let denominator = n * sum_xx - sum_x * sum_x;
			if denominator.abs() > 1e-10 {
				let m = (n * sum_xy - sum_x * sum_y) / denominator;
				let b = (sum_y - m * sum_x) / n;
				result[i] = m * xi + b;
			} else {
				result[i] = yi;
			}
		} else {
			result[i] = yi;
		}

		if window_y.len() > period {
			let old_y = window_y.pop_front().unwrap();
			let old_x = xi - period as f64;
			sum_x -= old_x;
			sum_y -= old_y;
			sum_xy -= old_x * old_y;
			sum_xx -= old_x * old_x;
		}
	}

	result
}

pub fn chande_forecast_oscillator(closings: &[f64]) -> Result<Vec<f64>, String> {
	if closings.is_empty() {
		return Err("Array cannot be empty".to_string());
	}

	let len = closings.len();
	let x: Vec<f64> = (0..len).map(|i| i as f64).collect();
	let r = linear_regression_least_squares(&x, closings);

	let result: Vec<f64> = closings
		.iter()
		.enumerate()
		.map(|(i, close)| {
			let ri = r[i];
			if *close != 0.0 {
				((*close - ri) / *close) * 100.0
			} else {
				0.0
			}
		})
		.collect();

	Ok(result)
}

pub fn moving_chande_forecast_oscillator(
	closings: &[f64],
	period: Option<u32>,
) -> Result<Vec<f64>, String> {
	let period = period.unwrap_or(4) as usize;

	if closings.is_empty() {
		return Err("Array cannot be empty".to_string());
	}

	let len = closings.len();
	let x: Vec<f64> = (0..len).map(|i| i as f64).collect();
	let r = moving_linear_regression_least_squares(period, &x, closings);

	let mut result = vec![0.0; len];

	for i in 0..len {
		let window_size = std::cmp::min(i + 1, period);
		if window_size == 1 {
			result[i] = 100.0;
		} else {
			let close = closings[i];
			let ri = r[i];
			if close != 0.0 {
				result[i] = ((close - ri) / close) * 100.0;
			}
		}
	}

	Ok(result)
}
