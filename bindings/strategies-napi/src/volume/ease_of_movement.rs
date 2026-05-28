use napi_derive::napi;

use strategies_core::EaseOfMovementConfig;

#[napi]
pub fn ease_of_movement_strategy(
	highs: Vec<f64>,
	lows: Vec<f64>,
	volumes: Vec<f64>,
	config: Option<EaseOfMovementConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::ease_of_movement_strategy(&highs, &lows, &volumes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}

pub fn ease_of_movement_strategy_metadata() -> serde_json::Value {
	strategies_core::ease_of_movement_strategy_metadata()
}

pub fn ease_of_movement_strategy_defaults() -> serde_json::Value {
	strategies_core::ease_of_movement_strategy_defaults()
}

pub fn ease_of_movement(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<EaseOfMovementConfig>(c).unwrap_or_default());
	strategies_core::ease_of_movement_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		input.volumes.as_ref().ok_or(napi::Error::new(
			napi::Status::InvalidArg,
			"Volumes required",
		))?,
		config,
	)
	.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}
