use crate::trend::rma::rma_internal;
use crate::IndicatorResult;

pub fn rolling_moving_average(values: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(4) as usize;
	crate::utils::validation::validate_period(period)?;
	Ok(rma_internal(values, period))
}
