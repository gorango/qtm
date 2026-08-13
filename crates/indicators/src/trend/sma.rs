use crate::internal::sma::sma_internal;
use crate::IndicatorResult;

pub fn sma(values: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(2) as usize;
	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_finite(&[values])?;
	Ok(sma_internal(values, period))
}
