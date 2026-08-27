use crate::trend::rma::rma_internal;
use crate::IndicatorResult;

/// Rolling Moving Average — alias for Wilder's RMA (`rma_internal`).
/// Same recurrence as `rma`/`smma`. Period defaults to 14.
///
/// # Errors
/// Returns an error if `period` is 0.
pub fn rolling_moving_average(values: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(4) as usize;
	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_finite(&[values])?;
	Ok(rma_internal(values, period))
}
