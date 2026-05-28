use crate::utils::validation;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct PercentileLinearInterpolationConfig {
	pub period: Option<u32>,
	pub percentage: Option<f64>,
}

fn percentile_linear_interpolation_internal(
	values: &[f64],
	period: usize,
	percentage: f64,
) -> Vec<f64> {
	let len = values.len();
	if len < period {
		return vec![f64::NAN; len];
	}

	let mut result = vec![f64::NAN; len];

	for i in period - 1..len {
		let mut window: Vec<f64> = values[i - period + 1..=i].to_vec();
		window.sort_by(|a, b| a.partial_cmp(b).unwrap());

		let p = percentage / 100.0;
		let mut index = p * period as f64 - 0.5;

		if index < 0.0 {
			index = 0.0;
		}
		if index > (period - 1) as f64 {
			index = (period - 1) as f64;
		}

		let lower_index = index.floor() as usize;
		let upper_index = index.ceil() as usize;

		if lower_index == upper_index {
			result[i] = window[lower_index];
		} else {
			let fraction = index - lower_index as f64;
			result[i] =
				window[lower_index] + fraction * (window[upper_index] - window[lower_index]);
		}
	}

	result
}

pub fn percentile_linear_interpolation(
	values: &[f64],
	config: Option<PercentileLinearInterpolationConfig>,
) -> Result<Vec<f64>, String> {
	let PercentileLinearInterpolationConfig { period, percentage } =
		config.unwrap_or(PercentileLinearInterpolationConfig {
			period: None,
			percentage: None,
		});
	let period = period.unwrap_or(14) as usize;
	let percentage = percentage.unwrap_or(50.0);

	validation::validate_period(period)?;

	Ok(percentile_linear_interpolation_internal(
		values, period, percentage,
	))
}
