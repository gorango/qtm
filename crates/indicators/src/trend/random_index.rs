use crate::internal::sma::sma_internal;
use crate::IndicatorResult;
use serde::{Deserialize, Serialize};

fn kdj_moving_max_internal(values: &[f64], period: usize) -> Vec<f64> {
	let len = values.len();
	let mut result = vec![0.0; len];

	for i in 0..len {
		let mut max = values[i];
		let start = if i >= period { i - period + 1 } else { 0 };

		for &val in &values[start..=i] {
			if val > max {
				max = val;
			}
		}

		result[i] = max;
	}

	result
}

fn kdj_moving_min_internal(values: &[f64], period: usize) -> Vec<f64> {
	let len = values.len();
	let mut result = vec![0.0; len];

	for i in 0..len {
		let mut min = values[i];
		let start = if i >= period { i - period + 1 } else { 0 };

		for &val in &values[start..=i] {
			if val < min {
				min = val;
			}
		}

		result[i] = min;
	}

	result
}

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct KDJResult {
	pub k: Vec<f64>,
	pub d: Vec<f64>,
	pub j: Vec<f64>,
}

pub fn random_index(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	r_period: Option<u32>,
	k_period: Option<u32>,
	d_period: Option<u32>,
) -> IndicatorResult<KDJResult> {
	crate::utils::validation::validate_multiple_arrays(&[highs, lows, closings])?;

	let r_period = r_period.unwrap_or(9) as usize;
	let k_period = k_period.unwrap_or(3) as usize;
	let d_period = d_period.unwrap_or(3) as usize;

	let highest = kdj_moving_max_internal(highs, r_period);
	let lowest = kdj_moving_min_internal(lows, r_period);

	let rsv: Vec<f64> = closings
		.iter()
		.enumerate()
		.map(|(i, close)| {
			let high = highest[i];
			let low = lowest[i];
			let denominator = high - low;
			if denominator.abs() > 1e-10 {
				((close - low) / denominator) * 100.0
			} else {
				0.0
			}
		})
		.collect();

	let k_value = sma_internal(&rsv, k_period);
	let d_value = sma_internal(&k_value, d_period);
	let j_value: Vec<f64> = k_value
		.iter()
		.enumerate()
		.map(|(i, k)| 3.0 * k - 2.0 * d_value[i])
		.collect();

	Ok(KDJResult {
		k: k_value,
		d: d_value,
		j: j_value,
	})
}
