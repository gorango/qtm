use indicators_core::volatility::keltner_channel::{
	kc as kc_core, keltner_channel as kc_alias, KCResult,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Kc
#[napi]
pub fn kc(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	period: Option<u32>,
) -> Result<KCResult> {
	kc_core(highs.as_ref(), lows.as_ref(), closings.as_ref(), period)
		.map_err(napi::Error::from_reason)
}

/// Keltner Channel (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn keltnerChannel(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	period: Option<u32>,
) -> Result<KCResult> {
	kc_alias(highs.as_ref(), lows.as_ref(), closings.as_ref(), period)
		.map_err(napi::Error::from_reason)
}
