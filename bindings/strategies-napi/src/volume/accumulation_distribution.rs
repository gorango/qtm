use napi_derive::napi;

use strategies_core::AccumulationDistributionConfig;

#[napi]
pub fn accumulation_distribution_strategy(
	highs: Vec<f64>,
	lows: Vec<f64>,
	closes: Vec<f64>,
	volumes: Vec<f64>,
	config: Option<AccumulationDistributionConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::accumulation_distribution_strategy(&highs, &lows, &closes, &volumes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}

pub fn accumulation_distribution_strategy_metadata() -> serde_json::Value {
	strategies_core::accumulation_distribution_strategy_metadata()
}

pub fn accumulation_distribution_strategy_defaults() -> serde_json::Value {
	strategies_core::accumulation_distribution_strategy_defaults()
}

pub fn accumulation_distribution(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config = config
		.map(|c| serde_json::from_value::<AccumulationDistributionConfig>(c).unwrap_or_default());
	strategies_core::accumulation_distribution_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		input.volumes.as_ref().ok_or(napi::Error::new(
			napi::Status::InvalidArg,
			"Volumes required",
		))?,
		config,
	)
	.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}
