use crate::internal::sma::sma_internal;
use crate::utils::validation::validate_period;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VolumeSurgeConfig {
	pub period: Option<u32>,
	pub multiplier: Option<f64>,
}

pub fn volume_surge(volumes: &[f64], config: Option<VolumeSurgeConfig>) -> Vec<bool> {
	if volumes.is_empty() {
		return vec![];
	}

	let len = volumes.len();

	let cfg = config.unwrap_or(VolumeSurgeConfig {
		period: Some(20),
		multiplier: Some(2.0),
	});

	let period = cfg.period.unwrap_or(20) as usize;
	let multiplier = cfg.multiplier.unwrap_or(2.0);

	if period > 0 && validate_period(period).is_err() {
		return vec![];
	}

	if len < period {
		return vec![];
	}

	let volume_ma = sma_internal(volumes, period);
	let mut result = vec![false; len];

	for i in 0..len {
		let ma = volume_ma[i];
		if !ma.is_nan() && volumes[i] > ma * multiplier {
			result[i] = true;
		}
	}

	result
}

pub fn vs(volumes: &[f64], config: Option<VolumeSurgeConfig>) -> Vec<bool> {
	volume_surge(volumes, config)
}
