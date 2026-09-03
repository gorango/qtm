use crate::utils::validation::validate_multiple_arrays;
use crate::IndicatorResult;

/// Break of Structure (BOS) / Change of Character (CHoCH).
///
/// Structural, price-only detector. No timestamps — session alignment is
/// caller-side (filter returned indices by session if you want ICT-faithful
/// behaviour). Two modes:
///
/// * `horizontal` (default): `close` crosses the last confirmed swing high/low
///   (via `find_peaks`/`find_troughs` with `lookaround`). Classic SMC.
/// * `trendline`: `close` crosses the extrapolated line through the last
///   `trendlinePoints` swing highs (resistance) or lows (support). Covers the
///   "line across last few peaks" interpretation.
/// * `either`: fires if either condition holds.
///
/// Returns per bar: `0` none, `1` bull BOS (continuation), `-1` bear BOS,
/// `2` bull CHoCH (break against prior trend), `-2` bear CHoCH.
pub fn break_of_structure(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	lookaround: Option<u32>,
	mode: Option<String>,
	trendline_points: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[highs, lows, closes])?;

	let lookaround = lookaround.unwrap_or(2) as usize;
	let mode = mode
		.unwrap_or_else(|| "horizontal".to_string())
		.to_lowercase();
	let trendline_points = trendline_points.unwrap_or(3) as usize;

	let mut results = vec![0.0; highs.len()];

	if highs.len() < 2 * lookaround + 5 {
		return Ok(results);
	}
	if trendline_points < 2 {
		return Ok(results);
	}

	let peaks = crate::patterns::helpers::find_peaks_internal(highs, lookaround);
	let troughs = crate::patterns::helpers::find_troughs_internal(lows, lookaround);

	// Need at least one confirmed swing before first evaluable bar.
	for i in (lookaround + 1)..highs.len() {
		// Confirmed means peak/trough + lookaround bars have elapsed.
		let last_peak = peaks.iter().rev().find(|&&p| p + lookaround < i).copied();
		let last_trough = troughs.iter().rev().find(|&&t| t + lookaround < i).copied();

		let mut bull_h = false;
		let mut bear_h = false;

		if let Some(p) = last_peak {
			let level = highs[p];
			if closes[i] > level && closes[i - 1] <= level {
				bull_h = true;
			}
		}
		if let Some(t) = last_trough {
			let level = lows[t];
			if closes[i] < level && closes[i - 1] >= level {
				bear_h = true;
			}
		}

		let mut bull_t = false;
		let mut bear_t = false;

		if mode == "trendline" || mode == "either" {
			// Collect N most recent confirmed peaks/troughs before i.
			let recent_peaks: Vec<usize> = peaks
				.iter()
				.copied()
				.filter(|&p| p + lookaround < i)
				.rev()
				.take(trendline_points)
				.collect();
			if recent_peaks.len() >= trendline_points {
				// Need at least 2 points to define a line; regression handles N.
				let (line, _) = fit_line(&recent_peaks, highs);
				let level = line[0] * i as f64 + line[1];
				if closes[i] > level && closes[i - 1] <= level {
					// Only count if this is above the last swing high as well
					// when in either mode we already have bull_h; standalone
					// trendline mode doesn't require it.
					bull_t = true;
				}
			}

			let recent_troughs: Vec<usize> = troughs
				.iter()
				.copied()
				.filter(|&t| t + lookaround < i)
				.rev()
				.take(trendline_points)
				.collect();
			if recent_troughs.len() >= trendline_points {
				let (line, _) = fit_line(&recent_troughs, lows);
				let level = line[0] * i as f64 + line[1];
				if closes[i] < level && closes[i - 1] >= level {
					bear_t = true;
				}
			}
		}

		let do_bull = match mode.as_str() {
			"horizontal" => bull_h,
			"trendline" => bull_t,
			"either" => bull_h || bull_t,
			_ => bull_h,
		};
		let do_bear = match mode.as_str() {
			"horizontal" => bear_h,
			"trendline" => bear_t,
			"either" => bear_h || bear_t,
			_ => bear_h,
		};

		// Don't emit both directions on the same bar; prefer the stronger
		// horizontal signal when in either mode (they can coincide).
		if do_bull && !do_bear {
			let choch = is_downtrend(&peaks, &troughs, highs, lows, i, lookaround);
			results[i] = if choch { 2.0 } else { 1.0 };
		} else if do_bear && !do_bull {
			let choch = is_uptrend(&peaks, &troughs, highs, lows, i, lookaround);
			results[i] = if choch { -2.0 } else { -1.0 };
		} else if do_bull && do_bear {
			// Rare if both resistance and support break same bar (e.g. huge
			// range expansion). Keep neutral to avoid flip-flop.
			continue;
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

fn is_uptrend(
	peaks: &[usize],
	troughs: &[usize],
	highs: &[f64],
	lows: &[f64],
	i: usize,
	lookaround: usize,
) -> bool {
	// Need at least 2 confirmed peaks and 2 troughs to define trend.
	let conf_peaks: Vec<usize> = peaks
		.iter()
		.copied()
		.filter(|&p| p + lookaround < i)
		.collect();
	let conf_troughs: Vec<usize> = troughs
		.iter()
		.copied()
		.filter(|&t| t + lookaround < i)
		.collect();
	if conf_peaks.len() < 2 || conf_troughs.len() < 2 {
		return false;
	}
	let p1 = conf_peaks[conf_peaks.len() - 1];
	let p2 = conf_peaks[conf_peaks.len() - 2];
	let t1 = conf_troughs[conf_troughs.len() - 1];
	let t2 = conf_troughs[conf_troughs.len() - 2];
	highs[p1] > highs[p2] && lows[t1] > lows[t2]
}

fn is_downtrend(
	peaks: &[usize],
	troughs: &[usize],
	highs: &[f64],
	lows: &[f64],
	i: usize,
	lookaround: usize,
) -> bool {
	let conf_peaks: Vec<usize> = peaks
		.iter()
		.copied()
		.filter(|&p| p + lookaround < i)
		.collect();
	let conf_troughs: Vec<usize> = troughs
		.iter()
		.copied()
		.filter(|&t| t + lookaround < i)
		.collect();
	if conf_peaks.len() < 2 || conf_troughs.len() < 2 {
		return false;
	}
	let p1 = conf_peaks[conf_peaks.len() - 1];
	let p2 = conf_peaks[conf_peaks.len() - 2];
	let t1 = conf_troughs[conf_troughs.len() - 1];
	let t2 = conf_troughs[conf_troughs.len() - 2];
	highs[p1] < highs[p2] && lows[t1] < lows[t2]
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::patterns::helpers::test_helpers::*;

	#[test]
	fn detects_horizontal_bull_bos() {
		// Descending highs 105 -> 102 -> 100 prevent early horizontal BOS;
		// final push to 104 breaks the last swing high (100) at bar 35.
		let pivots = [
			(0, 90.0),
			(10, 105.0),
			(15, 95.0),
			(20, 102.0),
			(25, 96.0),
			(30, 100.0),
			(35, 104.0),
			(45, 106.0),
		];
		let closes = series_from_pivots(&pivots, 60);
		let (_opens, highs, lows, closes) = ohlc_from_series(&closes);

		let sigs = break_of_structure(
			&highs,
			&lows,
			&closes,
			Some(2),
			Some("horizontal".into()),
			None,
		)
		.unwrap();
		assert!(sigs.contains(&1.0), "no bull BOS, got {sigs:?}");
		let idx = sigs.iter().position(|&s| s == 1.0).unwrap();
		assert!(idx >= 30, "BOS fired too early at {idx}");
	}

	#[test]
	fn detects_horizontal_bear_bos() {
		let pivots = [
			(0, 110.0),
			(10, 105.0),
			(15, 108.0),
			(25, 100.0),
			(30, 102.0),
			(35, 98.0),
			(45, 96.0),
		];
		let closes = series_from_pivots(&pivots, 60);
		let (_opens, highs, lows, closes) = ohlc_from_series(&closes);
		let sigs = break_of_structure(
			&highs,
			&lows,
			&closes,
			Some(2),
			Some("horizontal".into()),
			None,
		)
		.unwrap();
		assert!(sigs.contains(&-1.0), "no bear BOS");
	}

	#[test]
	fn detects_choch_when_against_trend() {
		// Downtrend: lower highs 102 -> 100 -> 98, lower lows 95 -> 93
		// Then bullish break above last high (98) against downtrend => CHoCH (2)
		let pivots = [
			(0, 102.0),
			(5, 95.0),
			(10, 100.0),
			(15, 93.0),
			(20, 98.0),
			(25, 91.0),
			(30, 100.5),
			(40, 103.0),
		];
		let closes = series_from_pivots(&pivots, 60);
		let (_opens, highs, lows, closes) = ohlc_from_series(&closes);
		let sigs = break_of_structure(
			&highs,
			&lows,
			&closes,
			Some(2),
			Some("horizontal".into()),
			None,
		)
		.unwrap();
		assert!(sigs.contains(&2.0), "expected bull CHoCH (2), got {sigs:?}");
	}

	#[test]
	fn detects_trendline_bos() {
		// Rising resistance line through peaks (10,100) (20,102) (30,104) slope 0.2
		// At bar 35, line ≈ 105, close breaks to 106.5 => trendline BOS
		let pivots = [
			(0, 95.0),
			(10, 100.0),
			(15, 97.0),
			(20, 102.0),
			(25, 99.0),
			(30, 104.0),
			(34, 102.0),
			(35, 106.5),
			(45, 108.0),
		];
		let closes = series_from_pivots(&pivots, 60);
		let (_opens, highs, lows, closes) = ohlc_from_series(&closes);
		let sigs = break_of_structure(
			&highs,
			&lows,
			&closes,
			Some(2),
			Some("trendline".into()),
			Some(3),
		)
		.unwrap();
		assert!(
			sigs.contains(&1.0) || sigs.contains(&2.0),
			"no trendline BOS, {sigs:?}"
		);
	}

	#[test]
	fn no_signal_on_insufficient_data() {
		let highs = vec![100.0; 4];
		let lows = vec![99.0; 4];
		let closes = vec![99.5; 4];
		let sigs = break_of_structure(&highs, &lows, &closes, None, None, None).unwrap();
		assert!(sigs.iter().all(|&s| s == 0.0));
	}
}
