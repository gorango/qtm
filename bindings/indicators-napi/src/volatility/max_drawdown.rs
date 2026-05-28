use indicators_core::max_drawdown as md_core;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Max Drawdown
#[napi]
pub fn max_drawdown(values: Float64Array, period: u32) -> Result<Vec<f64>> {
	md_core(values.as_ref(), period).map_err(napi::Error::from_reason)
}
