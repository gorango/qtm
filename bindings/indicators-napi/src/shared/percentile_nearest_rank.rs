use indicators_core::{
	percentile_nearest_rank as percentile_nearest_rank_core, PercentileNearestRankConfig,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Percentile Nearest Rank
#[napi]
pub fn percentile_nearest_rank(
	values: Float64Array,
	config: Option<PercentileNearestRankConfig>,
) -> Result<Vec<f64>> {
	percentile_nearest_rank_core(values.as_ref(), config).map_err(napi::Error::from_reason)
}
