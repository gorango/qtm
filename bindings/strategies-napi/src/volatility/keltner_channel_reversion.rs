use napi_derive::napi;

use strategies_core::KeltnerChannelConfig;

#[napi]
pub fn keltner_channel_reversion_strategy(
	highs: Vec<f64>,
	lows: Vec<f64>,
	closes: Vec<f64>,
	config: Option<KeltnerChannelConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::keltner_channel_reversion_strategy(&highs, &lows, &closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}

pub fn keltner_channel_reversion_strategy_metadata() -> serde_json::Value {
	strategies_core::keltner_channel_reversion_strategy_metadata()
}

pub fn keltner_channel_reversion_strategy_defaults() -> serde_json::Value {
	strategies_core::keltner_channel_reversion_strategy_defaults()
}

pub fn keltner_channel_reversion(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<KeltnerChannelConfig>(c).unwrap_or_default());
	strategies_core::keltner_channel_reversion_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
	.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}
