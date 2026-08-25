use crate::types::configs::KamaSlopeConfig;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// KAMA Slope State Machine Strategy
///
/// Signals fire once when the KAMA has risen for `consecutive_bars` straight
/// bars (buy) or fallen for `consecutive_bars` straight bars (sell). Because
/// KAMA's slope is nearly always exactly zero in chop, requiring consecutive
/// directional bars filters most whipsaw for free.
#[strategy(
	id = "kama_slope",
	name = "KAMA Slope State Machine",
	category = "trend",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals once the Kaufman adaptive moving average has risen for N consecutive bars and sell signals after N consecutive falling bars, using slope persistence as a noise filter",
	opt_params = r#"[
		{"param_name": "period", "min": 5.0, "max": 30.0, "step": 1.0},
		{"param_name": "consecutive_bars", "min": 2.0, "max": 8.0, "step": 1.0}
	]"#
)]
pub fn kama_slope_strategy(
	closes: &[f64],
	config: Option<KamaSlopeConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(10);
	let consecutive_bars = config.consecutive_bars.unwrap_or(3);

	if !(2..=200).contains(&period) {
		return Err(StrategyError::Validation(
			"KAMA period must be between 2 and 200".into(),
		));
	}
	if !(1..=50).contains(&consecutive_bars) {
		return Err(StrategyError::Validation(
			"consecutive_bars must be between 1 and 50".into(),
		));
	}

	let data_len = closes.len();
	if data_len <= period as usize + consecutive_bars as usize {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for KAMA Slope strategy".into(),
		));
	}

	let kama_values = indicators_core::kama(
		closes,
		Some(indicators_core::KAMAConfig {
			period: Some(period),
			fast: None,
			slow: None,
		}),
	)?;

	let mut signals = vec![0i8; data_len];
	let mut rising = 0u32;
	let mut falling = 0u32;

	for i in 1..data_len {
		let (prev, cur) = (kama_values[i - 1], kama_values[i]);
		if prev.is_nan() || cur.is_nan() {
			rising = 0;
			falling = 0;
			continue;
		}

		let delta = cur - prev;
		if delta > 0.0 {
			rising += 1;
			falling = 0;
		} else if delta < 0.0 {
			falling += 1;
			rising = 0;
		} else {
			rising = 0;
			falling = 0;
		}

		if rising == consecutive_bars {
			signals[i] = 1;
		} else if falling == consecutive_bars {
			signals[i] = -1;
		}
	}

	Ok(signals)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rally_signals_once_when_streak_reaches_n() {
		let closes: Vec<f64> = (0..60).map(|i| 100.0 + i as f64).collect();
		let signals = kama_slope_strategy(&closes, None).unwrap();

		let buys: Vec<usize> = signals
			.iter()
			.enumerate()
			.filter(|(_, &s)| s == 1)
			.map(|(i, _)| i)
			.collect();
		assert_eq!(buys, vec![10 + 3], "one buy exactly when the streak hits 3");
		assert!(signals.iter().all(|&s| s != -1));
	}

	#[test]
	fn decline_signals_once_downside() {
		let closes: Vec<f64> = (0..60).map(|i| 200.0 - i as f64).collect();
		let signals = kama_slope_strategy(&closes, None).unwrap();
		assert_eq!(signals.iter().filter(|&&s| s == -1).count(), 1);
		assert!(signals.iter().all(|&s| s != 1));
	}

	#[test]
	fn chop_never_arms_the_state_machine() {
		// alternating closes keep every KAMA delta tiny but nonzero —
		// direction still flips each bar, so streaks never reach 3
		let closes: Vec<f64> = (0..120).map(|i| 100.0 + (i % 2) as f64).collect();
		let signals = kama_slope_strategy(&closes, None).unwrap();
		assert!(signals.iter().all(|&s| s == 0));
	}

	#[test]
	fn insufficient_data_errors() {
		let short = kama_slope_strategy(&[100.0; 12], None);
		assert!(matches!(short, Err(StrategyError::InsufficientData(_))));
	}
}
