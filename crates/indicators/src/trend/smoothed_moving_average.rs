use crate::internal::smma::smma_internal;
use crate::IndicatorResult;

pub fn smoothed_moving_average(values: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(14) as usize;
	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_finite(&[values])?;
	Ok(smma_internal(values, period))
}
