use napi::bindgen_prelude::*;

/// Reject empty input collections.
pub(crate) fn validate_non_empty<T>(items: &[T], name: &str) -> Result<()> {
	if items.is_empty() {
		return Err(napi::Error::from_reason(format!(
			"{name} must not be empty"
		)));
	}
	Ok(())
}

/// Reject zero periods.
pub(crate) fn validate_period(period: u32, name: &str) -> Result<()> {
	if period == 0 {
		return Err(napi::Error::from_reason(format!(
			"{name} must be greater than zero"
		)));
	}
	Ok(())
}

/// Reject empty inputs and arrays of mismatched length.
pub(crate) fn validate_arrays(arrays: &[&[f64]], names: &[&str]) -> Result<()> {
	if arrays.is_empty() {
		return Err(napi::Error::from_reason("no input arrays provided"));
	}
	let len = arrays[0].len();
	for (&arr, name) in arrays.iter().zip(names) {
		if arr.is_empty() {
			return Err(napi::Error::from_reason(format!(
				"{name} must not be empty"
			)));
		}
		if arr.len() != len {
			return Err(napi::Error::from_reason(format!(
				"{name} has length {}, expected {}",
				arr.len(),
				len
			)));
		}
	}
	Ok(())
}
