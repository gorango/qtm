use indicators_core::{momentum_index as mi_core, MomentumIndexConfig};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Momentum Index
#[napi]
pub fn momentum_index(prices: Float64Array, config: Option<MomentumIndexConfig>) -> Vec<f64> {
	mi_core(prices.as_ref(), config)
}
