use indicators_core::volatility::average_true_range::{
	atr as atr_core, average_true_range as atr_alias, ATRConfig, ATRResult,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Atr
#[napi]
pub fn atr(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	config: Option<ATRConfig>,
) -> Result<ATRResult> {
	atr_core(highs.as_ref(), lows.as_ref(), closings.as_ref(), config)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Average True Range (camelCase alias)
#[napi]
#[allow(non_snake_case)]
pub fn averageTrueRange(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	config: Option<ATRConfig>,
) -> Result<ATRResult> {
	atr_alias(highs.as_ref(), lows.as_ref(), closings.as_ref(), config)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}
