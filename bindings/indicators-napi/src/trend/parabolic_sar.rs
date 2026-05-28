use indicators_core::{parabolic_sar as psar_core, PSARConfig, PSARResult};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Parabolic Sar
#[napi]
pub fn parabolic_sar(
	highs: Float64Array,
	lows: Float64Array,
	closings: Float64Array,
	config: Option<PSARConfig>,
) -> Result<PSARResult> {
	psar_core(highs.as_ref(), lows.as_ref(), closings.as_ref(), config)
		.map_err(napi::Error::from_reason)
}
