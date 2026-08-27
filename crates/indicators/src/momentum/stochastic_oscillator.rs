use crate::internal::sma::sma_internal;
use crate::utils::rolling::rolling_max;
use crate::utils::rolling::rolling_min;
use crate::utils::validation::validate_period;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StochConfig {
	pub k_period: Option<u32>,
	pub d_period: Option<u32>,
}

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct StochResult {
	pub k: Vec<f64>,
	pub d: Vec<f64>,
}

/// Stochastic Oscillator — `%K = 100*(close - lowest_low)/(highest_high - lowest_low)`; `%D = SMA(%K, d_period)`.
/// Bounded 0..100; >80 overbought. Defined by George Lane. `NaN` until warmup.
///
/// # Errors
/// Returns an error if inputs invalid.
pub fn stochastic_oscillator(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<StochConfig>,
) -> StochResult {
	let config_obj = config.unwrap_or(StochConfig {
		k_period: None,
		d_period: None,
	});
	let k_period = config_obj.k_period.unwrap_or(14) as usize;
	let k_period = if k_period == 0 { 14 } else { k_period };
	let d_period = config_obj.d_period.unwrap_or(3) as usize;
	let d_period = if d_period == 0 { 3 } else { d_period };

	let _ = validate_period(k_period);

	if highs.len() != lows.len() || highs.len() != closes.len() {
		return StochResult {
			k: vec![],
			d: vec![],
		};
	}

	let len = highs.len();
	let highest_highs = rolling_max(highs, k_period);
	let lowest_lows = rolling_min(lows, k_period);

	let mut k_value = vec![f64::NAN; len];

	for i in 0..len {
		let hh = highest_highs[i];
		let ll = lowest_lows[i];
		let denominator = hh - ll;

		if !hh.is_nan() && !ll.is_nan() && denominator > 1e-10 {
			k_value[i] = ((closes[i] - ll) / denominator) * 100.0;
		}
	}

	let valid_k_start_index = k_period - 1;

	let mut d_value = vec![f64::NAN; len];

	if len > valid_k_start_index {
		let valid_k_slice = &k_value[valid_k_start_index..];

		let valid_d_sma = sma_internal(valid_k_slice, d_period);

		for (i, val) in valid_d_sma.into_iter().enumerate() {
			let target_idx = valid_k_start_index + i;
			if target_idx < len {
				d_value[target_idx] = val;
			}
		}
	}

	StochResult {
		k: k_value,
		d: d_value,
	}
}
