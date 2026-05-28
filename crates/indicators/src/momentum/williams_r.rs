use crate::trend::moving_max::moving_max_internal;
use crate::trend::moving_min::moving_min_internal;
use crate::utils::validation;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct WilliamsRConfig {
	pub period: Option<u32>,
}

pub fn williams_r(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	config: Option<WilliamsRConfig>,
) -> Result<Vec<f64>, String> {
	let config_obj = config.unwrap_or(WilliamsRConfig { period: None });
	let period = config_obj.period.unwrap_or(14) as usize;

	validation::validate_period(period)?;

	validation::validate_multiple_arrays(&[highs, lows, closings])?;

	let len = highs.len();

	let highest_high = moving_max_internal(highs, period);
	let lowest_low = moving_min_internal(lows, period);

	let mut result = vec![0.0; len];

	for i in 0..len {
		let hh = highest_high[i];
		let ll = lowest_low[i];
		let close = closings[i];

		let numerator = hh - close;
		let denominator = hh - ll;

		if denominator.abs() > 1e-10 {
			result[i] = (numerator / denominator) * -100.0;
		} else {
			result[i] = 0.0;
		}
	}

	Ok(result)
}
