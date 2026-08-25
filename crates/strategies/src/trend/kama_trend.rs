use crate::types::configs::KamaTrendConfig;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// KAMA Trend Strategy
///
/// Buy when price crosses above the Kaufman Adaptive Moving Average, sell
/// when it crosses below — but only when the Kaufman efficiency ratio over
/// the same window is at least `er_threshold`. Crossings during chop are
/// ignored: KAMA already flattens in noise, the ER gate removes the rest.
#[strategy(
	id = "kama_trend",
	name = "KAMA Trend",
	category = "trend",
	default_timeframes = ["15m", "1h", "4h"],
	description = "Generates buy signals when price crosses above the Kaufman adaptive moving average and sell signals on crosses below it, gated by the Kaufman efficiency ratio so signals only fire in trending regimes",
	opt_params = r#"[
		{"param_name": "period", "min": 2.0, "max": 50.0, "step": 1.0},
		{"param_name": "er_threshold", "min": 0.0, "max": 0.8, "step": 0.05}
	]"#
)]
pub fn kama_trend_strategy(
	closes: &[f64],
	config: Option<KamaTrendConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let period = config.period.unwrap_or(10);
	let er_threshold = config.er_threshold.unwrap_or(0.3);

	if !(2..=200).contains(&period) {
		return Err(StrategyError::Validation(
			"KAMA period must be between 2 and 200".into(),
		));
	}
	if !(0.0..=1.0).contains(&er_threshold) {
		return Err(StrategyError::Validation(
			"ER threshold must be between 0 and 1".into(),
		));
	}

	let data_len = closes.len();
	if data_len <= period as usize {
		return Err(StrategyError::InsufficientData(
			"Insufficient data for KAMA Trend strategy".into(),
		));
	}

	let kama_config = indicators_core::KAMAConfig {
		period: Some(period),
		fast: None,
		slow: None,
	};
	let kama_values = indicators_core::kama(closes, Some(kama_config))?;
	let er = indicators_core::kaufman_efficiency_ratio(closes, Some(period))?;

	let mut signals = vec![0i8; data_len];
	for i in (period as usize + 1)..data_len {
		let (prev_k, cur_k) = (kama_values[i - 1], kama_values[i]);
		if prev_k.is_nan() || cur_k.is_nan() || er[i].is_nan() {
			continue;
		}
		if er[i] < er_threshold {
			continue; // choppy regime: stand aside
		}

		let crossed_up = closes[i - 1] <= prev_k && closes[i] > cur_k;
		let crossed_down = closes[i - 1] >= prev_k && closes[i] < cur_k;

		if crossed_up {
			signals[i] = 1;
		} else if crossed_down {
			signals[i] = -1;
		}
	}

	Ok(signals)
}

#[cfg(test)]
mod tests {
	use super::*;

	fn dip_then_rally() -> Vec<f64> {
		// decline for 40 bars, then a clean rally for 60 bars
		let mut closes: Vec<f64> = (0..40).map(|i| 100.0 - i as f64 * 0.5).collect();
		closes.extend((0..60).map(|i| 80.0 + i as f64));
		closes
	}

	#[test]
	fn dip_then_rally_fires_one_gated_buy() {
		let signals = kama_trend_strategy(&dip_then_rally(), None).unwrap();
		assert!(
			signals[..41].iter().all(|&s| s == 0),
			"no signals before the turn"
		);
		let buys = signals.iter().filter(|&&s| s == 1).count();
		assert_eq!(buys, 1, "exactly one cross-up buy, got {signals:?}");
		assert!(
			signals.iter().all(|&s| s != -1),
			"a pure rally never crosses down"
		);
	}

	#[test]
	fn chop_produces_no_signals_below_er_gate() {
		// perfect zigzag: price crosses KAMA constantly but ER ≈ 0
		let closes: Vec<f64> = (0..120).map(|i| 100.0 + (i % 2) as f64 * 2.0).collect();
		let signals = kama_trend_strategy(&closes, None).unwrap();
		assert!(signals.iter().all(|&s| s == 0));
	}

	#[test]
	fn zero_er_threshold_lets_chop_through() {
		let closes: Vec<f64> = (0..120).map(|i| 100.0 + (i % 2) as f64 * 2.0).collect();
		let ungated = kama_trend_strategy(
			&closes,
			Some(KamaTrendConfig {
				period: Some(10),
				er_threshold: Some(0.0),
			}),
		)
		.unwrap();
		assert!(
			ungated.iter().any(|&s| s != 0),
			"with the gate disabled, zigzag crossings must fire"
		);
	}

	#[test]
	fn rejects_invalid_params_and_short_data() {
		let bad_period = kama_trend_strategy(
			&vec![100.0; 50],
			Some(KamaTrendConfig {
				period: Some(500),
				er_threshold: None,
			}),
		);
		assert!(matches!(bad_period, Err(StrategyError::Validation(_))));

		let short = kama_trend_strategy(&[100.0; 5], None);
		assert!(matches!(short, Err(StrategyError::InsufficientData(_))));
	}
}
