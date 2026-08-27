use crate::internal::moving_std::std_dev_internal;
use crate::internal::sma::sma_internal;
use crate::{IndicatorError, IndicatorResult};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ZScoreConfig {
	pub period: Option<u32>,
}

impl Default for ZScoreConfig {
	fn default() -> Self {
		Self { period: Some(20) }
	}
}

/// Z-Score — `zs` short alias. `(value - SMA) / std_dev` over `period` bars.
/// Standardized distance from the mean. `NaN` where std is 0 or `NaN`.
pub fn zs(values: &[f64], config: Option<ZScoreConfig>) -> IndicatorResult<Vec<f64>> {
	let len = values.len();

	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(20) as usize;

	if len < period {
		return Err(IndicatorError::Custom(format!(
			"Not enough data points. Need at least {period}, got {len}"
		)));
	}

	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_finite(&[values])?;

	let means = sma_internal(values, period);
	let stds = std_dev_internal(values, period);

	let mut result = vec![0.0; len];

	for i in 0..len {
		let mean = means[i];
		let std = stds[i];

		if !mean.is_nan() && !std.is_nan() && std > 0.0 {
			result[i] = (values[i] - mean) / std;
		}
	}

	Ok(result)
}

/// Z-Score — `(value - SMA(period)) / std_dev(period)`. Full-name alias for `zs`.
/// See `zs` for details.
pub fn z_score(values: &[f64], config: Option<ZScoreConfig>) -> IndicatorResult<Vec<f64>> {
	zs(values, config)
}
