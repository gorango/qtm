use crate::internal::ema::ema_internal;
use crate::IndicatorResult;

/// Absolute Price Oscillator (APO).
///
/// Difference between two EMAs: `EMA(fast_period) - EMA(slow_period)`.
/// Positive values indicate bullish momentum. Direct implementation; no normalization.
/// Defaults: fast 14, slow 30. Returns `NaN` where either EMA is `NaN`.
///
/// # Errors
/// Returns an error if either `period` is 0.
pub fn absolute_price_oscillator(
	values: &[f64],
	fast_period: Option<u32>,
	slow_period: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	let fast_period = fast_period.unwrap_or(14) as usize;
	let slow_period = slow_period.unwrap_or(30) as usize;

	crate::utils::validation::validate_period(fast_period)?;
	crate::utils::validation::validate_period(slow_period)?;

	let fast_ema = ema_internal(values, fast_period);
	let slow_ema = ema_internal(values, slow_period);

	let result: Vec<f64> = fast_ema
		.iter()
		.enumerate()
		.map(|(i, f)| {
			let slow_val = slow_ema[i];
			if slow_val.is_nan() {
				f64::NAN
			} else {
				f - slow_val
			}
		})
		.collect();

	Ok(result)
}
