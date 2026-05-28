use indicators_core::volume::accumulation_distribution::{
	accumulation_distribution as ad_core, ad as ad_alias,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Accumulation Distribution
#[napi]
pub fn accumulation_distribution(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	volumes: Float64Array,
) -> Vec<f64> {
	ad_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		volumes.as_ref(),
	)
}

/// Ad
#[napi]
pub fn ad(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	volumes: Float64Array,
) -> Vec<f64> {
	ad_alias(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		volumes.as_ref(),
	)
}
