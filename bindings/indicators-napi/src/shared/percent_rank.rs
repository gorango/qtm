use indicators_core::{percent_rank as percent_rank_core, PercentRankConfig};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Percent Rank
#[napi]
pub fn percent_rank(values: Float64Array, config: Option<PercentRankConfig>) -> Result<Vec<f64>> {
	percent_rank_core(values.as_ref(), config).map_err(napi::Error::from_reason)
}
