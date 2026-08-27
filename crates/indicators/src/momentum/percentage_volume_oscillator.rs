use crate::internal::ema::ema_internal;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PercentageVolumeOscillatorConfig {
	pub fast_period: Option<u32>,
	pub slow_period: Option<u32>,
	pub signal_period: Option<u32>,
}

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct PercentageVolumeOscillatorResult {
	pub pvo_result: Vec<f64>,
	pub signal_period: Vec<f64>,
	pub histogram: Vec<f64>,
}

/// Percentage Volume Oscillator — `100*(EMA12(vol) - EMA26(vol))/EMA26(vol)` with signal.
/// Volume analog of PPO. Period defaults 12/26/9.
///
/// # Errors
/// Returns an error if periods are 0 or inputs invalid.
pub fn percentage_volume_oscillator(
	volumes: &[f64],
	config: Option<PercentageVolumeOscillatorConfig>,
) -> PercentageVolumeOscillatorResult {
	let config_obj = config.unwrap_or(PercentageVolumeOscillatorConfig {
		fast_period: None,
		slow_period: None,
		signal_period: None,
	});
	let fast_period = config_obj.fast_period.unwrap_or(12) as usize;
	let slow_period = config_obj.slow_period.unwrap_or(26) as usize;
	let signal_period = config_obj.signal_period.unwrap_or(9) as usize;

	let fast_ema = ema_internal(volumes, fast_period);
	let slow_ema = ema_internal(volumes, slow_period);

	let pvo_result: Vec<f64> = fast_ema
		.iter()
		.enumerate()
		.map(|(i, &fast)| {
			let slow = slow_ema.get(i).copied().unwrap_or(0.0);
			if slow != 0.0 {
				((fast - slow) / slow) * 100.0
			} else {
				0.0
			}
		})
		.collect();

	let signal = ema_internal(&pvo_result, signal_period);
	let histogram: Vec<f64> = pvo_result
		.iter()
		.enumerate()
		.map(|(i, &pvo)| pvo - signal.get(i).copied().unwrap_or(0.0))
		.collect();

	PercentageVolumeOscillatorResult {
		pvo_result,
		signal_period: signal,
		histogram,
	}
}

/// Alias `pvo` for Percentage Volume Oscillator.
pub fn pvo(
	volumes: &[f64],
	config: Option<PercentageVolumeOscillatorConfig>,
) -> PercentageVolumeOscillatorResult {
	percentage_volume_oscillator(volumes, config)
}
