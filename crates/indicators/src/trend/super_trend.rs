use crate::internal::true_range::tr_internal;
use crate::trend::rma::rma_internal;
use crate::IndicatorResult;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct SuperTrendResult {
	pub super_trend: Vec<f64>,
	pub direction: Vec<i32>,
}

pub fn super_trend(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	period: Option<u32>,
	multiplier: Option<f64>,
) -> IndicatorResult<SuperTrendResult> {
	crate::utils::validation::validate_multiple_arrays(&[highs, lows, closes])?;

	let period = period.unwrap_or(14) as usize;
	let multiplier = multiplier.unwrap_or(3.0);

	let len = closes.len();

	if len < period {
		return Ok(SuperTrendResult {
			super_trend: vec![0.0; len],
			direction: vec![0; len],
		});
	}

	let tr_line = tr_internal(highs, lows, closes);
	let atr_line = rma_internal(&tr_line, period);

	let mut super_trend_values = vec![0.0; len];
	let mut direction = vec![0; len];

	for i in 0..len {
		if i < period - 1 {
			super_trend_values[i] = 0.0;
			direction[i] = 0;
			continue;
		}

		let hl2 = (highs[i] + lows[i]) / 2.0;
		let basic_upper_band = hl2 + multiplier * atr_line[i];
		let basic_lower_band = hl2 - multiplier * atr_line[i];

		let (final_upper_band, final_lower_band) = if i == period - 1 {
			(basic_upper_band, basic_lower_band)
		} else {
			let fu = if closes[i - 1] > super_trend_values[i - 1] {
				basic_upper_band.min(super_trend_values[i - 1])
			} else {
				basic_upper_band
			};

			let fl = if closes[i - 1] < super_trend_values[i - 1] {
				basic_lower_band.max(super_trend_values[i - 1])
			} else {
				basic_lower_band
			};

			(fu, fl)
		};

		let trend = if closes[i] > final_upper_band {
			1
		} else if closes[i] < final_lower_band {
			-1
		} else if direction[i - 1] != 0 {
			direction[i - 1]
		} else {
			1
		};

		let super_trend_value = if trend == 1 {
			final_lower_band
		} else {
			final_upper_band
		};

		super_trend_values[i] = super_trend_value;
		direction[i] = trend;
	}

	Ok(SuperTrendResult {
		super_trend: super_trend_values,
		direction,
	})
}
