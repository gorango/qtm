use crate::utils::arrays::validate_arrays_equal_length;

pub fn validate_period(period: usize) -> Result<(), String> {
	if period == 0 {
		return Err("Period must be greater than 0".to_string());
	}
	Ok(())
}

pub fn validate_min_length(len: usize, min_len: usize) -> Result<(), String> {
	if len < min_len {
		return Err(format!(
			"Input array must have at least {} elements, got {}",
			min_len, len
		));
	}
	Ok(())
}

pub fn validate_multiple_arrays(arrays: &[&[f64]]) -> Result<(), String> {
	if arrays.is_empty() {
		return Err("At least one array must be provided".to_string());
	}

	validate_arrays_equal_length(arrays)
}
