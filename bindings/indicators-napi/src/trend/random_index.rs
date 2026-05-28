use indicators_core::{random_index as ri_core, KDJResult};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Random Index
#[napi]
pub fn random_index(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	r_period: Option<u32>,
	k_period: Option<u32>,
	d_period: Option<u32>,
) -> Result<KDJResult> {
	ri_core(
		highs.as_ref(),
		lows.as_ref(),
		closings.as_ref(),
		r_period,
		k_period,
		d_period,
	)
	.map_err(|e| napi::Error::from_reason(e.to_string()))
}
