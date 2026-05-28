use indicators_core::internal::sma::sma_internal;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Sma
#[napi]
pub fn sma(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	let period = period.unwrap_or(2) as usize;
	Ok(sma_internal(values.as_ref(), period))
}
