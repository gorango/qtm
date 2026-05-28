use napi_derive::napi;

use strategies_core::BollingerBandsConfig;

#[napi]
pub fn bollinger_bands_mean_reversion_strategy(
	closes: Vec<f64>,
	config: Option<BollingerBandsConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::bollinger_bands_mean_reversion_strategy(&closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}

pub fn bollinger_bands_mean_reversion_strategy_metadata() -> serde_json::Value {
	strategies_core::bollinger_bands_mean_reversion_strategy_metadata()
}

pub fn bollinger_bands_mean_reversion_strategy_defaults() -> serde_json::Value {
	strategies_core::bollinger_bands_mean_reversion_strategy_defaults()
}

pub fn bollinger_bands_mean_reversion(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<BollingerBandsConfig>(c).unwrap_or_default());
	strategies_core::bollinger_bands_mean_reversion_strategy(&input.closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}
