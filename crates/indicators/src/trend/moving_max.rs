use crate::IndicatorResult;
pub fn moving_max_internal(values: &[f64], period: usize) -> Vec<f64> {
	let len = values.len();
	let mut result = vec![0.0; len];

	for i in 0..len {
		let mut max = values[i];
		let start = if i >= period { i - period + 1 } else { 0 };

		for &val in &values[start..=i] {
			if val > max {
				max = val;
			}
		}

		result[i] = max;
	}

	result
}

pub fn moving_max(values: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(4) as usize;
	crate::utils::validation::validate_period(period)?;
	Ok(moving_max_internal(values, period))
}
