use indicators_core::{macd as macd_core, MACDConfig, MACDResult};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Macd
#[napi]
pub fn macd(closes: Float64Array, config: Option<MACDConfig>) -> Result<MACDResult> {
	macd_core(closes.as_ref(), config).map_err(|e| napi::Error::from_reason(e.to_string()))
}
