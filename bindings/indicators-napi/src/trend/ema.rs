use indicators_core::internal::ema::ema_internal;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Ema
#[napi]
pub fn ema(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	let period = period.unwrap_or(12) as usize;
	Ok(ema_internal(values.as_ref(), period))
}
