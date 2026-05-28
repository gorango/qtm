use crate::types::configs::DoubleTopStochasticConfig;

/// Double Top Stochastic
///
/// Sell on double-top pattern with stochastic overbought crossover.
pub fn double_top_stochastic_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<DoubleTopStochasticConfig>,
) -> Result<Vec<i8>, String> {
	let config = config.unwrap_or_default();
	let min_distance = config.min_distance.unwrap_or(10);
	let tolerance = config.tolerance.unwrap_or(0.03);
	let k_period = config.k_period.unwrap_or(14);
	let d_period = config.d_period.unwrap_or(3);
	let oversold = config.oversold.unwrap_or(20.0);
	let overbought = config.overbought.unwrap_or(80.0);

	let lookaround = 2;

	let closes_vec: Vec<f64> = closes.to_vec();
	let highs_vec: Vec<f64> = highs.to_vec();
	let lows_vec: Vec<f64> = lows.to_vec();
	let opens_vec: Vec<f64> = closes_vec.clone();
	let double_bottom_signals = indicators_core::double_bottom(
		&opens_vec,
		&highs_vec,
		&lows_vec,
		&closes_vec,
		Some(tolerance),
		Some(min_distance),
		Some(lookaround),
	)?;
	let double_top_signals = indicators_core::double_top(
		&opens_vec,
		&highs_vec,
		&lows_vec,
		&closes_vec,
		Some(tolerance),
		Some(min_distance),
		Some(lookaround),
	)?;

	let stoch_config = indicators_core::StochConfig {
		k_period: Some(k_period),
		d_period: Some(d_period),
	};
	let highs_vec: Vec<f64> = highs.to_vec();
	let lows_vec: Vec<f64> = lows.to_vec();
	let stoch_result = indicators_core::stochastic_oscillator(
		&highs_vec,
		&lows_vec,
		&closes_vec,
		Some(stoch_config),
	);

	let data_len = closes.len();
	let mut signals = Vec::with_capacity(data_len);

	let double_bottom = &double_bottom_signals;
	let double_top = &double_top_signals;

	for i in 0..data_len {
		let signal = if i < (k_period + d_period) as usize {
			0
		} else if double_bottom[i] == 1.0 && stoch_result.k[i] <= oversold {
			1
		} else if double_top[i] == 1.0 && stoch_result.k[i] >= overbought {
			-1
		} else {
			0
		};
		signals.push(signal);
	}

	Ok(signals)
}

pub fn double_top_stochastic_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "double-top-stochastic-reversal",
		"name": "Double Top/Bottom + Stochastic Reversal",
		"category": "composite",
		"description": "Double Top/Bottom pattern + Stochastic confirmation",
		"default_timeframes": ["15m", "1h", "4h"]
	})
}

pub fn double_top_stochastic_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"min_distance": 10,
			"tolerance": 0.03,
			"k_period": 14,
			"d_period": 3,
			"oversold": 20.0,
			"overbought": 80.0
		},
		"optimization_bounds": [
			{
				"param_name": "min_distance",
				"min": 5.0,
				"max": 20.0,
				"step": 1.0
			},
			{
				"param_name": "tolerance",
				"min": 0.01,
				"max": 0.1,
				"step": 0.01
			},
			{
				"param_name": "k_period",
				"min": 5.0,
				"max": 30.0,
				"step": 1.0
			},
			{
				"param_name": "d_period",
				"min": 2.0,
				"max": 10.0,
				"step": 1.0
			},
			{
				"param_name": "oversold",
				"min": 10.0,
				"max": 30.0,
				"step": 5.0
			},
			{
				"param_name": "overbought",
				"min": 70.0,
				"max": 90.0,
				"step": 5.0
			}
		]
	})
}
