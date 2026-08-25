pub fn find_peaks(values: &[f64], lookaround: u32) -> Vec<u32> {
	find_peaks_internal(values, lookaround as usize)
		.iter()
		.map(|&x| x as u32)
		.collect()
}

pub fn find_troughs(values: &[f64], lookaround: u32) -> Vec<u32> {
	find_troughs_internal(values, lookaround as usize)
		.iter()
		.map(|&x| x as u32)
		.collect()
}

pub fn linear_regression(points: Vec<f64>) -> Vec<f64> {
	linear_regression_internal(&points)
}

pub fn quadratic_regression(points: Vec<f64>) -> Vec<f64> {
	quadratic_regression_internal(&points)
}

pub fn find_peaks_internal(values: &[f64], lookaround: usize) -> Vec<usize> {
	let mut peaks = Vec::new();

	if values.len() < 2 * lookaround + 1 {
		return peaks;
	}

	for i in lookaround..values.len() - lookaround {
		let current_value = values[i];
		let mut is_peak = true;

		for j in 1..=lookaround {
			if values[i - j] >= current_value || values[i + j] >= current_value {
				is_peak = false;
				break;
			}
		}

		if is_peak {
			peaks.push(i);
		}
	}

	peaks
}

pub fn find_troughs_internal(values: &[f64], lookaround: usize) -> Vec<usize> {
	let mut troughs = Vec::new();

	if values.len() < 2 * lookaround + 1 {
		return troughs;
	}

	for i in lookaround..values.len() - lookaround {
		let current_value = values[i];
		let mut is_trough = true;

		for j in 1..=lookaround {
			if values[i - j] <= current_value || values[i + j] <= current_value {
				is_trough = false;
				break;
			}
		}

		if is_trough {
			troughs.push(i);
		}
	}

	troughs
}

pub fn linear_regression_internal(points: &[f64]) -> Vec<f64> {
	if points.len() < 4 || !points.len().is_multiple_of(2) {
		return vec![0.0, 0.0];
	}

	let n = points.len() / 2;
	let mut sum_x = 0.0;
	let mut sum_y = 0.0;
	let mut sum_xy = 0.0;
	let mut sum_xx = 0.0;

	for i in 0..n {
		let x = points[2 * i];
		let y = points[2 * i + 1];
		sum_x += x;
		sum_y += y;
		sum_xy += x * y;
		sum_xx += x * x;
	}

	let n_f64 = n as f64;
	let denominator = n_f64 * sum_xx - sum_x * sum_x;

	let slope = if denominator.abs() > 1e-10 {
		(n_f64 * sum_xy - sum_x * sum_y) / denominator
	} else {
		0.0
	};

	let intercept = (sum_y - slope * sum_x) / n_f64;

	vec![slope, intercept]
}

/// Fits a parabola `y = a*x^2 + b*x + c` to `(x, y)` pairs via least squares.
///
/// Input is the same interleaved `[x0, y0, x1, y1, ...]` layout used by
/// [`linear_regression_internal`]. Returns `[a, b, c]`; on degenerate input or
/// a singular normal matrix it returns `[0.0, 0.0, 0.0]`.
pub fn quadratic_regression_internal(points: &[f64]) -> Vec<f64> {
	if points.len() < 6 || !points.len().is_multiple_of(2) {
		return vec![0.0, 0.0, 0.0];
	}

	let n = points.len() / 2;
	let mut sx = 0.0;
	let mut sy = 0.0;
	let mut sxx = 0.0;
	let mut sxy = 0.0;
	let mut sxxx = 0.0;
	let mut sxxy = 0.0;
	let mut sxxxx = 0.0;

	for i in 0..n {
		let x = points[2 * i];
		let y = points[2 * i + 1];
		sx += x;
		sy += y;
		sxx += x * x;
		sxy += x * y;
		sxxx += x * x * x;
		sxxy += x * x * y;
		sxxxx += x * x * x * x;
	}

	// Normal equations for [c, b, a]: y = a*x^2 + b*x + c
	//   [[n, sx, sxx]    [c]   [sy]
	//    [sx, sxx, sxxx] [b] = [sxy]
	//    [sxx, sxxx, sxxxx] [a]   [sxxy]]
	let mut m = [[n as f64, sx, sxx], [sx, sxx, sxxx], [sxx, sxxx, sxxxx]];
	let mut v = [sy, sxy, sxxy];

	// Gaussian elimination with partial pivoting.
	for col in 0..3 {
		let mut best = col;
		for r in (col + 1)..3 {
			if m[r][col].abs() > m[best][col].abs() {
				best = r;
			}
		}
		m.swap(col, best);
		v.swap(col, best);

		let pivot = m[col][col];
		if pivot.abs() < 1e-12 {
			return vec![0.0, 0.0, 0.0];
		}

		for r in (col + 1)..3 {
			let factor = m[r][col] / pivot;
			for c in col..3 {
				m[r][c] -= factor * m[col][c];
			}
			v[r] -= factor * v[col];
		}
	}

	let mut solution = [0.0; 3];
	for r in (0..3).rev() {
		let mut acc = v[r];
		for (c, &s) in solution.iter().enumerate().skip(r + 1) {
			acc -= m[r][c] * s;
		}
		solution[r] = acc / m[r][r];
	}

	// solution = [c, b, a]
	vec![solution[2], solution[1], solution[0]]
}

pub fn zig_zag_filter(values: &[f64], deviation: f64) -> Vec<f64> {
	zig_zag_filter_internal(values, deviation)
}

pub fn zig_zag_filter_internal(values: &[f64], deviation: f64) -> Vec<f64> {
	if values.is_empty() {
		return Vec::new();
	}

	let mut filtered = vec![0.0; values.len()];
	filtered[0] = values[0];

	let mut last_pivot = 0;

	for i in 1..values.len() {
		let change = ((values[i] - values[last_pivot]) / values[last_pivot]).abs();
		if change >= deviation {
			filtered[i] = values[i];
			last_pivot = i;
		} else {
			filtered[i] = values[last_pivot];
		}
	}

	filtered
}

/// Synthetic-data builders shared by pattern tests.
///
/// Tests hand-craft OHLC arrays that trace a known swing structure, then
/// assert the detector fires at the expected bar. Kept behind `#[cfg(test)]`
/// so it never leaks into the public API.
#[cfg(test)]
pub(crate) mod test_helpers {
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
			for (i, slot) in series.iter_mut().enumerate().take(i1.min(len)).skip(i0) {
				let t = (i - i0) as f64 / span as f64;
				*slot = y0 + (y1 - y0) * t;
			}
		}

		if let Some(&(last, y)) = pivots.last() {
			for slot in series.iter_mut().take(len).skip(last) {
				*slot = y;
			}
		}

		series
	}

	/// Converts a dense close series into OHLC arrays.
	///
	/// Opens lag closes by one bar. Highs and lows are derived directly from
	/// the close (a scaled copy plus a tiny per-bar wiggle) rather than from
	/// the open/close body, so any local extremum in the close series maps 1:1
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
			// No per-bar wiggle: plateaus in the close series become plateaus in
			// the highs/lows (which strict pivot search ignores), so pivots are
			// found exactly where the swing structure says they should be.
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
}
