use napi_derive::napi;

use strategies_core::AlmahmaDivergenceConfig;

#[napi]
pub fn alma_hma_divergence_strategy(
	closes: Vec<f64>,
	config: Option<AlmahmaDivergenceConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::alma_hma_divergence_strategy(&closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}

pub fn alma_hma_divergence_strategy_metadata() -> serde_json::Value {
	strategies_core::alma_hma_divergence_strategy_metadata()
}

pub fn alma_hma_divergence_strategy_defaults() -> serde_json::Value {
	strategies_core::alma_hma_divergence_strategy_defaults()
}

pub fn alma_hma_divergence(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<AlmahmaDivergenceConfig>(c).unwrap_or_default());
	strategies_core::alma_hma_divergence_strategy(&input.closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}
