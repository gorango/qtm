use crate::IndicatorResult;
/// WMA kernel — weighted mean over `period` bars. `O(n*period)`, no validation; callers must validate.
/// Weights are `period, period-1, ..., 1` applied to the most-recent-first window.
pub fn wma_internal(values: &[f64], period: usize) -> Vec<f64> {
	let len = values.len();
	let mut result = vec![f64::NAN; len];
	if period == 0 || period > len {
		return result;
	}
	if period > usize::MAX / 2 {
		return result;
	}
	let sum_weights = (period * (period + 1) / 2) as f64;

	for i in (period - 1)..len {
		let mut sum = 0.0;
		for j in 0..period {
			let weight = (period - j) as f64;
			sum += weight * values[i - (period - 1) + j];
		}
		result[i] = sum / sum_weights;
	}

	result
}

/// Weighted Moving Average (WMA).
///
/// Linearly weighted mean where weight `period` is given to the most recent value and 1 to the oldest. Sum of weights = `period*(period+1)/2`. Direct implementation. Period defaults to 14. `wma_internal` is the `O(n*period)` kernel.
///
/// # Errors
/// Returns an error if `period` is 0 or inputs contain non-finite values.
pub fn wma(values: &[f64], period: Option<u32>) -> IndicatorResult<Vec<f64>> {
	let period = period.unwrap_or(14) as usize;
	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_finite(&[values])?;
	Ok(wma_internal(values, period))
}
