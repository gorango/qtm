use crate::internal::sma::sma_internal;

pub fn sma(values: &[f64], period: Option<u32>) -> Result<Vec<f64>, String> {
	let period = period.unwrap_or(2) as usize;
	crate::utils::validation::validate_period(period)?;
	Ok(sma_internal(values, period))
}
