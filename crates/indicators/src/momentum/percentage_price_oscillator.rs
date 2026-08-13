use crate::internal::ema::ema_internal;
use crate::utils::validation;
use crate::IndicatorResult;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PercentagePriceOscillatorConfig {
	pub fast_period: Option<u32>,
	pub slow_period: Option<u32>,
	pub signal_period: Option<u32>,
}

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct PercentagePriceOscillatorResult {
	pub ppo_result: Vec<f64>,
	pub signal: Vec<f64>,
	pub histogram: Vec<f64>,
}

pub fn percentage_price_oscillator(
	prices: &[f64],
	config: Option<PercentagePriceOscillatorConfig>,
) -> IndicatorResult<PercentagePriceOscillatorResult> {
	let config_obj = config.unwrap_or(PercentagePriceOscillatorConfig {
		fast_period: None,
		slow_period: None,
		signal_period: None,
	});

	let fast_period = config_obj.fast_period.unwrap_or(12) as usize;
	let slow_period = config_obj.slow_period.unwrap_or(26) as usize;
	let signal_period = config_obj.signal_period.unwrap_or(9) as usize;

	validation::validate_period(fast_period)?;
	validation::validate_period(slow_period)?;
	validation::validate_period(signal_period)?;

	let len = prices.len();

	let fast_ema = ema_internal(prices, fast_period);
	let slow_ema = ema_internal(prices, slow_period);

	let mut ppo_result = vec![f64::NAN; len];

	for i in 0..len {
		let fast = fast_ema[i];
		let slow = slow_ema[i];

		if fast.is_nan() || slow.is_nan() {
			continue;
		}

		if slow != 0.0 {
			ppo_result[i] = ((fast - slow) / slow) * 100.0;
		} else {
			ppo_result[i] = 0.0;
		}
	}

	let signal = ema_internal(&ppo_result, signal_period);

	let mut histogram = vec![f64::NAN; len];

	for i in 0..len {
		let ppo = ppo_result[i];
		let sig = signal[i];

		if ppo.is_nan() || sig.is_nan() {
			continue;
		}

		histogram[i] = ppo - sig;
	}

	Ok(PercentagePriceOscillatorResult {
		ppo_result,
		signal,
		histogram,
	})
}
