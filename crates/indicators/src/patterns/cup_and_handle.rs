use crate::utils::validation::validate_multiple_arrays;
use crate::IndicatorResult;

/// Cup and Handle — detects cup shape via trough and handle via flag.
/// Scores 0..100 per bar; higher = cleaner pattern. Heuristic; no canonical formula.
pub fn cup_and_handle(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	cup_depth: Option<f64>,
	handle_retracement: Option<f64>,
	min_duration: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let cup_depth = cup_depth.unwrap_or(0.15);
	let handle_retracement = handle_retracement.unwrap_or(0.3);
	let min_duration = min_duration.unwrap_or(20) as usize;

	let mut results = vec![0.0; highs.len()];

	if highs.len() < min_duration {
		return Ok(results);
	}

	// All troughs, not just the last 3: previously `rev().take(3)` meant a
	// cup that completed mid-history was never examined.
	let troughs = crate::patterns::helpers::find_troughs_internal(lows, 5);

	for &bottom_index in &troughs {
		if bottom_index < min_duration / 2 || bottom_index > highs.len() - min_duration / 2 {
			continue;
		}

		let bottom_price = lows[bottom_index];

		let left_shoulder = (bottom_index.saturating_sub(min_duration / 2)..bottom_index)
			.rev()
			.max_by(|&i, &j| {
				highs[i]
					.partial_cmp(&highs[j])
					.unwrap_or(std::cmp::Ordering::Equal)
			})
			.map(|i| (i, highs[i]));

		let (_left_shoulder_index, left_shoulder_price) = match left_shoulder {
			Some((idx, price)) => (idx, price),
			None => continue,
		};

		let right_shoulder = (bottom_index + 1..(bottom_index + min_duration / 2).min(highs.len()))
			.max_by(|&i, &j| {
				highs[i]
					.partial_cmp(&highs[j])
					.unwrap_or(std::cmp::Ordering::Equal)
			})
			.map(|i| (i, highs[i]));

		let (right_shoulder_index, right_shoulder_price) = match right_shoulder {
			Some((idx, price)) => (idx, price),
			None => continue,
		};

		let avg_shoulder = (left_shoulder_price + right_shoulder_price) / 2.0;
		let depth = (avg_shoulder - bottom_price) / avg_shoulder;

		if depth < cup_depth {
			continue;
		}

		// Handle window EXCLUDES the right-shoulder bar: starting at the
		// shoulder would make handle_high >= right_shoulder_price, forcing
		// retracement >= 1.0 and rejecting every cup.
		let handle_start = right_shoulder_index + 1;
		let handle_end = (handle_start + min_duration / 4).min(highs.len() - 1);

		if handle_end - handle_start < 5 {
			continue;
		}

		let handle_high = highs[handle_start..=handle_end]
			.iter()
			.fold(f64::NEG_INFINITY, |a, &b| a.max(b));
		let handle_low = lows[handle_start..=handle_end]
			.iter()
			.fold(f64::INFINITY, |a, &b| a.min(b));

		// Handle depth: the pullback from the rim (right shoulder) to the
		// handle's low, as a fraction of the cup's advance (rim - bottom).
		// The previous form `(handle_high - handle_low) / (rim - handle_low)`
		// compared the handle's own range against its drawdown-from-rim;
		// since the window starts one bar after the local top, its high sits
		// near the rim and the ratio approaches 1.0 for any realistic
		// pullback — rejecting effectively every real-data cup.
		let cup_advance = right_shoulder_price - bottom_price;
		if cup_advance <= 0.0 {
			continue;
		}
		let retracement = (right_shoulder_price - handle_low) / cup_advance;

		if retracement > handle_retracement {
			continue;
		}

		for i in (handle_end + 1)..highs.len() {
			if closes[i] > handle_high {
				results[i] = 1.0;
				break;
			}
		}
	}

	Ok(results)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::patterns::helpers::test_helpers::{bar, ohlc};

	/// Builds a V-shaped cup: descent to `bottom`, ascent back to the rim at
	/// `rim_price`, then `handle` bars pulling back, then a breakout bar.
	fn cup_series(bottom: f64, rim_price: f64, handle_low: f64) -> Vec<[f64; 4]> {
		let mut bars: Vec<[f64; 4]> = Vec::new();
		let price = rim_price + 3.0;
		// Descent (bars 0..10): drift down to the bottom.
		for i in 0..10 {
			let p = price - (price - bottom) * ((i + 1) as f64 / 11.0);
			bars.push(bar(p + 0.4, p + 0.6, p - 0.6, p));
		}
		// Bottom bar (index 10).
		bars.push(bar(bottom + 0.5, bottom + 0.8, bottom, bottom + 0.3));
		// Ascent (bars 11..19): recover toward the rim.
		for i in 1..=9 {
			let p = bottom + (rim_price - 1.0 - bottom) * (i as f64 / 9.0);
			bars.push(bar(p - 0.3, p + 0.5, p - 0.5, p));
		}
		// Rim bar (index 20).
		bars.push(bar(
			rim_price - 0.4,
			rim_price,
			rim_price - 0.9,
			rim_price - 0.2,
		));
		// Handle (bars 21..25): shallow pullback.
		let steps = [
			(rim_price - 0.8),
			(handle_low + 0.9),
			(handle_low + 0.5),
			(handle_low + 0.7),
			(handle_low + 1.1),
		];
		for &p in &steps {
			bars.push(bar(p + 0.2, p + 0.5, p - 0.5, p));
		}
		// Breakout bar (index 26): close above every handle high.
		bars.push(bar(
			handle_low + 2.0,
			rim_price + 0.7,
			handle_low + 1.6,
			rim_price + 0.5,
		));
		bars
	}

	#[test]
	fn detects_cup_and_handle_breakout() {
		// Cup: rim ~100, bottom 80 (depth ~0.21). Handle pulls back to 97
		// (retracement (100-97)/(100-80) = 0.15 <= 0.3), then a close above
		// the handle's high fires +1.
		let bars = cup_series(80.0, 100.0, 97.0);
		let (opens, highs, lows, closes) = ohlc(&bars);

		let signals = cup_and_handle(&opens, &highs, &lows, &closes, None, None, None).unwrap();

		assert!(signals.iter().any(|&s| s > 0.5), "no breakout signal");
	}

	#[test]
	fn rejects_deep_handle() {
		// Same cup but the handle gives back half the advance
		// ((100-90)/(100-80) = 0.5 > 0.3): must not fire.
		let bars = cup_series(80.0, 100.0, 90.0);
		let (opens, highs, lows, closes) = ohlc(&bars);

		let signals = cup_and_handle(&opens, &highs, &lows, &closes, None, None, None).unwrap();

		assert!(
			signals.iter().all(|&s| s == 0.0),
			"unexpected signal with over-deep handle"
		);
	}
}
