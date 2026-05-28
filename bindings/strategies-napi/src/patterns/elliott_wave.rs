use napi_derive::napi;

#[allow(clippy::too_many_arguments)]
#[napi]
pub fn elliott_wave_strategy(
	opens: Vec<f64>,
	highs: Vec<f64>,
	lows: Vec<f64>,
	closes: Vec<f64>,
	wave2_retracement: f64,
	wave4_retracement: f64,
	wave3_min_extension: f64,
	min_wave_separation: u32,
	lookaround: u32,
	retracement_tolerance: f64,
) -> napi::Result<Vec<i8>> {
	strategies_core::elliott_wave_strategy(
		&opens,
		&highs,
		&lows,
		&closes,
		wave2_retracement,
		wave4_retracement,
		wave3_min_extension,
		min_wave_separation,
		lookaround,
		retracement_tolerance,
	)
	.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}

pub fn elliott_wave_strategy_metadata() -> serde_json::Value {
	strategies_core::elliott_wave_strategy_metadata()
}

pub fn elliott_wave_strategy_defaults() -> serde_json::Value {
	strategies_core::elliott_wave_strategy_defaults()
}

pub fn elliott_wave(
	input: &crate::StrategyInput,
	config: Option<serde_json::Value>,
) -> napi::Result<Vec<i8>> {
	let _ = config;
	let opens = input.opens.as_ref().unwrap_or(&input.closes);
	let highs = input.highs.as_ref().unwrap_or(&input.closes);
	let lows = input.lows.as_ref().unwrap_or(&input.closes);
	strategies_core::elliott_wave_strategy(
		opens,
		highs,
		lows,
		&input.closes,
		0.5,
		0.38,
		1.618,
		5,
		2,
		0.05,
	)
	.map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
}
