use indicators_core::{
	ichimoku as ichimoku_core, ichimoku_cloud as ic_core, IchimokuCloudConfig, IchimokuCloudResult,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Ichimoku Cloud
#[napi]
pub fn ichimoku_cloud(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	config: Option<IchimokuCloudConfig>,
) -> IchimokuCloudResult {
	ic_core(highs.as_ref(), lows.as_ref(), closings.as_ref(), config)
}

/// Ichimoku
#[napi]
pub fn ichimoku(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	config: Option<IchimokuCloudConfig>,
) -> IchimokuCloudResult {
	ichimoku_core(highs.as_ref(), lows.as_ref(), closings.as_ref(), config)
}
