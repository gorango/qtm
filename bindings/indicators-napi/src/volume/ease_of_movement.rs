use indicators_core::volume::ease_of_movement::{ease_of_movement as eom_core, emv as emv_core};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Ease Of Movement
#[napi]
pub fn ease_of_movement(
	highs: Float64Array,
	lows: Float64Array,
	volumes: Float64Array,
	period: Option<u32>,
) -> Vec<f64> {
	let period = period.unwrap_or(14);
	eom_core(highs.as_ref(), lows.as_ref(), volumes.as_ref(), period)
}

/// Emv
#[napi]
pub fn emv(
	highs: Float64Array,
	lows: Float64Array,
	volumes: Float64Array,
	period: Option<u32>,
) -> Vec<f64> {
	let period = period.unwrap_or(14);
	emv_core(highs.as_ref(), lows.as_ref(), volumes.as_ref(), period)
}
