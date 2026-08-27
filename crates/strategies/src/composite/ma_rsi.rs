use crate::types::configs::MaRsiConfig;
use crate::utils::signals::{crossed_over_series, crossed_under_series};
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Ma Rsi Trend Following — strategy signal: `1` on entry long, `-1` on entry short, `0` otherwise (hold/flat).
/// See indicator docs for formula and regime notes. This is a thin signal wrapper.
#[strategy(
    id = "ma_rsi_trend_following",
    name = "MA + RSI Trend Following",
    category = "composite",
    default_timeframes = ["15m", "1h", "4h"],
    description = "Combine MA trend + RSI momentum",
    opt_params = r#"[{"param_name": "maPeriod", "min": 5.0, "max": 50.0, "step": 1.0}, {"param_name": "rsiPeriod", "min": 5.0, "max": 30.0, "step": 1.0}, {"param_name": "oversold", "min": 10.0, "max": 40.0, "step": 5.0}, {"param_name": "overbought", "min": 60.0, "max": 90.0, "step": 5.0}]"#
)]
pub fn ma_rsi_strategy(closes: &[f64], config: Option<MaRsiConfig>) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let ma_period = config.ma_period.unwrap_or(20);
	let rsi_period = config.rsi_period.unwrap_or(14);
	let oversold = config.oversold.unwrap_or(30.0);
	let overbought = config.overbought.unwrap_or(70.0);

	let data_len = closes.len();
	let min_periods = ma_period.max(rsi_period) as usize;

	if data_len < min_periods {
		return Err(StrategyError::InsufficientData(format!(
			"Insufficient data: MA + RSI requires at least {min_periods} data points, got {data_len}"
		)));
	}

	let closes_vec: Vec<f64> = closes.to_vec();
	let ma_values = indicators_core::sma(&closes_vec, Some(ma_period))?;

	let rsi_config = indicators_core::RSIConfig {
		period: Some(rsi_period),
	};
	let rsi_values = indicators_core::rsi(&closes_vec, Some(rsi_config));

	let mut signals = Vec::with_capacity(data_len);

	for (i, &rsi_value) in rsi_values.iter().enumerate().take(data_len) {
		let signal = if i < min_periods {
			0
		} else {
			let crossed_over_ma = crossed_over_series(&closes_vec, &ma_values, i as u32);
			let crossed_under_ma = crossed_under_series(&closes_vec, &ma_values, i as u32);

			if crossed_over_ma && rsi_value <= oversold {
				1
			} else if crossed_under_ma && rsi_value >= overbought {
				-1
			} else {
				0
			}
		};
		signals.push(signal);
	}

	Ok(signals)
}
