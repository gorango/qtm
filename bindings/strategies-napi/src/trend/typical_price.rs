use napi_derive::napi;

use strategies_core::TypicalPriceConfig;

#[napi]
pub fn typical_price_strategy(
	highs: Vec<f64>,
	lows: Vec<f64>,
	closes: Vec<f64>,
	config: Option<TypicalPriceConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::typical_price_strategy(&highs, &lows, &closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}

pub fn typical_price_strategy_metadata() -> serde_json::Value {
	strategies_core::typical_price_strategy_metadata()
}

pub fn typical_price_strategy_defaults() -> serde_json::Value {
	strategies_core::typical_price_strategy_defaults()
}

pub fn typical_price(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<TypicalPriceConfig>(c).unwrap_or_default());
	strategies_core::typical_price_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
	.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}
