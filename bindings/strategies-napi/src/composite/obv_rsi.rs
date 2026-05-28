use napi_derive::napi;

use strategies_core::RSIConfig;

#[napi]
pub fn obv_rsi_strategy(
	closes: Vec<f64>,
	volumes: Vec<f64>,
	rsi_config: Option<RSIConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::obv_rsi_strategy(&closes, &volumes, rsi_config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}

pub fn obv_rsi_strategy_metadata() -> serde_json::Value {
	strategies_core::obv_rsi_strategy_metadata()
}

pub fn obv_rsi_strategy_defaults() -> serde_json::Value {
	strategies_core::obv_rsi_strategy_defaults()
}

pub fn obv_rsi(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<RSIConfig>(c).unwrap_or_default());
	strategies_core::obv_rsi_strategy(
		&input.closes,
		input.volumes.as_ref().ok_or(napi::Error::new(
			napi::Status::InvalidArg,
			"Volumes required",
		))?,
		config,
	)
	.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}
