use indicators_core::market::advance_decline::advance_decline_line as adl_core;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Advance Decline Line
#[napi]
pub fn advance_decline_line(advances: Float64Array, declines: Float64Array) -> Vec<f64> {
	adl_core(advances.as_ref(), declines.as_ref())
}
