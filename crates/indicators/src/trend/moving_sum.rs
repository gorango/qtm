pub fn moving_sum(values: &[f64], period: Option<u32>) -> Result<Vec<f64>, String> {
	let period = period.unwrap_or(4) as usize;
	crate::utils::validation::validate_period(period)?;
	Ok(crate::internal::moving_sum::moving_sum_internal(
		values, period,
	))
}
