use crate::validation::validate_arrays;
use indicators_core::market::advance_decline::advance_decline_line as adl_core;
use indicators_core::market::mcclellan_oscillator::mcclellan_oscillator as mo_core;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Advance Decline Line
#[napi]
pub fn advance_decline_line(advances: Float64Array, declines: Float64Array) -> Result<Vec<f64>> {
	validate_arrays(
		&[advances.as_ref(), declines.as_ref()],
		&["advances", "declines"],
	)?;
	Ok(adl_core(advances.as_ref(), declines.as_ref()))
}

/// McClellan Oscillator
#[napi]
pub fn mcclellan_oscillator(advances: Float64Array, declines: Float64Array) -> Result<Vec<f64>> {
	mo_core(advances.as_ref(), declines.as_ref())
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}
