use crate::IndicatorResult;
/// RMA kernel — Wilder's smoothing. `O(n)`, no validation; callers must validate.
/// Recurrence: `prev = (prev*(period-1) + value[i]) / period` after seeding with SMA.
pub fn rma_internal(values: &[f64], period: usize) -> Vec<f64> {
	let len = values.len();
	let mut result = vec![0.0; len];
	let mut sum = 0.0;

	for i in 0..len {
		let count = if i < period {
			sum += values[i];
			(i + 1) as f64
		} else {
			sum = result[i - 1] * (period - 1) as f64 + values[i];
			period as f64
		};

		result[i] = sum / count;
	}

	result
}

/// Wilder's Smoothed Moving Average (RMA / SMMA).
///
/// Recursive: `RMA[i] = (RMA[i-1]*(period-1) + value[i]) / period`. Equivalent to EMA with alpha=1/period but seeded differently. Used inside RSI/ADX. `rma_internal` is the kernel.
///
/// # Errors
/// Returns an error if `period` is 0 or inputs contain non-finite values.
pub fn rma(values: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(4) as usize;
	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_finite(&[values])?;
	Ok(rma_internal(values, period))
}
