use napi::bindgen_prelude::*;

#[allow(dead_code)]
pub fn vec_to_float64array(_env: &Env, vec: Vec<f64>) -> Result<Float64Array> {
	Ok(Float64Array::new(vec))
}

pub fn validate_arrays_equal_length(arrays: &[&Float64Array]) -> Result<()> {
	if arrays.is_empty() {
		return Ok(());
	}

	let len = arrays[0].len();
	for (i, &arr) in arrays.iter().enumerate() {
		if arr.len() != len {
			return Err(Error::new(
				Status::InvalidArg,
				format!(
					"Array at index {} has length {}, expected {}",
					i,
					arr.len(),
					len
				),
			));
		}
	}

	Ok(())
}

#[allow(dead_code)]
pub fn init_result_array_nan(len: usize) -> Vec<f64> {
	vec![f64::NAN; len]
}

#[allow(dead_code)]
pub fn clone_with_nan_fallback(arr: &[f64], len: usize) -> Vec<f64> {
	if arr.len() >= len {
		arr[..len].to_vec()
	} else {
		let mut result = arr.to_vec();
		result.resize(len, f64::NAN);
		result
	}
}
