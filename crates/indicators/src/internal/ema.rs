use crate::internal::sma::sma_internal;

/// Exponential Moving Average (EMA) internal implementation.
///
/// Uses the smoothing factor `k = 2 / (period + 1)`. Seeds the initial values
/// with SMA then applies the recursive formula.
///
/// # Examples
/// ```
/// use indicators_core::ema_internal;
///
/// let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let result = ema_internal(&values, 3);
/// assert_eq!(result.len(), 5);
/// ```
pub fn ema_internal(values: &[f64], period: usize) -> Vec<f64> {
	let len = values.len();
	if len == 0 {
		return vec![];
	}

	let sma_vals = sma_internal(values, period);
	let mut result = vec![f64::NAN; len];

	for i in 0..sma_vals.len() {
		if !sma_vals[i].is_nan() {
			result[i] = sma_vals[i];
		}
	}

	if len > period {
		let k = 2.0 / (period + 1) as f64;
		let m = 1.0 - k;

		for i in period..len {
			result[i] = values[i] * k + result[i - 1] * m;
		}
	}

	result
}
