use crate::internal::sma::sma_internal;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct ABResult {
	pub upper: Vec<f64>,
	pub middle: Vec<f64>,
	pub lower: Vec<f64>,
}

pub fn ab(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	period: Option<u32>,
	multiplier: Option<f64>,
) -> Result<ABResult, String> {
	crate::utils::validation::validate_multiple_arrays(&[highs, lows, closings])?;

	let len = highs.len();

	if len == 0 {
		return Err("Highs, lows, and closings arrays cannot be empty".to_string());
	}

	let period = period.unwrap_or(20) as usize;
	let multiplier = multiplier.unwrap_or(4.0);
	crate::utils::validation::validate_period(period)?;

	let mut k = vec![0.0; len];

	for i in 0..len {
		let high = highs[i];
		let low = lows[i];
		let denominator = high + low;
		k[i] = if denominator.abs() > 1e-10 {
			(high - low) / denominator
		} else {
			0.0
		};
	}

	let mut upper_values = vec![0.0; len];
	let mut lower_values = vec![0.0; len];

	for i in 0..len {
		let high = highs[i];
		let low = lows[i];
		let k_val = k[i];
		upper_values[i] = high * (1.0 + multiplier * k_val);
		lower_values[i] = low * (1.0 - multiplier * k_val);
	}

	let upper = sma_internal(&upper_values, period);
	let middle = sma_internal(closings, period);
	let lower = sma_internal(&lower_values, period);

	Ok(ABResult {
		upper,
		middle,
		lower,
	})
}

pub fn acceleration_bands(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	period: Option<u32>,
	multiplier: Option<f64>,
) -> Result<ABResult, String> {
	ab(highs, lows, closings, period, multiplier)
}
