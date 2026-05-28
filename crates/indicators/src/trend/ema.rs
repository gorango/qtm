use crate::internal::ema::ema_internal;

pub fn ema(values: &[f64], period: Option<u32>) -> Result<Vec<f64>, String> {
	let period = period.unwrap_or(12) as usize;
	crate::utils::validation::validate_period(period)?;
	Ok(ema_internal(values, period))
}
