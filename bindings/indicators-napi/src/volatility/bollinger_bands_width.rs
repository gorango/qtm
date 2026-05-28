use indicators_core::volatility::bollinger_bands::BBResult;
use indicators_core::volatility::bollinger_bands_width::{
	bbw as bbw_core, bollinger_bands_width as bbw_alias, BBWResult,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Bbw
#[napi]
pub fn bbw(bb: BBResult, period: Option<u32>) -> Result<BBWResult> {
	bbw_core(bb, period).map_err(napi::Error::from_reason)
}

/// Bollinger Bands Width (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn bollingerBandsWidth(bb: BBResult, period: Option<u32>) -> Result<BBWResult> {
	bbw_alias(bb, period).map_err(napi::Error::from_reason)
}
