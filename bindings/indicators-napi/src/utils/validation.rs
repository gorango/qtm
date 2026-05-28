use crate::utils::arrays::validate_arrays_equal_length;
use napi::bindgen_prelude::*;

pub fn validate_period(period: usize) -> Result<()> {
	if period == 0 {
		return Err(Error::new(
			Status::InvalidArg,
			"Period must be greater than 0",
		));
	}
	Ok(())
}

pub fn validate_min_length(len: usize, min_len: usize) -> Result<()> {
	if len < min_len {
		return Err(Error::new(
			Status::InvalidArg,
			format!(
				"Input array must have at least {} elements, got {}",
				min_len, len
			),
		));
	}
	Ok(())
}

pub fn validate_multiple_arrays(arrays: &[&Float64Array]) -> Result<()> {
	if arrays.is_empty() {
		return Err(Error::new(
			Status::InvalidArg,
			"At least one array must be provided",
		));
	}

	validate_arrays_equal_length(arrays)
}
