use napi_derive::napi;

use strategies_core::VwapBreakoutConfig;

#[napi]
pub fn vwap_breakout_strategy(
	closes: Vec<f64>,
	highs: Vec<f64>,
	lows: Vec<f64>,
	volumes: Vec<f64>,
	config: Option<VwapBreakoutConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::vwap_breakout_strategy(&closes, &highs, &lows, &volumes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}

pub fn vwap_breakout_strategy_metadata() -> serde_json::Value {
	strategies_core::vwap_breakout_strategy_metadata()
}

pub fn vwap_breakout_strategy_defaults() -> serde_json::Value {
	strategies_core::vwap_breakout_strategy_defaults()
}

pub fn vwap_breakout(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<VwapBreakoutConfig>(c).unwrap_or_default());
	strategies_core::vwap_breakout_strategy(
		&input.closes,
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		input.volumes.as_ref().ok_or(napi::Error::new(
			napi::Status::InvalidArg,
			"Volumes required",
		))?,
		config,
	)
	.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}
