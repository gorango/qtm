use crate::internal::ema::ema_internal;
use crate::utils::arrays::validate_arrays_equal_length;
use crate::utils::validation::validate_period;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
/// Force Index config.
pub struct FIConfig {
	/// EMA period for Force Index (default 13). Valid 2..=100.
	pub period: Option<u32>,
}

/// Force Index — `(close - prev_close) * volume` smoothed with EMA(period).
/// Combines price and volume to measure buying/selling pressure. Defined by Alexander Elder.
pub fn force_index(closings: &[f64], volumes: &[f64], config: Option<FIConfig>) -> Vec<f64> {
	if validate_arrays_equal_length(&[closings, volumes]).is_err() {
		return vec![];
	}

	let len = closings.len();

	let cfg = config.unwrap_or(FIConfig { period: Some(13) });
	let period = cfg.period.unwrap_or(13) as usize;

	if period > 0 && validate_period(period).is_err() {
		return vec![];
	}

	let mut changes = Vec::with_capacity(len);
	let mut force = Vec::with_capacity(len);

	for i in 0..len {
		if i > 0 {
			changes.push(closings[i] - closings[i - 1]);
		} else {
			changes.push(closings[i]);
		}
		force.push(changes[i] * volumes[i]);
	}

	ema_internal(&force, period)
}

/// Alias `fi` for Force Index.
pub fn fi(closings: &[f64], volumes: &[f64], config: Option<FIConfig>) -> Vec<f64> {
	force_index(closings, volumes, config)
}
