use indicators_core::volatility::bollinger_bands::{
	bb as bb_core, bollinger_bands as bbands_core, BBConfig, BBResult,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Bollinger Bands
#[napi]
pub fn bb(closings: Float64Array, config: Option<BBConfig>) -> Result<BBResult> {
	bb_core(closings.as_ref(), config).map_err(napi::Error::from_reason)
}

/// Bollinger Bands (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn bollingerBands(closings: Float64Array, config: Option<BBConfig>) -> Result<BBResult> {
	bbands_core(closings.as_ref(), config).map_err(napi::Error::from_reason)
}
