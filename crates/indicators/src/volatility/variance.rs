use crate::IndicatorResult;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VarianceConfig {
	pub period: Option<u32>,
}

impl Default for VarianceConfig {
	fn default() -> Self {
		Self { period: Some(14) }
	}
}

/// Rolling Variance — variance over `period` bars. Thin wrapper with validation.
/// Population variance (`/ period`). `NaN` for first `period - 1` bars.
///
/// # Errors
/// Returns an error if `period` is 0 or inputs invalid.
pub fn variance(values: &[f64], config: Option<VarianceConfig>) -> IndicatorResult<Vec<f64>> {
	let len = values.len();

	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14) as usize;
	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_finite(&[values])?;

	let mut result = vec![f64::NAN; len];

	if len >= period {
		for i in period - 1..len {
			let mut sum = 0.0;
			let mut sum_squares = 0.0;

			for j in 0..period {
				let val = values[i - (period - 1) + j];
				sum += val;
				sum_squares += val * val;
			}

			let mean = sum / period as f64;
			result[i] = sum_squares / period as f64 - mean * mean;
		}
	}

	Ok(result)
}

/// Alias for `variance` — rolling variance (full name).
pub fn rolling_variance(
	values: &[f64],
	config: Option<VarianceConfig>,
) -> IndicatorResult<Vec<f64>> {
	variance(values, config)
}
