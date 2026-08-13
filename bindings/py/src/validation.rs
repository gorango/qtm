use pyo3::prelude::*;

use crate::convert::err;

/// Reject empty input collections.
pub fn validate_non_empty<T>(items: &[T], name: &str) -> PyResult<()> {
	if items.is_empty() {
		Err(err(format!("{name} must not be empty")))
	} else {
		Ok(())
	}
}

/// Reject zero periods.
pub fn validate_period(period: u32, name: &str) -> PyResult<()> {
	if period == 0 {
		Err(err(format!("{name} must be greater than zero")))
	} else {
		Ok(())
	}
}

/// Reject empty inputs and arrays of mismatched length.
pub fn validate_arrays<'a, I>(arrays: I) -> PyResult<()>
where
	I: IntoIterator<Item = (&'a Vec<f64>, &'a str)>,
{
	let mut it = arrays.into_iter();
	let (first, first_name) = it.next().expect("at least one array");
	if first.is_empty() {
		return Err(err(format!("{first_name} must not be empty")));
	}
	let len = first.len();
	for (arr, name) in it {
		if arr.is_empty() {
			return Err(err(format!("{name} must not be empty")));
		}
		if arr.len() != len {
			return Err(err(format!(
				"{name} has length {}, expected {}",
				arr.len(),
				len
			)));
		}
	}
	Ok(())
}
