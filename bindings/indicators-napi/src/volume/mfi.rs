use indicators_core::{mfi as mfi_core, money_flow_index as mfi_alias, MFIConfig};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Mfi
#[napi]
pub fn mfi(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	volumes: Float64Array,
	config: Option<MFIConfig>,
) -> Vec<f64> {
	mfi_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		volumes.as_ref(),
		config,
	)
}

/// Money Flow Index
#[napi]
pub fn money_flow_index(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	volumes: Float64Array,
	config: Option<MFIConfig>,
) -> Vec<f64> {
	mfi_alias(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		volumes.as_ref(),
		config,
	)
}
