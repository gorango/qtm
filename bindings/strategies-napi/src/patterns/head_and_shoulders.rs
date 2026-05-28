use napi_derive::napi;

use strategies_core::HeadAndShouldersConfig;

#[napi]
pub fn head_and_shoulders_strategy(
	opens: Vec<f64>,
	highs: Vec<f64>,
	lows: Vec<f64>,
	closes: Vec<f64>,
	config: Option<HeadAndShouldersConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::head_and_shoulders_strategy(&opens, &highs, &lows, &closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}

pub fn head_and_shoulders_strategy_metadata() -> serde_json::Value {
	strategies_core::head_and_shoulders_strategy_metadata()
}

pub fn head_and_shoulders_strategy_defaults() -> serde_json::Value {
	strategies_core::head_and_shoulders_strategy_defaults()
}

pub fn head_and_shoulders(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<HeadAndShouldersConfig>(c).unwrap_or_default());
	strategies_core::head_and_shoulders_strategy(
		input.opens.as_ref().unwrap_or(&input.closes),
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
	.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}
