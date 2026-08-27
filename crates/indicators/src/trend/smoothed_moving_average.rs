use crate::internal::smma::smma_internal;
use crate::IndicatorResult;

/// Smoothed Moving Average (SMMA / RMA).
///
/// Alias for Wilder's SMMA (`smma_internal`): `SMMA[i] = (SMMA[i-1]*(period-1) + value[i]) / period`.
/// Same as RMA. Period defaults to 14. Direct implementation.
///
/// # Errors
/// Returns an error if `period` is 0 or inputs contain non-finite values.
pub fn smoothed_moving_average(values: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(14) as usize;
	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_finite(&[values])?;
	Ok(smma_internal(values, period))
}
