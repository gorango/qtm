use indicators_core::{kst as kst_core, KSTConfig, KSTResult};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Kst
#[napi]
pub fn kst(prices: Float64Array, config: Option<KSTConfig>) -> KSTResult {
	kst_core(prices.as_ref(), config)
}
