use indicators_core::volatility::ttm_squeeze::{
	ttm_squeeze as tts_alias, ttm_squeeze as tts_core, TTMSqueezeResult,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Ttm Squeeze
#[napi]
pub fn ttm_squeeze(
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
	bb_period: Option<u32>,
	bb_std_dev: Option<f64>,
	kc_period: Option<u32>,
) -> Result<TTMSqueezeResult> {
	tts_core(
		highs.as_ref(),
		lows.as_ref(),
		closes.as_ref(),
		bb_period,
		bb_std_dev,
		kc_period,
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Ttm Squeeze (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn ttmSqueeze(
	highs: Float64Array,
	lows: Float64Array,
	closes: Float64Array,
	bb_period: Option<u32>,
	bb_std_dev: Option<f64>,
	kc_period: Option<u32>,
) -> Result<TTMSqueezeResult> {
	tts_alias(
		highs.as_ref(),
		lows.as_ref(),
		closes.as_ref(),
		bb_period,
		bb_std_dev,
		kc_period,
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}
