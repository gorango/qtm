use crate::internal::smma::smma_internal;

pub fn smoothed_moving_average(values: &[f64], period: Option<u32>) -> Result<Vec<f64>, String> {
	let period = period.unwrap_or(14) as usize;
	crate::utils::validation::validate_period(period)?;
	Ok(smma_internal(values, period))
}
