use crate::utils::arrays::validate_arrays_equal_length;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct ValueWhenConfig {
	pub occurrence: Option<u32>,
}

fn value_when_internal(condition: &[f64], source: &[f64], occurrence: usize) -> Vec<f64> {
	let len = condition.len();
	let mut result = vec![f64::NAN; len];

	for (i, result_val) in result.iter_mut().enumerate().take(len) {
		let mut count = 0;
		for j in (0..=i).rev() {
			if condition[j] != 0.0 {
				count += 1;
				if count == occurrence {
					*result_val = source[j];
					break;
				}
			}
		}
	}

	result
}

pub fn value_when(
	condition: &[f64],
	source: &[f64],
	config: Option<ValueWhenConfig>,
) -> Result<Vec<f64>, String> {
	let ValueWhenConfig { occurrence } = config.unwrap_or(ValueWhenConfig { occurrence: None });
	let occurrence = occurrence.unwrap_or(1) as usize;

	validate_arrays_equal_length(&[condition, source])?;

	Ok(value_when_internal(condition, source, occurrence))
}
