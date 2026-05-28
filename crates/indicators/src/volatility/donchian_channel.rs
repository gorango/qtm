use crate::utils::rolling::rolling_max_growing;
use crate::utils::rolling::rolling_min_growing;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct DCResult {
	pub upper: Vec<f64>,
	pub middle: Vec<f64>,
	pub lower: Vec<f64>,
}

pub fn dc(closings: &[f64], period: Option<u32>) -> Result<DCResult, String> {
	let len = closings.len();

	if len == 0 {
		return Err("Closings array cannot be empty".to_string());
	}

	let period = period.unwrap_or(4) as usize;
	crate::utils::validation::validate_period(period)?;

	let upper = rolling_max_growing(closings, period);
	let lower = rolling_min_growing(closings, period);
	let mut middle = vec![0.0; len];

	for i in 0..len {
		let upper_val = upper[i];
		let lower_val = lower[i];

		middle[i] = (upper_val + lower_val) / 2.0;
	}

	Ok(DCResult {
		upper,
		middle,
		lower,
	})
}

pub fn donchian_channel(closings: &[f64], period: Option<u32>) -> Result<DCResult, String> {
	dc(closings, period)
}
