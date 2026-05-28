use indicators_core::{williams_r as wr_core, WilliamsRConfig};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Williams %R
#[napi]
pub fn williams_r(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	config: Option<WilliamsRConfig>,
) -> Result<Vec<f64>> {
	wr_core(highs.as_ref(), lows.as_ref(), closings.as_ref(), config)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}
