use crate::IndicatorResult;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MeanAbsoluteDeviationConfig {
	pub period: Option<u32>,
}

impl Default for MeanAbsoluteDeviationConfig {
	fn default() -> Self {
		Self { period: Some(14) }
	}
}

/// Mean Absolute Deviation — `dev` short alias.
/// Mean of `|value - SMA(value, period)|` over `period` bars. Used inside CCI.
pub fn dev(
	values: &[f64],
	config: Option<MeanAbsoluteDeviationConfig>,
) -> IndicatorResult<Vec<f64>> {
	let len = values.len();

	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(14) as usize;
	crate::utils::validation::validate_period(period)?;

	let mut result = vec![f64::NAN; len];

	if len >= period {
		for i in period - 1..len {
			let mut sum = 0.0;

			for j in 0..period {
				sum += values[i - (period - 1) + j];
			}

			let mean = sum / period as f64;
			let mut sum_deviation = 0.0;

			for j in 0..period {
				sum_deviation += (values[i - (period - 1) + j] - mean).abs();
			}

			result[i] = sum_deviation / period as f64;
		}
	}

	Ok(result)
}

/// Mean Absolute Deviation (MAD) — average absolute deviation from the SMA.
///
/// `MAD = mean(|value[i] - SMA(value, period)|)` over `period` bars. Used as denominator in CCI.
/// Period defaults to 20. `NaN` for first `period - 1` bars.
///
/// # Errors
/// Returns an error if `period` is 0 or inputs invalid.
pub fn mean_absolute_deviation(
	values: &[f64],
	config: Option<MeanAbsoluteDeviationConfig>,
) -> IndicatorResult<Vec<f64>> {
	dev(values, config)
}
