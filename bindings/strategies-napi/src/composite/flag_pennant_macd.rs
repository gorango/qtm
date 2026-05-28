use napi_derive::napi;

use strategies_core::FlagsPennantsConfig;
use strategies_core::MACDConfig;

#[napi]
pub fn flag_pennant_macd_strategy(
	highs: Vec<f64>,
	lows: Vec<f64>,
	closes: Vec<f64>,
	fp_config: Option<FlagsPennantsConfig>,
	macd_config: Option<MACDConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::flag_pennant_macd_strategy(&highs, &lows, &closes, fp_config, macd_config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}

pub fn flag_pennant_macd_strategy_metadata() -> serde_json::Value {
	strategies_core::flag_pennant_macd_strategy_metadata()
}

pub fn flag_pennant_macd_strategy_defaults() -> serde_json::Value {
	strategies_core::flag_pennant_macd_strategy_defaults()
}

pub fn flag_pennant_macd(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<FlagsPennantsConfig>(c).unwrap_or_default());
	strategies_core::flag_pennant_macd_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
		None,
	)
	.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}
