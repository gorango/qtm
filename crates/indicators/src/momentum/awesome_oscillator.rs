use crate::internal::sma::sma_internal;
use crate::utils::validation;
use crate::IndicatorResult;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AwesomeOscillatorConfig {
	pub fast_period: Option<u32>,
	pub slow_period: Option<u32>,
}

pub fn awesome_oscillator(
	highs: &[f64],
	lows: &[f64],
	config: Option<AwesomeOscillatorConfig>,
) -> IndicatorResult<Vec<f64>> {
	let config_obj = config.unwrap_or(AwesomeOscillatorConfig {
		fast_period: None,
		slow_period: None,
	});

	let fast_period = config_obj.fast_period.unwrap_or(5) as usize;
	let slow_period = config_obj.slow_period.unwrap_or(34) as usize;

	validation::validate_period(fast_period)?;
	validation::validate_period(slow_period)?;

	validation::validate_multiple_arrays(&[highs, lows])?;

	let len = highs.len();

	let mut median_price = vec![0.0; len];
	for i in 0..len {
		median_price[i] = (highs[i] + lows[i]) / 2.0;
	}

	let sma_fast = sma_internal(&median_price, fast_period);
	let sma_slow = sma_internal(&median_price, slow_period);

	let mut result = vec![f64::NAN; len];

	for i in 0..len {
		let fast_val = sma_fast[i];
		let slow_val = sma_slow[i];

		if fast_val.is_nan() || slow_val.is_nan() {
			result[i] = f64::NAN;
		} else {
			result[i] = fast_val - slow_val;
		}
	}

	Ok(result)
}
