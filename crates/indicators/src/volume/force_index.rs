use crate::internal::ema::ema_internal;
use crate::utils::arrays::validate_arrays_equal_length;
use crate::utils::validation::validate_period;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct FIConfig {
	pub period: Option<u32>,
}

pub fn force_index(closings: &[f64], volumes: &[f64], config: Option<FIConfig>) -> Vec<f64> {
	validate_arrays_equal_length(&[closings, volumes]).unwrap();

	let len = closings.len();

	let cfg = config.unwrap_or(FIConfig { period: Some(13) });
	let period = cfg.period.unwrap_or(13) as usize;

	if period > 0 {
		validate_period(period).unwrap();
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

pub fn fi(closings: &[f64], volumes: &[f64], config: Option<FIConfig>) -> Vec<f64> {
	force_index(closings, volumes, config)
}
