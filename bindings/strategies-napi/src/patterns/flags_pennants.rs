use napi_derive::napi;

use strategies_core::FlagsPennantsConfig;

#[napi]
pub fn flags_pennants_strategy(
	opens: Vec<f64>,
	highs: Vec<f64>,
	lows: Vec<f64>,
	closes: Vec<f64>,
	config: Option<FlagsPennantsConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::flags_pennants_strategy(&opens, &highs, &lows, &closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}

pub fn flags_pennants_strategy_metadata() -> serde_json::Value {
	strategies_core::flags_pennants_strategy_metadata()
}

pub fn flags_pennants_strategy_defaults() -> serde_json::Value {
	strategies_core::flags_pennants_strategy_defaults()
}

pub fn flags_pennants(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<FlagsPennantsConfig>(c).unwrap_or_default());
	strategies_core::flags_pennants_strategy(
		input.opens.as_ref().unwrap_or(&input.closes),
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
	.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}
