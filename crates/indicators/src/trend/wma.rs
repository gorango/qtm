use crate::IndicatorResult;
pub fn wma_internal(values: &[f64], period: usize) -> Vec<f64> {
	let len = values.len();
	let mut result = vec![f64::NAN; len];
	if period == 0 || period > len {
		return result;
	}
	if period > usize::MAX / 2 {
		return result;
	}
	let sum_weights = (period * (period + 1) / 2) as f64;

	for i in (period - 1)..len {
		let mut sum = 0.0;
		for j in 0..period {
			let weight = (period - j) as f64;
			sum += weight * values[i - period + 1 + j];
		}
		result[i] = sum / sum_weights;
	}

	result
}

pub fn wma(values: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(14) as usize;
	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_finite(&[values])?;
	Ok(wma_internal(values, period))
}
