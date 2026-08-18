use crate::utils::validation::validate_multiple_arrays;
use crate::IndicatorResult;

/// Triple top (bearish reversal).
///
/// Three peaks at roughly the same price level separated by a minimum
/// distance. Confirmed when a close breaks down through the neckline (the
/// lowest low spanned by the three peaks).
pub fn triple_top(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	tolerance: Option<f64>,
	min_separation: Option<u32>,
	lookaround: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	triple_top_bottom_internal(
		opens,
		highs,
		lows,
		closes,
		false,
		tolerance,
		min_separation,
		lookaround,
	)
}

/// Triple bottom (bullish reversal).
///
/// Three troughs at roughly the same price level. Confirmed when a close
/// breaks up through the neckline (the highest high spanned by the troughs).
pub fn triple_bottom(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	tolerance: Option<f64>,
	min_separation: Option<u32>,
	lookaround: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	triple_top_bottom_internal(
		opens,
		highs,
		lows,
		closes,
		true,
		tolerance,
		min_separation,
		lookaround,
	)
}

fn triple_top_bottom_internal(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	bullish: bool,
	tolerance: Option<f64>,
	min_separation: Option<u32>,
	lookaround: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let tolerance = tolerance.unwrap_or(0.03);
	let min_separation = min_separation.unwrap_or(8) as usize;
	let lookaround = lookaround.unwrap_or(2) as usize;

	let mut results = vec![0.0; highs.len()];

	let pivots = if bullish {
		crate::patterns::helpers::find_troughs_internal(lows, lookaround)
	} else {
		crate::patterns::helpers::find_peaks_internal(highs, lookaround)
	};

	if pivots.len() < 3 {
		return Ok(results);
	}

	// `saturating_sub(2)` matters: with exactly 3 pivots the loop must still
	// run once (index 0 on its own). With `saturating_sub(3)` it would loop
	// zero times and never fire.
	for i in 0..pivots.len().saturating_sub(2) {
		let p1 = pivots[i];
		let p2 = pivots[i + 1];
		let p3 = pivots[i + 2];

		if p2 - p1 < min_separation || p3 - p2 < min_separation {
			continue;
		}

		let v1 = if bullish { lows[p1] } else { highs[p1] };
		let v2 = if bullish { lows[p2] } else { highs[p2] };
		let v3 = if bullish { lows[p3] } else { highs[p3] };

		let avg = (v1 + v2 + v3) / 3.0;
		if (v1 - avg).abs() / avg > tolerance
			|| (v2 - avg).abs() / avg > tolerance
			|| (v3 - avg).abs() / avg > tolerance
		{
			continue;
		}

		// Middle pivot should be the extreme (deepest low / highest peak).
		if bullish && (v2 > v1 || v2 > v3) {
			continue;
		}
		if !bullish && (v2 < v1 || v2 < v3) {
			continue;
		}

		let neckline = if bullish {
			highs[p1..=p3]
				.iter()
				.fold(f64::NEG_INFINITY, |a, &b| a.max(b))
		} else {
			lows[p1..=p3].iter().fold(f64::INFINITY, |a, &b| a.min(b))
		};

		for k in (p3 + 1)..highs.len() {
			let confirmed = if bullish {
				closes[k] > neckline
			} else {
				closes[k] < neckline
			};
			if confirmed {
				results[k] = if bullish { 1.0 } else { -1.0 };
				break;
			}
		}
	}

	Ok(results)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::patterns::helpers::test_helpers::*;

	#[test]
	fn detects_triple_top_breakdown() {
		// Three peaks near 100 with intervening dips, then a breakdown.
		let pivots = [
			(0, 96.0),
			(10, 100.0),
			(20, 97.0),
			(30, 100.5),
			(40, 96.5),
			(50, 100.0),
			(60, 93.0),
		];
		let closes = series_from_pivots(&pivots, 80);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let signals = triple_top(&opens, &highs, &lows, &closes, None, Some(5), Some(2)).unwrap();

		assert!(signals.iter().any(|&s| s < -0.5), "no bearish signal");
		let idx = signals.iter().position(|&s| s < -0.5).unwrap();
		assert!(
			idx > 50,
			"signal should fire after the third peak, got {idx}"
		);
	}

	#[test]
	fn detects_triple_bottom_breakout() {
		let pivots = [
			(0, 104.0),
			(10, 100.0),
			(20, 103.0),
			(30, 99.5),
			(40, 103.5),
			(50, 100.0),
			(60, 107.0),
		];
		let closes = series_from_pivots(&pivots, 80);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let signals =
			triple_bottom(&opens, &highs, &lows, &closes, None, Some(5), Some(2)).unwrap();

		assert!(signals.iter().any(|&s| s > 0.5), "no bullish signal");
		let idx = signals.iter().position(|&s| s > 0.5).unwrap();
		assert!(
			idx > 50,
			"signal should fire after the third trough, got {idx}"
		);
	}
}
