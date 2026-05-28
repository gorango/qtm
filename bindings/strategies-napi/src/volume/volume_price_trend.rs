use napi_derive::napi;

use strategies_core::VolumePriceTrendConfig;

#[napi]
pub fn volume_price_trend_strategy(
	closes: Vec<f64>,
	volumes: Vec<f64>,
	config: Option<VolumePriceTrendConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::volume_price_trend_strategy(&closes, &volumes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}

pub fn volume_price_trend_strategy_metadata() -> serde_json::Value {
	strategies_core::volume_price_trend_strategy_metadata()
}

pub fn volume_price_trend_strategy_defaults() -> serde_json::Value {
	strategies_core::volume_price_trend_strategy_defaults()
}

pub fn volume_price_trend(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<VolumePriceTrendConfig>(c).unwrap_or_default());
	strategies_core::volume_price_trend_strategy(
		&input.closes,
		input.volumes.as_ref().ok_or(napi::Error::new(
			napi::Status::InvalidArg,
			"Volumes required",
		))?,
		config,
	)
	.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}
