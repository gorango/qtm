use crate::error::{IndicatorError, IndicatorResult};
use crate::utils::arrays::validate_arrays_equal_length;

pub fn validate_period(period: usize) -> IndicatorResult<()> {
	if period == 0 {
		return Err(IndicatorError::InvalidPeriod(period));
	}
	Ok(())
}

pub fn validate_min_length(len: usize, min_len: usize) -> IndicatorResult<()> {
	if len < min_len {
		return Err(IndicatorError::InsufficientData {
			min: min_len,
			actual: len,
		});
	}
	Ok(())
}

pub fn validate_multiple_arrays(arrays: &[&[f64]]) -> IndicatorResult<()> {
	if arrays.is_empty() {
		return Err(IndicatorError::EmptyInput);
	}
	validate_arrays_equal_length(arrays)
}
