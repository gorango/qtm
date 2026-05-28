use indicators_core::{volume_weighted_average_price as vwap_alias, vwap as vwap_core, VWAPConfig};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Vwap
#[napi]
pub fn vwap(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	volumes: Float64Array,
	config: Option<VWAPConfig>,
) -> Vec<f64> {
	vwap_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		volumes.as_ref(),
		config,
	)
}

/// Volume Weighted Average Price
#[napi]
pub fn volume_weighted_average_price(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	volumes: Float64Array,
	config: Option<VWAPConfig>,
) -> Vec<f64> {
	vwap_alias(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		volumes.as_ref(),
		config,
	)
}
