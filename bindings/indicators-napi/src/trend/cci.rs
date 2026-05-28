use indicators_core::{cci as cci_core, CCIConfig};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Cci
#[napi]
pub fn cci(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	config: Option<CCIConfig>,
) -> Result<Vec<f64>> {
	cci_core(highs.as_ref(), lows.as_ref(), closings.as_ref(), config)
		.map_err(|e| napi::Error::from_reason(e.to_string()))
}
