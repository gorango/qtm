use napi_derive::napi;

use strategies_core::DoubleTopStochasticConfig;

#[napi]
pub fn double_top_stochastic_strategy(
	highs: Vec<f64>,
	lows: Vec<f64>,
	closes: Vec<f64>,
	config: Option<DoubleTopStochasticConfig>,
) -> napi::Result<Vec<i8>> {
	strategies_core::double_top_stochastic_strategy(&highs, &lows, &closes, config)
		.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}

pub fn double_top_stochastic_strategy_metadata() -> serde_json::Value {
	strategies_core::double_top_stochastic_strategy_metadata()
}

pub fn double_top_stochastic_strategy_defaults() -> serde_json::Value {
	strategies_core::double_top_stochastic_strategy_defaults()
}

pub fn double_top_stochastic(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let config =
		config.map(|c| serde_json::from_value::<DoubleTopStochasticConfig>(c).unwrap_or_default());
	strategies_core::double_top_stochastic_strategy(
		input.highs.as_ref().unwrap_or(&input.closes),
		input.lows.as_ref().unwrap_or(&input.closes),
		&input.closes,
		config,
	)
	.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
}
