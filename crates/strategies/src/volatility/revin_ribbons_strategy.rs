use crate::types::configs::RevinRibbonsStrategyConfig;
use crate::{StrategyError, StrategyResult};
use strategies_proc_macro::strategy;

/// Revin Ribbons Strategy — open approximation.
///
/// Core premise (per TV docs): use the ribbons midline as trend bias.
/// Close above midline = long, close below = short. Each band above/below
/// can serve as a profit level (not modeled here; this signal file models
/// the entry/flip logic only). Loses in chop, gains in trends.
///
/// Extra TV controls approximated:
/// - `midline_forgiveness_pct` — % of band width, not hard %. Barely-crossed
///   midline does not flip.
/// - `max_consecutive_flips` + `cooldown_bars` — after N flips, wait M bars.
/// - `exit_flat_on_cooldown` — if false, hold through cooldown instead of flat.
#[strategy(
	id = "revin_ribbons_strategy",
	name = "Revin Ribbons Strategy",
	category = "volatility",
	default_timeframes = ["1h", "4h", "1d"],
	description = "Open approximation of Revin Ribbons midline flip system: close>midline long, close<midline short, with forgiveness/cooldown filters and optional flat-on-cooldown. Loses in chop, captures trends.",
	opt_params = r#"[
		{"param_name": "period", "min": 5.0, "max": 50.0, "step": 1.0},
		{"param_name": "atrPeriod", "min": 5.0, "max": 50.0, "step": 1.0},
		{"param_name": "midlineForgivenessPct", "min": 0.0, "max": 50.0, "step": 5.0},
		{"param_name": "maxConsecutiveFlips", "min": 1.0, "max": 5.0, "step": 1.0},
		{"param_name": "cooldownBars", "min": 0.0, "max": 20.0, "step": 1.0}
	]"#
)]
pub fn revin_ribbons_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<RevinRibbonsStrategyConfig>,
) -> StrategyResult<Vec<i8>> {
	let cfg = config.unwrap_or_default();
	let period = cfg.period.unwrap_or(20);
	let atr_period = cfg.atr_period.unwrap_or(14);
	let s1_mult = cfg.s1_mult.unwrap_or(1.5);
	let s2_mult = cfg.s2_mult.unwrap_or(2.5);
	let s3_mult = cfg.s3_mult.unwrap_or(3.5);
	let forgiveness = cfg.midline_forgiveness_pct.unwrap_or(0.0);
	let max_flips = cfg.max_consecutive_flips.unwrap_or(3);
	let cooldown_bars = cfg.cooldown_bars.unwrap_or(5);
	let exit_flat = cfg.exit_flat_on_cooldown.unwrap_or(true);

	if !(2..=100).contains(&period) {
		return Err(StrategyError::Validation(
			"Revin Ribbons period must be between 2 and 100".into(),
		));
	}
	if !(2..=100).contains(&atr_period) {
		return Err(StrategyError::Validation(
			"Revin Ribbons atr_period must be between 2 and 100".into(),
		));
	}
	if !(0.1..=10.0).contains(&s1_mult)
		|| !(0.1..=10.0).contains(&s2_mult)
		|| !(0.1..=10.0).contains(&s3_mult)
	{
		return Err(StrategyError::Validation(
			"Revin Ribbons multipliers must be between 0.1 and 10".into(),
		));
	}
	if !(0.0..=100.0).contains(&forgiveness) {
		return Err(StrategyError::Validation(
			"midline_forgiveness_pct must be between 0 and 100".into(),
		));
	}

	let ribbons_cfg = indicators_core::RevinRibbonsConfig {
		period: Some(period),
		atr_period: Some(atr_period),
		s1_mult: Some(s1_mult),
		s2_mult: Some(s2_mult),
		s3_mult: Some(s3_mult),
	};
	let ribbons = indicators_core::revin_ribbons(highs, lows, closes, Some(ribbons_cfg))?;
	let n = closes.len();
	let mut signals = vec![0i8; n];

	let mut direction: i8 = 0; // 1 long, -1 short, 0 flat
	let mut flips_in_window: u32 = 0;
	let mut cooldown_remaining: u32 = 0;
	let mut last_flip_idx: Option<usize> = None;

	for i in 0..n {
		let mid = ribbons.midline[i];
		let r1 = ribbons.r1[i];
		let s1 = ribbons.s1[i];
		let close = closes[i];

		if mid.is_nan() || r1.is_nan() || s1.is_nan() || close.is_nan() {
			signals[i] = 0;
			continue;
		}

		// Cooldown handling
		if cooldown_remaining > 0 {
			cooldown_remaining -= 1;
			if exit_flat {
				direction = 0;
			}
			signals[i] = 0;
			continue;
		}

		let band_width = (r1 - s1).abs();
		let forgiveness_val = band_width * (forgiveness / 100.0);

		// Determine desired direction with forgiveness
		// Close must exceed midline by forgiveness to flip long, or drop below by forgiveness to flip short.
		let desired = if close > mid + forgiveness_val {
			1
		} else if close < mid - forgiveness_val {
			-1
		} else {
			// inside forgiveness zone -> no change, hold last direction if any, else stay flat
			direction
		};

		let flipped = desired != 0 && desired != direction;

		if flipped {
			// Count consecutive flips in short window (since last flip)
			if let Some(last) = last_flip_idx {
				// if within 2*period bars, consider it consecutive chop
				if i - last <= (period as usize * 2).max(5) {
					flips_in_window += 1;
				} else {
					flips_in_window = 1;
				}
			} else {
				flips_in_window = 1;
			}
			last_flip_idx = Some(i);

			if flips_in_window >= max_flips {
				// trigger cooldown after emitting this flip
				cooldown_remaining = cooldown_bars;
				flips_in_window = 0;
			}

			direction = desired;
			signals[i] = direction;
		} else if direction != 0 {
			// No flip, but we are in a position — maintain exposure as 0 signal (hold)
			// Strategy contract in this repo emits 1/-1 only on entry bars, 0 otherwise.
			// So we keep signals[i] = 0 for hold.
			signals[i] = 0;
		} else {
			// flat and no signal
			signals[i] = 0;
		}
	}

	Ok(signals)
}

/// Alias id without _strategy suffix for convenience.
pub fn revin_ribbons_alias_strategy(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	config: Option<RevinRibbonsStrategyConfig>,
) -> StrategyResult<Vec<i8>> {
	revin_ribbons_strategy(highs, lows, closes, config)
}
