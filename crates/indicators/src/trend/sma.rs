use crate::internal::sma::sma_internal;
use crate::IndicatorResult;

/// Simple Moving Average (SMA).
///
/// Arithmetic mean of the last `period` values. Direct implementation of the textbook definition. Output is `NaN` for the first `period - 1` bars; period defaults to 2 if `None`.
///
/// # Errors
/// Returns an error if `period` is 0 or inputs contain non-finite values.
pub fn sma(values: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(2) as usize;
	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_finite(&[values])?;
	Ok(sma_internal(values, period))
}
