use crate::internal::moving_sum::moving_sum_internal;
use crate::{IndicatorError, IndicatorResult};

/// Volume-Weighted Moving Average (VWMA).
///
/// `sum(close * volume) / sum(volume)` over `period` bars. Like VWAP but rolling window on closes.
/// Period defaults to 20. `NaN` for first `period - 1` bars.
///
/// # Errors
/// Returns an error if `period` is 0 or inputs invalid/mismatched.
pub fn vwma(closings: &[f64], volumes: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(20) as usize;

	crate::utils::validation::validate_multiple_arrays(&[closings, volumes])?;
	crate::utils::validation::validate_finite(&[closings, volumes])?;

	if closings.is_empty() {
		return Err(IndicatorError::Custom("Arrays cannot be empty".into()));
	}

	let price_volume: Vec<f64> = closings
		.iter()
		.enumerate()
		.map(|(i, close)| close * volumes[i])
		.collect();

	let price_volume_sum = moving_sum_internal(&price_volume, period);
	let volume_sum = moving_sum_internal(volumes, period);

	let result: Vec<f64> = price_volume_sum
		.iter()
		.enumerate()
		.map(|(i, pv)| {
			let v = volume_sum[i];
			if v != 0.0 {
				pv / v
			} else {
				0.0
			}
		})
		.collect();

	Ok(result)
}
