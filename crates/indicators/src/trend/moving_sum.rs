use crate::IndicatorResult;
/// Moving Sum — rolling sum over `period` bars. Thin wrapper over `moving_sum_internal`.
/// Direct implementation. Period defaults to 4. `NaN` for first `period - 1` bars.
///
/// # Errors
/// Returns an error if `period` is 0 or inputs contain non-finite values.
pub fn moving_sum(values: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(4) as usize;
	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_finite(&[values])?;
	Ok(crate::internal::moving_sum::moving_sum_internal(
		values, period,
	))
}
