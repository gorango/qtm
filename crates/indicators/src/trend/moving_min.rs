use crate::IndicatorResult;
pub fn moving_min_internal(values: &[f64], period: usize) -> Vec<f64> {
	let len = values.len();
	let mut result = vec![0.0; len];

	for i in 0..len {
		let mut min = values[i];
		let start = if i >= period { i - period + 1 } else { 0 };

		for &val in &values[start..=i] {
			if val < min {
				min = val;
			}
		}

		result[i] = min;
	}

	result
}

pub fn moving_min(values: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(4) as usize;
	crate::utils::validation::validate_period(period)?;
	Ok(moving_min_internal(values, period))
}
