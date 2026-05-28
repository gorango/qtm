use indicators_core::mass_index as mi_core;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Mass Index
#[napi]
pub fn mass_index(
	highs: Float64Array,
	lows: Float64Array,
	ema_period: Option<u32>,
	mi_period: Option<u32>,
) -> Result<Vec<f64>> {
	mi_core(highs.as_ref(), lows.as_ref(), ema_period, mi_period).map_err(napi::Error::from_reason)
}
