use indicators_core::{larsson as larsson_core, LarssonResult};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Larsson
#[napi]
pub fn larsson(highs: Float64Array, lows: Float64Array) -> LarssonResult {
	larsson_core(highs.as_ref(), lows.as_ref())
}
