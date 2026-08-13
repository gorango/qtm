use crate::IndicatorResult;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ALMAConfig {
	pub period: Option<u32>,
	pub offset: Option<f64>,
	pub sigma: Option<f64>,
}

pub fn alma(values: &[f64], config: Option<ALMAConfig>) -> IndicatorResult<Vec<f64>> {
	let config = config.unwrap_or(ALMAConfig {
		period: Some(9),
		offset: Some(0.85),
		sigma: Some(6.0),
	});

	let period = config.period.unwrap_or(9) as usize;
	let offset = config.offset.unwrap_or(0.85);
	let sigma = config.sigma.unwrap_or(6.0);

	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_finite(&[values])?;

	let len = values.len();
	if len < period {
		return Ok(vec![f64::NAN; len]);
	}

	if period == 0 {
		return Ok(vec![f64::NAN; len]);
	}

	let m = offset * (period - 1) as f64;
	let s = period as f64 / sigma;

	let mut weights = Vec::with_capacity(period);
	let mut weight_sum = 0.0;

	for i in 0..period {
		let weight = ((-(i as f64 - m).powi(2)) / (2.0 * s * s)).exp();
		weights.push(weight);
		weight_sum += weight;
	}

	for w in &mut weights {
		*w /= weight_sum;
	}

	let mut result = vec![f64::NAN; len];

	for i in (period - 1)..len {
		let mut sum = 0.0;
		for j in 0..period {
			sum += weights[j] * values[i - (period - 1) + j];
		}
		result[i] = sum;
	}

	Ok(result)
}
