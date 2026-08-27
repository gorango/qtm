use crate::internal::ema::ema_internal;
use crate::IndicatorResult;

/// Exponential Moving Average (EMA).
///
/// EMA with smoothing factor `2 / (period + 1)`. More weight to recent values than SMA. Direct implementation. Period defaults to 12. Output is `NaN` until `period - 1`.
///
/// # Errors
/// Returns an error if `period` is 0 or inputs contain non-finite values.
pub fn ema(values: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(12) as usize;
	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_finite(&[values])?;
	Ok(ema_internal(values, period))
}
