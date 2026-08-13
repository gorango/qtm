use crate::types::configs::LarssonConfig;
use crate::utils::signals::consolidating;
use crate::{StrategyError, StrategyResult};
use indicators_core::LarssonSignal;
use strategies_proc_macro::strategy;

/// Larsson Trend Strategy
///
/// Generates buy signals when price is above Fibonacci level
/// Generates sell signals when price is below Fibonacci level
#[strategy(
	id = "larsson",
	name = "Larsson Trend",
	category = "trend",
	default_timeframes = ["4h", "1d"],
	description = "Generates buy signals for impulse waves and sell signals for corrective waves using Larsson indicator with consolidating filter",
	opt_params = r#"[
		{"param_name": "useConsolidatingFilter", "min": 0.0, "max": 1.0, "step": 1.0},
		{"param_name": "consolidatingLookback", "min": 5.0, "max": 20.0, "step": 1.0},
		{"param_name": "consolidatingThresholdPct", "min": 0.01, "max": 0.05, "step": 0.005},
		{"param_name": "signalOffset", "min": -2.0, "max": 2.0, "step": 1.0}
	]"#
)]
pub fn larsson_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<LarssonConfig>,
) -> StrategyResult<Vec<i8>> {
	let config = config.unwrap_or_default();
	let use_consolidating_filter = config.use_consolidating_filter.unwrap_or(1) > 0;
	let consolidating_lookback = config.consolidating_lookback.unwrap_or(10) as usize;
	let consolidating_threshold_pct = config.consolidating_threshold_pct.unwrap_or(0.02);
	let signal_offset = config.signal_offset.unwrap_or(0);

	let data_len = closes.len();
	if highs.len() != data_len || lows.len() != data_len {
		return Err(StrategyError::Validation(
			"All input arrays must have equal length".into(),
		));
	}

	let warmup_period = 30; // Based on SMMA period used in Larsson indicator
	if data_len < warmup_period {
		return Err(StrategyError::InsufficientData(format!(
			"Insufficient data for Larsson strategy (need at least {warmup_period} bars)"
		)));
	}

	// Convert to vectors for reuse
	let highs_vec = highs;
	let lows_vec = lows;
	let closes_vec = closes;

	// Get Larsson indicator result
	let larsson_result = indicators_core::larsson(highs, lows);

	// Generate signals based on Larsson signals
	let mut signals = Vec::with_capacity(data_len);

	for i in 0..data_len {
		if i < warmup_period {
			signals.push(0); // Insufficient data for Larsson
			continue;
		}

		// Apply consolidating filter to avoid signals in low-volatility periods
		if use_consolidating_filter {
			let is_consolidating = consolidating(
				highs_vec,
				lows_vec,
				closes_vec,
				i,
				consolidating_lookback,
				consolidating_threshold_pct,
			);
			if is_consolidating {
				signals.push(0); // Hold during consolidation
				continue;
			}
		}

		// Apply signal offset (clamp to valid indices)
		let signal_index = (i as i32 + signal_offset).max(0).min(data_len as i32 - 1) as usize;

		// Map Larsson signals to strategy signals
		let signal = match &larsson_result.signal[signal_index] {
			LarssonSignal::P2 => 1,  // Buy on bullish condition
			LarssonSignal::P3 => -1, // Sell on bearish/downtrend
			LarssonSignal::P1 => 0,  // Hold on neutral
		};

		signals.push(signal);
	}

	Ok(signals)
}
