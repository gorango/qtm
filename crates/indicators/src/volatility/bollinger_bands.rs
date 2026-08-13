use crate::internal::moving_std::std_dev_internal;
use crate::internal::sma::sma_internal;
use crate::IndicatorResult;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct BBResult {
	pub upper: Vec<f64>,
	pub middle: Vec<f64>,
	pub lower: Vec<f64>,
}

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BBConfig {
	pub period: Option<u32>,
	pub std_dev: Option<f64>,
}

impl Default for BBConfig {
	fn default() -> Self {
		Self {
			period: Some(20),
			std_dev: Some(2.0),
		}
	}
}

/// Bollinger Bands.
///
/// Calculates upper and lower bands at `std_dev` standard deviations around a SMA middle band.
///
/// # Errors
/// Returns an error if `period` is 0.
///
/// # Examples
/// ```
/// use indicators_core::{bb, BBConfig};
///
/// let closings = vec![10.0, 11.0, 12.0, 13.0, 14.0];
/// let result = bb(&closings, None).unwrap();
/// assert_eq!(result.upper.len(), 5);
/// assert_eq!(result.middle.len(), 5);
/// assert_eq!(result.lower.len(), 5);
/// ```
pub fn bb(closings: &[f64], config: Option<BBConfig>) -> IndicatorResult<BBResult> {
	let len = closings.len();

	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20) as usize;
	let std_dev_multiplier = config.std_dev.unwrap_or(2.0);
	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_finite(&[closings])?;

	let std_dev_array = std_dev_internal(closings, period);
	let middle = sma_internal(closings, period);

	let mut upper = vec![f64::NAN; len];
	let mut lower = vec![f64::NAN; len];

	for i in 0..len {
		if !std_dev_array[i].is_nan() && !middle[i].is_nan() {
			upper[i] = middle[i] + std_dev_array[i] * std_dev_multiplier;
			lower[i] = middle[i] - std_dev_array[i] * std_dev_multiplier;
		}
	}

	Ok(BBResult {
		upper,
		middle,
		lower,
	})
}

pub fn bollinger_bands(closings: &[f64], config: Option<BBConfig>) -> IndicatorResult<BBResult> {
	bb(closings, config)
}
