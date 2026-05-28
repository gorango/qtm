use napi_derive::napi;

use strategies_core::VwmaConfig;

#[napi]
pub fn vwma_strategy(
	closes: Vec<f64>,
	volumes: Vec<f64>,
	config: Option<VwmaConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::vwma_strategy(&closes, &volumes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}

pub fn vwma_strategy_metadata() -> serde_json::Value {
	strategies_core::vwma_strategy_metadata()
}

pub fn vwma_strategy_defaults() -> serde_json::Value {
	strategies_core::vwma_strategy_defaults()
}

pub fn vwma(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<VwmaConfig>(c).unwrap_or_default());
	strategies_core::vwma_strategy(
		&input.closes,
		input.volumes.as_ref().ok_or(napi::Error::new(
			napi::Status::InvalidArg,
			"Volumes required",
		))?,
		config,
	)
	.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}
