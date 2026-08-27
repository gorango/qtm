use crate::internal::ema::ema_internal;
use crate::volume::accumulation_distribution;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChaikinOscillatorConfig {
	pub fast_period: Option<u32>,
	pub slow_period: Option<u32>,
}

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize)]
pub struct ChaikinOscillatorResult {
	pub ad_result: Vec<f64>,
	pub cmo_result: Vec<f64>,
}

/// Chaikin Oscillator — `EMA(A/D, fast) - EMA(A/D, slow)` on the Accumulation/Distribution line.
/// Measures volume-based momentum; positive = accumulation. Defined by Marc Chaikin.
///
/// # Errors
/// Returns an error if periods are 0 or inputs mismatched.
pub fn chaikin_oscillator(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	volumes: &[f64],
	config: Option<ChaikinOscillatorConfig>,
) -> ChaikinOscillatorResult {
	let config_obj = config.unwrap_or(ChaikinOscillatorConfig {
		fast_period: None,
		slow_period: None,
	});
	let fast_period = config_obj.fast_period.unwrap_or(3) as usize;
	let slow_period = config_obj.slow_period.unwrap_or(10) as usize;

	let ad_result = accumulation_distribution(highs, lows, closings, volumes);
	let ad_slice = ad_result.as_slice();

	let fast_ema = ema_internal(ad_slice, fast_period);
	let slow_ema = ema_internal(ad_slice, slow_period);

	let cmo_result: Vec<f64> = fast_ema
		.iter()
		.enumerate()
		.map(|(i, &fast)| {
			let slow_val = slow_ema.get(i).copied();
			if let Some(slow) = slow_val {
				if !slow.is_nan() {
					fast - slow
				} else {
					f64::NAN
				}
			} else {
				f64::NAN
			}
		})
		.collect();

	ChaikinOscillatorResult {
		ad_result,
		cmo_result,
	}
}

/// Alias `cmo` for Chaikin Oscillator (alternate short name).
pub fn cmo(
	highs: &[f64],
	lows: &[f64],
	closings: &[f64],
	volumes: &[f64],
	config: Option<ChaikinOscillatorConfig>,
) -> ChaikinOscillatorResult {
	chaikin_oscillator(highs, lows, closings, volumes, config)
}
