use crate::internal::ema::ema_internal;
use crate::IndicatorResult;

/// Triple Exponential Moving Average (TEMA).
///
/// TEMA = 3*EMA1 - 3*EMA2 + EMA3 where EMA1/2/3 are successive EMAs. Even less lag than DEMA. Defined by Mulloy (1994). Period defaults to 12.
///
/// # Errors
/// Returns an error if `period` is 0 or inputs contain non-finite values.
pub fn tema(values: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(2) as usize;
	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_finite(&[values])?;

	let ema1 = ema_internal(values, period);
	let ema2 = ema_internal(&ema1, period);
	let ema3 = ema_internal(&ema2, period);

	let result: Vec<f64> = ema1
		.iter()
		.enumerate()
		.map(|(i, e1)| 3.0 * e1 - 3.0 * ema2[i] + ema3[i])
		.collect();

	Ok(result)
}
