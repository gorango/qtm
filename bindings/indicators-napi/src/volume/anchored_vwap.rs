use indicators_core::volume::anchored_vwap::anchored_vwap as avwap_core;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Anchored Vwap
#[napi]
pub fn anchored_vwap(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	volumes: Float64Array,
	anchor_index: Option<u32>,
) -> Vec<f64> {
	let anchor = anchor_index.unwrap_or(0);
	avwap_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		volumes.as_ref(),
		anchor,
	)
}
