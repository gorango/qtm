use crate::utils::validation::validate_multiple_arrays;
use crate::IndicatorResult;

/// Power of Three (AMD) — Accumulation / Manipulation / Distribution.
///
/// Structural-only (no session/clock). Callers filter the returned indices by
/// session if they want ICT-faithful London/NY alignment.
///
/// Definition per bar `i` (distribution bar):
/// 1. Accumulation: `accumulationPeriod` bars ending at `acc_end = i - manipulationBars - 1`
///    have tight range: `(accHigh - accLow) / accMid <= accumulationThreshold`.
/// 2. Manipulation: window `(acc_end+1 .. i)` contains a wick beyond the
///    accumulation extreme by `manipulationThreshold` (fraction of price) that
///    is then reclaimed (`close` back inside the range before `i`).
/// 3. Distribution: `close` breaks the *opposite* side of the accumulation
///    range at `i` (cross, not just outside).
///
/// Returns per bar: `0` none, `1.0` bullish AMD (bear trap → bull distribution),
/// `-1.0` bearish AMD (bull trap → bear distribution).
pub fn power_of_three(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	accumulation_period: Option<u32>,
	accumulation_threshold: Option<f64>,
	manipulation_threshold: Option<f64>,
	manipulation_bars: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[highs, lows, closes])?;

	let accumulation_period = accumulation_period.unwrap_or(20) as usize;
	let accumulation_threshold = accumulation_threshold.unwrap_or(0.015);
	let manipulation_threshold = manipulation_threshold.unwrap_or(0.005);
	let manipulation_bars = manipulation_bars.unwrap_or(5) as usize;

	let mut results = vec![0.0; highs.len()];

	if highs.len() < accumulation_period + manipulation_bars + 1 {
		return Ok(results);
	}
	if accumulation_threshold <= 0.0 || manipulation_threshold < 0.0 {
		return Ok(results);
	}

	for i in (accumulation_period + manipulation_bars)..highs.len() {
		let acc_end = i - manipulation_bars - 1;
		// acc_end is inclusive index, need at least accumulation_period bars ending at acc_end
		if acc_end + 1 < accumulation_period {
			continue;
		}
		let acc_start = acc_end + 1 - accumulation_period;

		// Accumulation range
		let acc_high = highs[acc_start..=acc_end]
			.iter()
			.fold(f64::NEG_INFINITY, |a, &b| a.max(b));
		let acc_low = lows[acc_start..=acc_end]
			.iter()
			.fold(f64::INFINITY, |a, &b| a.min(b));

		if !acc_high.is_finite() || !acc_low.is_finite() || acc_low <= 0.0 {
			continue;
		}
		let acc_mid = (acc_high + acc_low) * 0.5;
		if acc_mid <= 0.0 {
			continue;
		}
		let spread = (acc_high - acc_low) / acc_mid;
		if spread > accumulation_threshold {
			continue;
		}
		// Guard against zero-width accumulation (flat synthetic can still be valid
		// but needs some room for manipulation to exceed).
		if acc_high - acc_low < 1e-9 {
			continue;
		}

		let manip_start = acc_end + 1;
		// manipulation window is [manip_start, i) — must contain the trap wick
		// and the reclaim before distribution at i.
		if manip_start >= i {
			continue;
		}

		let manip_low = lows[manip_start..i]
			.iter()
			.fold(f64::INFINITY, |a, &b| a.min(b));
		let manip_high = highs[manip_start..i]
			.iter()
			.fold(f64::NEG_INFINITY, |a, &b| a.max(b));

		// Need a reclaim: close back inside the accumulation range before distribution.
		// Use i-1 as the last bar before distribution.
		let prev_close = closes[i - 1];

		// Bullish AMD: bear trap below accLow, then close back above accLow, then break above accHigh at i
		let bull_trap = manip_low < acc_low * (1.0 - manipulation_threshold);
		let bull_reclaimed = prev_close > acc_low;
		let bull_dist = closes[i] > acc_high && prev_close <= acc_high;

		// Bearish AMD: bull trap above accHigh, then close back below accHigh, then break below accLow at i
		let bear_trap = manip_high > acc_high * (1.0 + manipulation_threshold);
		let bear_reclaimed = prev_close < acc_high;
		let bear_dist = closes[i] < acc_low && prev_close >= acc_low;

		if bull_trap && bull_reclaimed && bull_dist {
			results[i] = 1.0;
		} else if bear_trap && bear_reclaimed && bear_dist {
			results[i] = -1.0;
		}
	}

	Ok(results)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn detects_bullish_po3() {
		// Accumulation: 20 bars flat 100..101 (spread ~1%)
		// Manipulation: dip to 97 (below 100*0.995=99.5), reclaim to 100.5 at i-1
		// Distribution: break above 101 at i to 102
		let mut highs = Vec::new();
		let mut lows = Vec::new();
		let mut closes = Vec::new();
		// 0..19 accumulation: oscillate 100-101
		for b in 0..20 {
			let c = if b % 2 == 0 { 100.5 } else { 100.8 };
			highs.push(101.0);
			lows.push(100.0);
			closes.push(c);
		}
		// 20..24 manipulation: trap
		highs.extend([100.6, 100.4, 100.3, 100.5, 100.5]);
		lows.extend([99.2, 97.0, 98.0, 99.5, 99.8]);
		closes.extend([99.0, 98.5, 99.8, 100.2, 100.5]); // last prev_close > accLow
												   // 25 distribution: break above accHigh 101
		highs.push(102.5);
		lows.push(100.8);
		closes.push(102.0);
		// pad a bit
		for _ in 0..5 {
			highs.push(103.0);
			lows.push(101.5);
			closes.push(102.5);
		}

		let sigs = power_of_three(
			&highs,
			&lows,
			&closes,
			Some(20),
			Some(0.02),
			Some(0.005),
			Some(5),
		)
		.unwrap();
		assert!(sigs.contains(&1.0), "no bullish PO3, signals: {sigs:?}");
		let idx = sigs.iter().position(|&s| s == 1.0).unwrap();
		assert_eq!(idx, 25, "expected distribution at 25, got {idx}");
	}

	#[test]
	fn detects_bearish_po3() {
		let mut highs = Vec::new();
		let mut lows = Vec::new();
		let mut closes = Vec::new();
		for b in 0..20 {
			let c = if b % 2 == 0 { 100.2 } else { 99.8 };
			highs.push(101.0);
			lows.push(100.0);
			closes.push(c);
		}
		// manipulation: bull trap above 101 to 103
		highs.extend([102.0, 103.0, 101.5, 101.2, 101.0]);
		lows.extend([100.2, 100.5, 100.3, 100.1, 100.0]);
		closes.extend([101.5, 101.2, 100.8, 100.4, 100.3]); // prev_close < accHigh
													  // distribution: break below 100
		highs.push(100.2);
		lows.push(98.0);
		closes.push(98.5);

		let sigs = power_of_three(
			&highs,
			&lows,
			&closes,
			Some(20),
			Some(0.02),
			Some(0.005),
			Some(5),
		)
		.unwrap();
		assert!(sigs.contains(&-1.0), "no bearish PO3, {sigs:?}");
	}

	#[test]
	fn rejects_wide_accumulation() {
		// Same as bullish but accumulation spread 10% > threshold 0.015 => no signal
		let mut highs = Vec::new();
		let mut lows = Vec::new();
		let mut closes = Vec::new();
		for b in 0..20 {
			highs.push(110.0);
			lows.push(90.0);
			closes.push(100.0 + (b as f64 % 3.0));
		}
		highs.extend([100.6, 100.4, 100.3, 100.5, 100.5, 102.5]);
		lows.extend([99.2, 97.0, 98.0, 99.5, 99.8, 100.8]);
		closes.extend([99.0, 98.5, 99.8, 100.2, 100.5, 102.0]);

		let sigs = power_of_three(
			&highs,
			&lows,
			&closes,
			Some(20),
			Some(0.015),
			Some(0.005),
			Some(5),
		)
		.unwrap();
		assert!(
			sigs.iter().all(|&s| s == 0.0),
			"should not fire on wide range"
		);
	}

	#[test]
	fn no_signal_on_insufficient_data() {
		let highs = vec![100.0; 10];
		let lows = vec![99.0; 10];
		let closes = vec![99.5; 10];
		let sigs = power_of_three(&highs, &lows, &closes, None, None, None, None).unwrap();
		assert!(sigs.iter().all(|&s| s == 0.0));
	}
}
