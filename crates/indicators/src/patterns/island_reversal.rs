use crate::utils::validation::validate_multiple_arrays;
use crate::IndicatorResult;

/// Island reversal.
///
/// Price gaps away, trades in a small isolated cluster (the "island"), then
/// gaps back over the original level — stranding the island. A bullish island
/// is a down-gap followed by an up-gap that lifts price back above the
/// pre-island high; a bearish island is the mirror (up-gap then down-gap
/// below the pre-island low).
pub fn island_reversal(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	min_island_bars: Option<u32>,
	max_island_bars: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let min_island_bars = min_island_bars.unwrap_or(2) as usize;
	let max_island_bars = max_island_bars.unwrap_or(15) as usize;

	let mut results = vec![0.0; highs.len()];

	if highs.len() < 3 {
		return Ok(results);
	}

	// Track the most recent gap-down / gap-up bar and the level it started from.
	let mut last_gap_down: Option<(usize, f64)> = None; // (bar, level before the gap)
	let mut last_gap_up: Option<(usize, f64)> = None;

	for i in 1..highs.len() {
		let gap_down = highs[i] < lows[i - 1];
		let gap_up = lows[i] > highs[i - 1];

		if gap_down {
			// Complete a bearish island if a prior up-gap exists within range.
			if let Some((g1, level)) = last_gap_up {
				let island_bars = i - g1 - 1;
				if island_bars >= min_island_bars
					&& island_bars <= max_island_bars
					&& highs[i] < level
				{
					results[i] = -1.0;
				}
			}
			last_gap_down = Some((i, highs[i - 1]));
		}

		if gap_up {
			// Complete a bullish island if a prior down-gap exists within range.
			if let Some((g1, level)) = last_gap_down {
				let island_bars = i - g1 - 1;
				if island_bars >= min_island_bars
					&& island_bars <= max_island_bars
					&& lows[i] > level
				{
					results[i] = 1.0;
				}
			}
			last_gap_up = Some((i, lows[i - 1]));
		}
	}

	Ok(results)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::patterns::helpers::test_helpers::*;

	#[test]
	fn detects_bullish_island_reversal() {
		let mut bars: Vec<[f64; 4]> = Vec::new();
		// Flat baseline around 100.
		for _ in 0..10 {
			bars.push(bar(100.0, 100.5, 99.5, 100.0));
		}
		bars.push(bar(99.8, 100.2, 99.4, 99.9)); // pre-gap bar
		bars.push(bar(95.0, 95.8, 93.5, 94.2)); // gap down
		bars.push(bar(94.0, 94.6, 93.2, 94.1)); // island
		bars.push(bar(94.3, 94.8, 93.6, 94.0)); // island
		bars.push(bar(94.2, 95.2, 93.8, 95.0)); // island
		bars.push(bar(101.5, 102.5, 101.0, 102.0)); // gap up back over baseline
		bars.push(bar(102.0, 102.8, 101.5, 102.6));

		let (opens, highs, lows, closes) = ohlc(&bars);
		let signals = island_reversal(&opens, &highs, &lows, &closes, Some(3), Some(10)).unwrap();

		let idx = signals
			.iter()
			.position(|&s| s > 0.5)
			.expect("no bullish signal");
		assert_eq!(idx, 15, "signal should fire on the covering gap-up");
	}

	#[test]
	fn detects_bearish_island_reversal() {
		let mut bars: Vec<[f64; 4]> = Vec::new();
		for _ in 0..10 {
			bars.push(bar(100.0, 100.5, 99.5, 100.0));
		}
		bars.push(bar(100.2, 100.6, 99.8, 100.1)); // pre-gap bar
		bars.push(bar(105.0, 105.8, 104.5, 105.2)); // gap up
		bars.push(bar(104.4, 105.4, 104.2, 105.1)); // island
		bars.push(bar(104.7, 105.2, 104.0, 104.5)); // island
		bars.push(bar(104.5, 105.0, 104.3, 104.6)); // island
		bars.push(bar(98.5, 99.0, 98.0, 98.4)); // gap down back below baseline
		bars.push(bar(98.2, 98.8, 97.8, 98.0));

		let (opens, highs, lows, closes) = ohlc(&bars);
		let signals = island_reversal(&opens, &highs, &lows, &closes, Some(3), Some(10)).unwrap();

		let idx = signals
			.iter()
			.position(|&s| s < -0.5)
			.expect("no bearish signal");
		assert_eq!(idx, 15, "signal should fire on the covering gap-down");
	}
}
