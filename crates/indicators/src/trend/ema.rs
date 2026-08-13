use crate::internal::ema::ema_internal;
use crate::IndicatorResult;

pub fn ema(values: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(12) as usize;
	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_finite(&[values])?;
	Ok(ema_internal(values, period))
}
