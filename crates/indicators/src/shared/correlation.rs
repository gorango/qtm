use crate::utils::arrays::validate_arrays_equal_length;
use crate::utils::validation;
use crate::IndicatorResult;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CorrelationConfig {
	pub period: Option<u32>,
}

fn correlation_internal(values1: &[f64], values2: &[f64], period: usize) -> Vec<f64> {
	let len = values1.len();
	if len < period {
		return vec![f64::NAN; len];
	}

	let mut result = vec![f64::NAN; len];

	for i in period - 1..len {
		let mut sum_x = 0.0;
		let mut sum_y = 0.0;
		let mut sum_xy = 0.0;
		let mut sum_x2 = 0.0;
		let mut sum_y2 = 0.0;
		let mut count = 0;

		for j in 0..period {
			let x = values1[i - period + 1 + j];
			let y = values2[i - period + 1 + j];

			if x.is_nan() || y.is_nan() {
				continue;
			}

			sum_x += x;
			sum_y += y;
			sum_xy += x * y;
			sum_x2 += x * x;
			sum_y2 += y * y;
			count += 1;
		}

		if count < 2 {
			result[i] = f64::NAN;
			continue;
		}

		let numerator = count as f64 * sum_xy - sum_x * sum_y;
		let denominator_x = count as f64 * sum_x2 - sum_x * sum_x;
		let denominator_y = count as f64 * sum_y2 - sum_y * sum_y;

		if denominator_x <= 1e-10 || denominator_y <= 1e-10 {
			result[i] = 0.0;
		} else {
			result[i] = numerator / (denominator_x * denominator_y).sqrt();
		}
	}

	result
}

pub fn correlation(
	values1: &[f64],
	values2: &[f64],
	config: Option<CorrelationConfig>,
) -> IndicatorResult<Vec<f64>> {
	let CorrelationConfig { period } = config.unwrap_or(CorrelationConfig { period: None });
	let period = period.unwrap_or(14) as usize;

	validation::validate_period(period)?;
	validate_arrays_equal_length(&[values1, values2])?;

	Ok(correlation_internal(values1, values2, period))
}

pub fn pearson_correlation(
	values1: &[f64],
	values2: &[f64],
	config: Option<CorrelationConfig>,
) -> IndicatorResult<Vec<f64>> {
	correlation(values1, values2, config)
}
