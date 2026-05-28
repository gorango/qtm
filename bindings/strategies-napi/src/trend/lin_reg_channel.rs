use napi_derive::napi;

use strategies_core::LinRegChannelConfig;

#[napi]
pub fn lin_reg_channel_strategy(
	closes: Vec<f64>,
	config: Option<LinRegChannelConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::lin_reg_channel_strategy(&closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}

pub fn lin_reg_channel_strategy_metadata() -> serde_json::Value {
	strategies_core::lin_reg_channel_strategy_metadata()
}

pub fn lin_reg_channel_strategy_defaults() -> serde_json::Value {
	strategies_core::lin_reg_channel_strategy_defaults()
}

pub fn lin_reg_channel(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<LinRegChannelConfig>(c).unwrap_or_default());
	strategies_core::lin_reg_channel_strategy(&input.closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}
