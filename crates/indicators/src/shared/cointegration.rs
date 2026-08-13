use crate::utils::arrays::validate_arrays_equal_length;
use crate::utils::validation;
use crate::IndicatorResult;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CointegrationConfig {
	pub period: Option<u32>,
	pub beta_period: Option<u32>,
}

fn ols_beta(y: &[f64], x: &[f64]) -> f64 {
	let n = y.len();
	if n != x.len() || n < 2 {
		return f64::NAN;
	}

	let mut sum_x = 0.0;
	let mut sum_y = 0.0;
	let mut sum_xy = 0.0;
	let mut sum_x2 = 0.0;

	for i in 0..n {
		if y[i].is_nan() || x[i].is_nan() {
			return f64::NAN;
		}
		sum_x += x[i];
		sum_y += y[i];
		sum_xy += x[i] * y[i];
		sum_x2 += x[i] * x[i];
	}

	let numerator = n as f64 * sum_xy - sum_x * sum_y;
	let denominator = n as f64 * sum_x2 - sum_x * sum_x;

	if denominator.abs() > 1e-10 {
		numerator / denominator
	} else {
		f64::NAN
	}
}

fn cointegration_internal(
	values1: &[f64],
	values2: &[f64],
	period: usize,
	beta_period: usize,
) -> Vec<f64> {
	let len = values1.len();
	if len < period.max(beta_period) {
		return vec![f64::NAN; len];
	}

	let mut betas = vec![f64::NAN; len];

	for i in beta_period - 1..len {
		let x = &values2[i - beta_period + 1..=i];
		let y = &values1[i - beta_period + 1..=i];
		betas[i] = ols_beta(y, x);
	}

	let mut spreads = vec![f64::NAN; len];
	for i in 0..len {
		let beta = betas[i];
		if !beta.is_nan() {
			spreads[i] = values1[i] - beta * values2[i];
		}
	}

	let mut result = vec![f64::NAN; len];
	for i in period - 1..len {
		let window: Vec<f64> = spreads[i - period + 1..=i]
			.iter()
			.filter(|v| !v.is_nan())
			.copied()
			.collect();

		if window.len() < 2 {
			continue;
		}

		let mean = window.iter().sum::<f64>() / window.len() as f64;
		let variance =
			window.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (window.len() - 1) as f64;
		let std = variance.sqrt();

		if std > 0.0 {
			result[i] = (spreads[i] - mean) / std;
		}
	}

	result
}

pub fn cointegration(
	values1: &[f64],
	values2: &[f64],
	config: Option<CointegrationConfig>,
) -> IndicatorResult<Vec<f64>> {
	let CointegrationConfig {
		period,
		beta_period,
	} = config.unwrap_or(CointegrationConfig {
		period: None,
		beta_period: None,
	});
	let period = period.unwrap_or(20) as usize;
	let beta_period = beta_period.unwrap_or(60) as usize;

	validation::validate_period(period)?;
	validation::validate_period(beta_period)?;
	validate_arrays_equal_length(&[values1, values2])?;

	Ok(cointegration_internal(
		values1,
		values2,
		period,
		beta_period,
	))
}
