use indicators_core::{value_when as value_when_core, ValueWhenConfig};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Value When
#[napi]
pub fn value_when(
	condition: Float64Array,
	source: Float64Array,
	config: Option<ValueWhenConfig>,
) -> Result<Vec<f64>> {
	value_when_core(condition.as_ref(), source.as_ref(), config).map_err(napi::Error::from_reason)
}
