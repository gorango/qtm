use crate::types::configs::DualKamaCrossoverConfig;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Dual KAMA Crossover Strategy
///
/// Buy when the fast KAMA crosses above the slow KAMA, sell on the opposite
/// cross. Both legs adapt their smoothing to the efficiency ratio, so unlike
/// an SMA crossover the slow side speeds up in clean trends instead of
/// lagging fixed.
#[strategy(
	id = "dual_kama_crossover",
	name = "Dual KAMA Crossover",
	category = "trend",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when the fast Kaufman adaptive moving average crosses above the slow one and sell signals on the opposite cross, with both smoothing constants adapting to trend efficiency",
	opt_params = r#"[
		{"param_name": "fast_period", "min": 2.0, "max": 15.0, "step": 1.0},
		{"param_name": "slow_period", "min": 10.0, "max": 60.0, "step": 5.0}
	]"#
)]
pub fn dual_kama_crossover_strategy(
	closes: &[f64],
	config: Option<DualKamaCrossoverConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let fast_period = config.fast_period.unwrap_or(5);
	let slow_period = config.slow_period.unwrap_or(20);

	if !(2..=200).contains(&fast_period) || !(2..=200).contains(&slow_period) {
		return Err(StrategyError::Validation(
			"KAMA periods must be between 2 and 200".into(),
		));
	}
	if fast_period >= slow_period {
		return Err(StrategyError::Validation(
			"fast_period must be less than slow_period".into(),
		));
	}

	let data_len = closes.len();
	if data_len <= slow_period as usize {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for Dual KAMA Crossover strategy".into(),
		));
	}

	let make_config = |period: u32| indicators_core::KAMAConfig {
		period: Some(period),
		fast: None,
		slow: None,
	};
	let fast = indicators_core::kama(closes, Some(make_config(fast_period)))?;
	let slow = indicators_core::kama(closes, Some(make_config(slow_period)))?;

	let mut signals = vec![0i8; data_len];
	// Both KAMAs share the same smoothing constant in perfectly efficient
	// trends and converge to the same lag, so near-equality is float noise,
	// not a cross. Only move the state on separations outside a small
	// relative tolerance; inside the dead-band the previous state persists.
	const CROSS_TOLERANCE: f64 = 1e-9;
	let mut prev_above: Option<bool> = None;

	for i in (slow_period as usize + 1)..data_len {
		if [fast[i], slow[i]].iter().any(|v| v.is_nan()) {
			continue;
		}

		let tol = CROSS_TOLERANCE * fast[i].abs().max(slow[i].abs()).max(1.0);
		let now = if fast[i] > slow[i] + tol {
			true
		} else if fast[i] < slow[i] - tol {
			false
		} else {
			match prev_above {
				Some(state) => state,
				None => continue,
			}
		};

		if let Some(prev) = prev_above {
			if !prev && now {
				signals[i] = 1;
			} else if prev && !now {
				signals[i] = -1;
			}
		}
		prev_above = Some(now);
	}

	Ok(signals)
}

#[cfg(test)]
mod tests {
	use super::*;

	fn rise_then_fall() -> Vec<f64> {
		let mut closes: Vec<f64> = (0..60).map(|i| 100.0 + i as f64).collect();
		closes.extend((0..60).map(|i| 160.0 - i as f64));
		closes
	}

	#[test]
	fn fires_buy_on_turn_up_and_sell_on_turn_down() {
		let signals = dual_kama_crossover_strategy(&rise_then_fall(), None).unwrap();
		assert!(
			signals.contains(&1),
			"a V-shape must produce at least one buy"
		);
		assert!(
			signals.contains(&(-1)),
			"a V-shape must produce at least one sell"
		);
		// no signals before both series have warmed up
		assert!(signals[..21].iter().all(|&s| s == 0));
	}

	#[test]
	fn monotonic_series_never_flips() {
		let closes: Vec<f64> = (0..100).map(|i| 100.0 + i as f64 * 0.25).collect();
		let signals = dual_kama_crossover_strategy(&closes, None).unwrap();
		assert!(
			signals.iter().all(|&s| s == 0),
			"unexpected flips: {signals:?}"
		);
	}

	#[test]
	fn requires_fast_below_slow() {
		let err = dual_kama_crossover_strategy(
			&vec![100.0; 100],
			Some(DualKamaCrossoverConfig {
				fast_period: Some(20),
				slow_period: Some(20),
			}),
		);
		assert!(matches!(err, Err(StrategyError::Validation(_))));
	}
}
