use indicators_core::annualized_volatility as av_core;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Annualized Volatility
#[napi]
pub fn annualized_volatility(returns: Float64Array, periods: Option<u32>) -> Result<Vec<f64>> {
	av_core(returns.as_ref(), periods).map_err(|e| napi::Error::from_reason(e.to_string()))
}
