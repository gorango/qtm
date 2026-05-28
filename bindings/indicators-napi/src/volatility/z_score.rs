use indicators_core::volatility::z_score::{z_score as zscore_alias, zs as zs_core, ZScoreConfig};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Zs
#[napi]
pub fn zs(values: Float64Array, config: Option<ZScoreConfig>) -> Result<Vec<f64>> {
	zs_core(values.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Z Score (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn zScore(values: Float64Array, config: Option<ZScoreConfig>) -> Result<Vec<f64>> {
	zscore_alias(values.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}
