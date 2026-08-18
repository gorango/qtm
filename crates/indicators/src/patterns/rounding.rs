use crate::utils::validation::validate_multiple_arrays;
use crate::IndicatorResult;

/// Rounding bottom / saucer (bullish reversal).
///
/// A wide, shallow U-shape: price declines, bottoms out in the middle of the
/// window, and recovers toward its starting level. Curvature is measured with
/// a least-squares parabola fit on the closes of the whole lookback window.
/// Confirmed when a close breaks back above the rim (the midpoint between the
/// window's starting and current close levels).
pub fn rounding_bottom(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	curvature_tolerance: Option<f64>,
	lookback: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	rounding_internal(opens, highs, lows, closes, true, curvature_tolerance, lookback)
}

/// Rounding top (bearish reversal).
///
/// Mirror of [`rounding_bottom`]: a wide, shallow ∩-shape in the closes.
/// Confirmed when a close breaks back below the pattern's floor.
pub fn rounding_top(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	curvature_tolerance: Option<f64>,
	lookback: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	rounding_internal(opens, highs, lows, closes, false, curvature_tolerance, lookback)
}

fn rounding_internal(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	bullish: bool,
	curvature_tolerance: Option<f64>,
	lookback: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let curvature_tolerance = curvature_tolerance.unwrap_or(0.01);
	let lookback = lookback.unwrap_or(120) as usize;

	let mut results = vec![0.0; highs.len()];

	if highs.len() < lookback + 2 {
		return Ok(results);
	}

	for i in lookback..closes.len() {
		let start = i - lookback;

		// Least-squares parabola over the whole window's closes. `x` is
		// rebased to the window start so the span is well-conditioned.
		let mut points = Vec::with_capacity((i - start + 1) * 2);
		let mut sum = 0.0;
		for (k, &c) in closes[start..=i].iter().enumerate() {
			points.push(k as f64);
			points.push(c);
			sum += c;
		}
		let mean_close = sum / (i - start + 1) as f64;

		let quad = crate::patterns::helpers::quadratic_regression_internal(&points);
		let a = quad[0];

		// Normalized curvature: a * span^2 relative to the mean close. Positive
		// (convex, saucer) for a bottom, negative for a top.
		let span = (i - start) as f64;
		let rel_curvature = a * span * span / mean_close;

		if bullish && rel_curvature < curvature_tolerance {
			continue;
		}
		if !bullish && rel_curvature > -curvature_tolerance {
			continue;
		}

		// Rim/floor estimate: the midpoint between the window's left edge and
		// the current close. The breakout fires when the close crosses it.
		let level = (closes[start] + closes[i]) / 2.0;

		if bullish && closes[i - 1] <= level && closes[i] > level {
			results[i] = 1.0;
		} else if !bullish && closes[i - 1] >= level && closes[i] < level {
			results[i] = -1.0;
		}
	}

	Ok(results)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::patterns::helpers::test_helpers::*;

	#[test]
	fn detects_rounding_bottom_breakout() {
		// A flat baseline around 100 (bars 0-15), a wide U down to 92 at bar 55,
		// recovery back to 100 at bar 85, then an upside breakout.
		let pivots = [
			(0, 100.0),
			(15, 100.0),
			(35, 96.0),
			(55, 92.0),
			(75, 96.0),
			(85, 100.0),
			(95, 106.0),
		];
		let closes = series_from_pivots(&pivots, 110);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let signals = rounding_bottom(&opens, &highs, &lows, &closes, None, Some(70)).unwrap();

		assert!(signals.iter().any(|&s| s > 0.5), "no bullish signal");
		let idx = signals.iter().position(|&s| s > 0.5).unwrap();
		assert!(idx >= 80, "signal should fire after the saucer completes, got {idx}");
	}

	#[test]
	fn detects_rounding_top_breakdown() {
		// Mirror: flat baseline at 100, a wide dome up to 108 at bar 55, a
		// decline back to 100 at bar 85, then a downside breakdown.
		let pivots = [
			(0, 100.0),
			(15, 100.0),
			(35, 104.0),
			(55, 108.0),
			(75, 104.0),
			(85, 100.0),
			(95, 95.0),
		];
		let closes = series_from_pivots(&pivots, 110);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let signals = rounding_top(&opens, &highs, &lows, &closes, None, Some(70)).unwrap();

		assert!(signals.iter().any(|&s| s < -0.5), "no bearish signal");
		let idx = signals.iter().position(|&s| s < -0.5).unwrap();
		assert!(idx >= 80, "signal should fire after the rounding top completes, got {idx}");
	}
}
