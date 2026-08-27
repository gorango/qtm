use crate::utils::validation;
use crate::IndicatorResult;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
/// Percent rank config.
pub struct PercentRankConfig {
	/// Lookback period (default 20-60). Valid 2..=500.
	pub period: Option<u32>,
}

fn percent_rank_internal(values: &[f64], period: usize) -> Vec<f64> {
	let len = values.len();
	if len < period + 1 {
		return vec![f64::NAN; len];
	}

	let mut result = vec![f64::NAN; len];

	for i in period..len {
		let current_value = values[i];
		if current_value.is_nan() {
			result[i] = f64::NAN;
			continue;
		}

		let mut count = 0;
		let mut valid_values = 0;

		for j in 1..=period {
			let val = values[i - j];
			if val.is_nan() {
				continue;
			}
			valid_values += 1;
			if val <= current_value {
				count += 1;
			}
		}

		if valid_values == 0 {
			result[i] = f64::NAN;
		} else {
			result[i] = (count as f64 / valid_values as f64) * 100.0;
		}
	}

	result
}

/// Percent Rank — `(rank of current value within period) / period * 100`. Range 0..100.
pub fn percent_rank(
	values: &[f64],
	config: Option<PercentRankConfig>,
) -> IndicatorResult<Vec<f64>> {
	let PercentRankConfig { period } = config.unwrap_or(PercentRankConfig { period: None });
	let period = period.unwrap_or(14) as usize;

	validation::validate_period(period)?;

	Ok(percent_rank_internal(values, period))
}
