use indicators_core::trend::rma::rma_internal;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Rma
#[napi]
pub fn rma(values: Float64Array, period: Option<u32>) -> Result<Vec<f64>> {
	let period = period.unwrap_or(4) as usize;
	Ok(rma_internal(values.as_ref(), period))
}
