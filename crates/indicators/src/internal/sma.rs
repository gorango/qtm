/// Simple Moving Average (SMA) internal implementation.
///
/// Calculates the arithmetic mean over a sliding window of `period` elements.
/// Returns `NaN` for indices where the window is not yet full.
///
/// # Panics
/// Never panics.
///
/// # Examples
/// ```
/// use indicators_core::sma_internal;
///
/// let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let result = sma_internal(&values, 3);
/// assert_eq!(result.len(), 5);
/// assert!(result[0].is_nan());
/// assert!((result[4] - 4.0).abs() < 1e-10);
/// ```
pub fn sma_internal(values: &[f64], period: usize) -> Vec<f64> {
	let len = values.len();
	if len < period || period == 0 {
		return vec![f64::NAN; len];
	}

	let mut result = vec![f64::NAN; len];
	let mut sum = 0.0;
	let mut nan_in_window = 0usize;

	for i in 0..len {
		if values[i].is_nan() {
			nan_in_window += 1;
		} else {
			sum += values[i];
		}
		if i >= period {
			let leaving = values[i - period];
			if leaving.is_nan() {
				nan_in_window -= 1;
			} else {
				sum -= leaving;
			}
		}
		if i >= period - 1 && nan_in_window == 0 {
			result[i] = sum / period as f64;
		}
	}

	result
}
