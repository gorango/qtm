use napi_derive::napi;

use strategies_core::VolumeProfileRsiConfig;

#[napi]
pub fn volume_profile_rsi_strategy(
	highs: Vec<f64>,
	lows: Vec<f64>,
	closes: Vec<f64>,
	volumes: Vec<f64>,
	config: Option<VolumeProfileRsiConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::volume_profile_rsi_strategy(&highs, &lows, &closes, &volumes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}

pub fn volume_profile_rsi_strategy_metadata() -> serde_json::Value {
	strategies_core::volume_profile_rsi_strategy_metadata()
}

pub fn volume_profile_rsi_strategy_defaults() -> serde_json::Value {
	strategies_core::volume_profile_rsi_strategy_defaults()
}

pub fn volume_profile_rsi(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<VolumeProfileRsiConfig>(c).unwrap_or_default());
	strategies_core::volume_profile_rsi_strategy(
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
