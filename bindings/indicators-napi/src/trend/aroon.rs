use indicators_core::{aroon as aroon_core, AroonConfig, AroonResult};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Aroon
#[napi]
pub fn aroon(
	highs: Float64Array,
	lows: Float64Array,
	config: Option<AroonConfig>,
) -> Result<AroonResult> {
	aroon_core(highs.as_ref(), lows.as_ref(), config).map_err(napi::Error::from_reason)
}
