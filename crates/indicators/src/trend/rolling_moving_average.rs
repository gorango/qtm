use crate::trend::rma::rma_internal;

pub fn rolling_moving_average(values: &[f64], period: Option<u32>) -> Result<Vec<f64>, String> {
	let period = period.unwrap_or(4) as usize;
	crate::utils::validation::validate_period(period)?;
	Ok(rma_internal(values, period))
}
