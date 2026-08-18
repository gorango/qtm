use crate::utils::validation::validate_multiple_arrays;
use crate::IndicatorResult;

/// Rectangle (continuation) pattern.
///
/// Price oscillates between roughly parallel horizontal support and
/// resistance lines (both trendline slopes near zero) after a directional
/// move. The confirmed signal fires when a close breaks the rectangle in the
/// direction of the prior trend — a bullish continuation after an uptrend, a
/// bearish one after a downtrend.
pub fn rectangle(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	min_points: Option<u32>,
	slope_tolerance: Option<f64>,
	min_spread: Option<f64>,
	lookback: Option<u32>,
	trend_bars: Option<u32>,
	min_trend: Option<f64>,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let min_points = min_points.unwrap_or(3) as usize;
	let slope_tolerance = slope_tolerance.unwrap_or(0.0002);
	let min_spread = min_spread.unwrap_or(0.01);
	let lookback = lookback.unwrap_or(120) as usize;
	let trend_bars = trend_bars.unwrap_or(30) as usize;
	let min_trend = min_trend.unwrap_or(0.03);

	let mut results = vec![0.0; highs.len()];

	if highs.len() < lookback + trend_bars + 5 {
		return Ok(results);
	}

	let peaks = crate::patterns::helpers::find_peaks_internal(highs, 1);
	let troughs = crate::patterns::helpers::find_troughs_internal(lows, 1);

	if peaks.len() < min_points || troughs.len() < min_points {
		return Ok(results);
	}

	for i in (lookback + trend_bars)..highs.len() {
		let start = i - lookback;
		let trend_start = start - trend_bars;

		let win_peaks: Vec<usize> = peaks
			.iter()
			.copied()
			.filter(|&p| p > start && p < i)
			.collect();
		let win_troughs: Vec<usize> = troughs
			.iter()
			.copied()
			.filter(|&t| t > start && t < i)
			.collect();

		if win_peaks.len() < min_points || win_troughs.len() < min_points {
			continue;
		}

		let (high_line, mean_high) = fit_line(&win_peaks, highs);
		let (low_line, mean_low) = fit_line(&win_troughs, lows);

		let high_slope = high_line[0] / mean_high;
		let low_slope = low_line[0] / mean_low;

		// Both lines must be (near-)horizontal.
		if high_slope.abs() > slope_tolerance || low_slope.abs() > slope_tolerance {
			continue;
		}

		let support = low_line[1] + low_line[0] * i as f64;
		let resistance = high_line[1] + high_line[0] * i as f64;
		let spread = (resistance - support) / support;
		if spread < min_spread {
			continue;
		}

		// Prior trend direction before the rectangle.
		let prior_move = (closes[start] - closes[trend_start]) / closes[trend_start];

		if prior_move > min_trend && closes[i - 1] <= resistance && closes[i] > resistance {
			results[i] = 1.0;
		} else if prior_move < -min_trend && closes[i - 1] >= support && closes[i] < support {
			results[i] = -1.0;
		}
	}

	Ok(results)
}

fn fit_line(pivots: &[usize], prices: &[f64]) -> (Vec<f64>, f64) {
	let mut points = Vec::with_capacity(pivots.len() * 2);
	let mut mean = 0.0;
	for &p in pivots {
		points.push(p as f64);
		points.push(prices[p]);
		mean += prices[p];
	}
	mean /= pivots.len() as f64;

	(
		crate::patterns::helpers::linear_regression_internal(&points),
		mean,
	)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::patterns::helpers::test_helpers::*;

	#[test]
	fn detects_bullish_rectangle_breakout() {
		// Uptrend 80 -> 100 (bars 0-15), then a flat box between the support
		// line (troughs ~98) and a flat resistance (peaks ~102), then a
		// breakout above resistance. The box must oscillate enough to yield
		// >= 3 peaks and >= 3 troughs inside every scan window.
		let pivots = [
			(0, 80.0),
			(15, 100.0),
			(28, 98.0),
			(36, 102.0),
			(44, 98.1),
			(52, 101.8),
			(60, 98.2),
			(68, 102.0),
			(76, 98.3),
			(80, 105.0),
			(90, 109.0),
		];
		let closes = series_from_pivots(&pivots, 100);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let signals = rectangle(
			&opens,
			&highs,
			&lows,
			&closes,
			Some(3),
			None,
			None,
			Some(60),
			Some(15),
			None,
		)
		.unwrap();

		assert!(signals.iter().any(|&s| s > 0.5), "no bullish signal");
		let idx = signals.iter().position(|&s| s > 0.5).unwrap();
		assert!(idx >= 75, "signal should fire on the breakout, got {idx}");
	}

	#[test]
	fn detects_bearish_rectangle_breakdown() {
		// Mirror: downtrend 120 -> 100, then a flat box between a flat
		// resistance (~102) and the support line (troughs ~98), then a
		// breakdown below support.
		let pivots = [
			(0, 120.0),
			(15, 100.0),
			(28, 102.0),
			(36, 98.0),
			(44, 101.9),
			(52, 98.2),
			(60, 101.8),
			(68, 98.0),
			(76, 101.7),
			(80, 95.0),
			(90, 91.0),
		];
		let closes = series_from_pivots(&pivots, 100);
		let (opens, highs, lows, closes) = ohlc_from_series(&closes);

		let signals = rectangle(
			&opens,
			&highs,
			&lows,
			&closes,
			Some(3),
			None,
			None,
			Some(60),
			Some(15),
			None,
		)
		.unwrap();

		assert!(signals.iter().any(|&s| s < -0.5), "no bearish signal");
		let idx = signals.iter().position(|&s| s < -0.5).unwrap();
		assert!(idx >= 75, "signal should fire on the breakdown, got {idx}");
	}
}
