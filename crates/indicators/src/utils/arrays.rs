use crate::error::{IndicatorError, IndicatorResult};

/// Validates that all arrays have equal length. Returns error if mismatched.
pub fn validate_arrays_equal_length(arrays: &[&[f64]]) -> IndicatorResult<()> {
	if arrays.is_empty() {
		return Ok(());
	}

	let len = arrays[0].len();
	for (i, &arr) in arrays.iter().enumerate() {
		if arr.len() != len {
			return Err(IndicatorError::ArrayLengthMismatch(format!(
				"Array at index {} has length {}, expected {}",
				i,
				arr.len(),
				len
			)));
		}
	}

	Ok(())
}

/// Allocates a `Vec<f64>` of length `len` filled with `NaN`.
pub fn init_result_array_nan(len: usize) -> Vec<f64> {
	vec![f64::NAN; len]
}

/// Clones `arr` into a `len`-sized vector; fills remainder with `NaN` if shorter.
pub fn clone_with_nan_fallback(arr: &[f64], len: usize) -> Vec<f64> {
	if arr.len() >= len {
		arr[..len].to_vec()
	} else {
		let mut result = arr.to_vec();
		result.resize(len, f64::NAN);
		result
	}
}
