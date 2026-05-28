use napi_derive::napi;

use strategies_core::MadReversionConfig;

#[napi]
pub fn mad_reversion_strategy(
	closes: Vec<f64>,
	config: Option<MadReversionConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::mad_reversion_strategy(&closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}

pub fn mad_reversion_strategy_metadata() -> serde_json::Value {
	strategies_core::mad_reversion_strategy_metadata()
}

pub fn mad_reversion_strategy_defaults() -> serde_json::Value {
	strategies_core::mad_reversion_strategy_defaults()
}

pub fn mad_reversion(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<MadReversionConfig>(c).unwrap_or_default());
	strategies_core::mad_reversion_strategy(&input.closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}
