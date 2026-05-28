use napi_derive::napi;

use strategies_core::NegativeVolumeIndexConfig;

#[napi]
pub fn negative_volume_index_strategy(
	closes: Vec<f64>,
	volumes: Vec<f64>,
	config: Option<NegativeVolumeIndexConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::negative_volume_index_strategy(&closes, &volumes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}

pub fn negative_volume_index_strategy_metadata() -> serde_json::Value {
	strategies_core::negative_volume_index_strategy_metadata()
}

pub fn negative_volume_index_strategy_defaults() -> serde_json::Value {
	strategies_core::negative_volume_index_strategy_defaults()
}

pub fn negative_volume_index(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<NegativeVolumeIndexConfig>(c).unwrap_or_default());
	strategies_core::negative_volume_index_strategy(
		&input.closes,
		input.volumes.as_ref().ok_or(napi::Error::new(
			napi::Status::InvalidArg,
			"Volumes required",
		))?,
		config,
	)
	.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}
