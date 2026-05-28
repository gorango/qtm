use crate::utils::validation;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct PercentileNearestRankConfig {
	pub period: Option<u32>,
	pub percentage: Option<f64>,
}

fn percentile_nearest_rank_internal(values: &[f64], period: usize, percentage: f64) -> Vec<f64> {
	let len = values.len();
	if len < period {
		return vec![f64::NAN; len];
	}

	let mut result = vec![f64::NAN; len];

	for i in period - 1..len {
		let mut window: Vec<f64> = values[i - period + 1..=i].to_vec();
		window.sort_by(|a, b| a.partial_cmp(b).unwrap());

		let p = percentage / 100.0;
		let mut index = (p * period as f64).ceil() as usize - 1;

		if index >= period {
			index = period - 1;
		}

		result[i] = window[index];
	}

	result
}

pub fn percentile_nearest_rank(
	values: &[f64],
	config: Option<PercentileNearestRankConfig>,
) -> Result<Vec<f64>, String> {
	let PercentileNearestRankConfig { period, percentage } =
		config.unwrap_or(PercentileNearestRankConfig {
			period: None,
			percentage: None,
		});
	let period = period.unwrap_or(14) as usize;
	let percentage = percentage.unwrap_or(50.0);

	validation::validate_period(period)?;

	Ok(percentile_nearest_rank_internal(values, period, percentage))
}
