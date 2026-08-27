use crate::error::{IndicatorError, IndicatorResult};
use crate::utils::arrays::validate_arrays_equal_length;

/// Checks that `value` is in [`min`, `max`].
/// `err` is a callable `|msg: String| -> E` that constructs the error.
#[macro_export]
macro_rules! validate_range {
	($value:expr, $min:expr, $max:expr, $name:expr, $err:expr) => {
		if $value < $min {
			return Err(($err)(format!("{} must be at least {}", $name, $min)));
		}
		if $value > $max {
			return Err(($err)(format!("{} must be at most {}", $name, $max)));
		}
	};
}

/// Checks that `len >= min_len`.
/// `err` is a callable `|min, actual| -> E` that constructs the error.
#[macro_export]
macro_rules! validate_min_data {
	($len:expr, $min_len:expr, $err:expr) => {
		if $len < $min_len {
			return Err(($err)($min_len, $len));
		}
	};
}

/// Validates `period > 0`. Returns error if 0.
pub fn validate_period(period: usize) -> IndicatorResult<()> {
	if period == 0 {
		return Err(IndicatorError::InvalidPeriod(period));
	}
	Ok(())
}

/// Validates `len >= min_len`.
pub fn validate_min_length(len: usize, min_len: usize) -> IndicatorResult<()> {
	if len < min_len {
		return Err(IndicatorError::InsufficientData {
			min: min_len,
			actual: len,
		});
	}
	Ok(())
}

/// Validates that multiple arrays are equal length and finite.
pub fn validate_multiple_arrays(arrays: &[&[f64]]) -> IndicatorResult<()> {
	if arrays.is_empty() {
		return Err(IndicatorError::EmptyInput);
	}
	validate_arrays_equal_length(arrays)
}

/// Returns an error if any value in any input array is NaN or infinite.
///
/// This validates INPUT data only. NaN warmup in indicator OUTPUT (the first
/// `period - 1` values) is standard behavior and must not be treated as an error.
/// Validates that all values are finite (no NaN/inf).
pub fn validate_finite(arrays: &[&[f64]]) -> IndicatorResult<()> {
	for arr in arrays {
		if arr.iter().any(|v| !v.is_finite()) {
			return Err(IndicatorError::Custom(
				"Input contains a non-finite value (NaN or Infinity)".into(),
			));
		}
	}
	Ok(())
}
