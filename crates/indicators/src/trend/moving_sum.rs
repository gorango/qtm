use crate::IndicatorResult;
pub fn moving_sum(values: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(4) as usize;
	crate::utils::validation::validate_period(period)?;
	Ok(crate::internal::moving_sum::moving_sum_internal(
		values, period,
	))
}
