use crate::internal::sma::sma_internal;
use crate::internal::true_range::tr_internal;
use crate::utils::validation::validate_multiple_arrays;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UltimateOscillatorConfig {
	pub period1: Option<u32>,
	pub period2: Option<u32>,
	pub period3: Option<u32>,
}

pub fn ultimate_oscillator(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	config: Option<UltimateOscillatorConfig>,
) -> Vec<f64> {
	if validate_multiple_arrays(&[highs, lows, closings]).is_err() {
		return vec![];
	}

	let config_obj = config.unwrap_or(UltimateOscillatorConfig {
		period1: None,
		period2: None,
		period3: None,
	});
	let period1 = config_obj.period1.unwrap_or(7) as usize;
	let period2 = config_obj.period2.unwrap_or(14) as usize;
	let period3 = config_obj.period3.unwrap_or(28) as usize;

	let len = highs.len();

	let mut result = vec![f64::NAN; len];

	let mut true_low = vec![0.0; len];
	let mut bp = vec![0.0; len];

	for i in 0..len {
		let prev_close = if i == 0 { closings[i] } else { closings[i - 1] };
		true_low[i] = lows[i].min(prev_close);
		bp[i] = closings[i] - true_low[i];
	}

	let true_range = tr_internal(highs, lows, closings);

	let avg_bp1 = sma_internal(&bp, period1);
	let avg_tr1 = sma_internal(&true_range, period1);
	let avg_bp2 = sma_internal(&bp, period2);
	let avg_tr2 = sma_internal(&true_range, period2);
	let avg_bp3 = sma_internal(&bp, period3);
	let avg_tr3 = sma_internal(&true_range, period3);

	for i in 0..len {
		let ratio1 = if avg_tr1[i] != 0.0 {
			avg_bp1[i] / avg_tr1[i]
		} else {
			0.0
		};
		let ratio2 = if avg_tr2[i] != 0.0 {
			avg_bp2[i] / avg_tr2[i]
		} else {
			0.0
		};
		let ratio3 = if avg_tr3[i] != 0.0 {
			avg_bp3[i] / avg_tr3[i]
		} else {
			0.0
		};

		result[i] = (100.0 * (4.0 * ratio1 + 2.0 * ratio2 + ratio3)) / 7.0;
	}

	result
}

pub fn uo(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	config: Option<UltimateOscillatorConfig>,
) -> Vec<f64> {
	ultimate_oscillator(highs, lows, closings, config)
}
