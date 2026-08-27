use crate::internal::ema::ema_internal;
use crate::IndicatorResult;

/// TRIX.
///
/// Triple-smoothed EMA rate-of-change: `100 * (EMA3[i] - EMA3[i-1]) / EMA3[i-1]` where EMA3 is EMA applied three times. Signals momentum; zero-line cross indicates trend change. Period defaults to 14.
///
/// # Errors
/// Returns an error if `period` is 0 or inputs contain non-finite values.
pub fn trix(values: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(4) as usize;
	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_finite(&[values])?;

	let ema1 = ema_internal(values, period);
	let ema2 = ema_internal(&ema1, period);
	let ema3 = ema_internal(&ema2, period);

	let mut result = vec![f64::NAN; ema3.len()];
	for i in 1..ema3.len() {
		let current = ema3[i];
		let previous = ema3[i - 1];

		if current.is_nan() || previous.is_nan() {
			result[i] = f64::NAN;
		} else if previous != 0.0 {
			result[i] = (current - previous) / previous;
		} else {
			result[i] = 0.0;
		}
	}

	Ok(result)
}
