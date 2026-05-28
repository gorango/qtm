use crate::IndicatorResult;
pub fn rma_internal(values: &[f64], period: usize) -> Vec<f64> {
	let len = values.len();
	let mut result = vec![0.0; len];
	let mut sum = 0.0;

	for i in 0..len {
		let count = if i < period {
			sum += values[i];
			(i + 1) as f64
		} else {
			sum = result[i - 1] * (period - 1) as f64 + values[i];
			period as f64
		};

		result[i] = sum / count;
	}

	result
}

pub fn rma(values: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(4) as usize;
	crate::utils::validation::validate_period(period)?;
	Ok(rma_internal(values, period))
}
