use crate::IndicatorResult;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LinRegConfig {
	pub period: Option<u32>,
	pub offset: Option<u32>,
}

/// Linear Regression (LinReg) — endpoint of the least-squares line over `period` bars.
///
/// Fits `y = a + b*x` to the last `period` values and returns the fitted value at the last bar.
/// Equivalent to the moving linear regression forecast.
///
/// # Errors
/// Returns an error if `period` is 0 or inputs invalid.
pub fn linreg(values: &[f64], config: Option<LinRegConfig>) -> IndicatorResult<Vec<f64>> {
	let config = config.unwrap_or(LinRegConfig {
		period: Some(14),
		offset: Some(0),
	});

	let period = config.period.unwrap_or(14) as usize;
	let offset = config.offset.unwrap_or(0) as usize;

	crate::utils::validation::validate_period(period)?;
	crate::utils::validation::validate_finite(&[values])?;

	let len = values.len();
	if len < period {
		return Ok(vec![f64::NAN; len]);
	}

	let mut result = vec![f64::NAN; len];

	for i in (period - 1)..len {
		let mut sum_x = 0.0;
		let mut sum_y = 0.0;
		let mut sum_xy = 0.0;
		let mut sum_xx = 0.0;
		let n = period as f64;

		for j in 0..period {
			let x = (period - 1 - j) as f64;
			let y = values[i - (period - 1) + j];
			sum_x += x;
			sum_y += y;
			sum_xy += x * y;
			sum_xx += x * x;
		}

		let denominator = n * sum_xx - sum_x * sum_x;
		if denominator.abs() > 1e-10 {
			let slope = (n * sum_xy - sum_x * sum_y) / denominator;
			let intercept = (sum_y - slope * sum_x) / n;
			result[i] = intercept + slope * (period - 1 - offset) as f64;
		}
	}

	Ok(result)
}
