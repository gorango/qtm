//! Synthetic-data builders shared by pattern strategy tests.
//!
//! Tests hand-craft OHLC arrays that trace a known swing structure, then
//! assert the strategy fires at the expected bar with the expected direction.
//! Mirrors the helpers in `indicators-core` so strategy tests can reuse the
//! same verified scenarios. Kept behind `#[cfg(test)]` so it never leaks into
//! the public API.

/// Linearly interpolates between `(index, price)` pivot points so a sparse
/// swing list becomes a dense bar-by-bar close series.
pub fn series_from_pivots(pivots: &[(usize, f64)], len: usize) -> Vec<f64> {
	let mut series = vec![0.0; len];
	if pivots.is_empty() {
		return series;
	}

	for w in pivots.windows(2) {
		let (i0, y0) = w[0];
		let (i1, y1) = w[1];
		let span = i1 - i0;
		series[i0..i1.min(len)]
			.iter_mut()
			.enumerate()
			.for_each(|(j, s)| {
				let t = j as f64 / span as f64;
				*s = y0 + (y1 - y0) * t;
			});
	}

	if let Some(&(last, y)) = pivots.last() {
		series[last..len].fill(y);
	}

	series
}

/// Converts a dense close series into OHLC arrays.
///
/// Opens lag closes by one bar. Highs and lows are derived directly from the
/// close (a scaled copy) so any local extremum in the close series maps 1:1
/// to a strict extremum in the high/low series at the same bar — which keeps
/// pivot detection deterministic in tests.
pub fn ohlc_from_series(closes: &[f64]) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
	let mut opens = closes.to_vec();
	if opens.len() > 1 {
		for i in (1..opens.len()).rev() {
			opens[i] = closes[i - 1];
		}
	}

	let mut highs = Vec::with_capacity(closes.len());
	let mut lows = Vec::with_capacity(closes.len());
	for &c in closes {
		highs.push(c * 1.001);
		lows.push(c * 0.999);
	}

	(opens, highs, lows, closes.to_vec())
}

/// Builds a bar from explicit open/high/low/close.
pub fn bar(o: f64, h: f64, l: f64, c: f64) -> [f64; 4] {
	[o, h, l, c]
}

/// Converts `(open, high, low, close)` tuples into the four arrays.
pub fn ohlc(bars: &[[f64; 4]]) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
	let mut opens = Vec::with_capacity(bars.len());
	let mut highs = Vec::with_capacity(bars.len());
	let mut lows = Vec::with_capacity(bars.len());
	let mut closes = Vec::with_capacity(bars.len());
	for b in bars {
		opens.push(b[0]);
		highs.push(b[1]);
		lows.push(b[2]);
		closes.push(b[3]);
	}
	(opens, highs, lows, closes)
}
