use crate::internal::sma::sma_internal;
use crate::IndicatorResult;
use serde::{Deserialize, Serialize};

fn typical_price_for_cci(highs: &[f64], lows: &[f64], closings: &[f64]) -> Vec<f64> {
	highs
		.iter()
		.enumerate()
		.map(|(i, high)| (high + lows[i] + closings[i]) / 3.0)
		.collect()
}

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CCIConfig {
	pub period: Option<u32>,
}

pub fn cci(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	config: Option<CCIConfig>,
) -> IndicatorResult<Vec<f64>> {
	crate::utils::validation::validate_multiple_arrays(&[highs, lows, closings])?;

	let config = config.unwrap_or(CCIConfig { period: Some(20) });
	let period = config.period.unwrap_or(20) as usize;
	crate::utils::validation::validate_period(period)?;

	let tp = typical_price_for_cci(highs, lows, closings);
	let ma = sma_internal(&tp, period);

	let result: Vec<f64> = tp
		.iter()
		.enumerate()
		.map(|(i, price)| {
			if i < period - 1 {
				return f64::NAN;
			}

			let current_sma = ma[i];

			if current_sma.is_nan() {
				return f64::NAN;
			}

			let mut sum_deviations = 0.0;
			for j in 0..period {
				let idx = i - j;
				sum_deviations += (tp[idx] - current_sma).abs();
			}

			let mean_deviation = sum_deviations / period as f64;

			if mean_deviation == 0.0 {
				return 0.0;
			}

			(*price - current_sma) / (0.015 * mean_deviation)
		})
		.collect();

	Ok(result)
}
