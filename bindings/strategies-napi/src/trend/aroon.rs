use napi_derive::napi;

use strategies_core::AroonConfig;

#[napi]
pub fn aroon_strategy(
	highs: Vec<f64>,
	lows: Vec<f64>,
	config: Option<AroonConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::aroon_strategy(&highs, &lows, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}

pub fn aroon_strategy_metadata() -> serde_json::Value {
	strategies_core::aroon_strategy_metadata()
}

pub fn aroon_strategy_defaults() -> serde_json::Value {
	strategies_core::aroon_strategy_defaults()
}

pub fn aroon(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config = config.map(|c| serde_json::from_value::<AroonConfig>(c).unwrap_or_default());
	strategies_core::aroon_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		config,
	)
	.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}
