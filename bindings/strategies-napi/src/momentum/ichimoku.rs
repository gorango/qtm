use napi_derive::napi;

use strategies_core::IchimokuCloudConfig;

#[napi]
pub fn ichimoku_strategy(
	closes: Vec<f64>,
	highs: Vec<f64>,
	lows: Vec<f64>,
	config: Option<IchimokuCloudConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::ichimoku_strategy(&closes, &highs, &lows, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}

pub fn ichimoku_strategy_metadata() -> serde_json::Value {
	strategies_core::ichimoku_strategy_metadata()
}

pub fn ichimoku_strategy_defaults() -> serde_json::Value {
	strategies_core::ichimoku_strategy_defaults()
}

pub fn ichimoku(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<IchimokuCloudConfig>(c).unwrap_or_default());
	strategies_core::ichimoku_strategy(
		&input.closes,
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		config,
	)
	.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}
