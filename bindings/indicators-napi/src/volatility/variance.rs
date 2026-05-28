use indicators_core::volatility::variance::{
	rolling_variance as rv_core, variance as var_core, VarianceConfig,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Variance
#[napi]
pub fn variance(values: Float64Array, config: Option<VarianceConfig>) -> Result<Vec<f64>> {
	var_core(values.as_ref(), config).map_err(napi::Error::from_reason)
}

/// Rolling Variance (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn rollingVariance(values: Float64Array, config: Option<VarianceConfig>) -> Result<Vec<f64>> {
	rv_core(values.as_ref(), config).map_err(napi::Error::from_reason)
}
