use crate::IndicatorResult;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MSTDConfig {
	pub period: Option<u32>,
}

impl Default for MSTDConfig {
	fn default() -> Self {
		Self { period: Some(4) }
	}
}

/// Moving Standard Deviation — `mstd` short alias. Rolling population std over `period` bars. `NaN` for first `period - 1` bars.
pub fn mstd(values: &[f64], config: Option<MSTDConfig>) -> IndicatorResult<Vec<f64>> {
	let len = values.len();

	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(4) as usize;
	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_finite(&[values])?;

	let mut result = vec![f64::NAN; len];

	let start_idx = period - 1;
	for i in start_idx..len {
		let window_start = i - (period - 1);
		let window = &values[window_start..=i];
		let mut mean = 0.0;
		let mut m2 = 0.0;

		for (idx, &x) in window.iter().enumerate() {
			let count = idx + 1;
			let count_f = count as f64;
			let delta = x - mean;
			mean += delta / count_f;
			let delta2 = x - mean;
			m2 += delta * delta2;
		}

		if !mean.is_nan() {
			let variance = m2 / period as f64;
			result[i] = variance.sqrt();
		}
	}

	Ok(result)
}

/// Moving Standard Deviation — rolling population standard deviation. Full-name alias for `mstd`.
/// Welford's method per window. Period defaults to 4. See `mstd`.
pub fn moving_standard_deviation(
	values: &[f64],
	config: Option<MSTDConfig>,
) -> IndicatorResult<Vec<f64>> {
	mstd(values, config)
}
