use indicators_core::trend::typical_price::typical_price_internal;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Typical Price
#[napi]
pub fn typical_price(highs: Float64Array, lows: Float64Array, closes: Float64Array) -> Vec<f64> {
	typical_price_internal(highs.as_ref(), lows.as_ref(), closes.as_ref())
}
