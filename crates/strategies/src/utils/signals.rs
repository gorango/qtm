/// Check if value crossed over threshold at given index
pub fn crossed_over(values: &[f64], threshold: f64, index: u32) -> bool {
	let idx = index as usize;
	if idx == 0 || values.len() <= idx {
		return false;
	}
	values[idx - 1] <= threshold && values[idx] > threshold
}

/// Check if value crossed under threshold at given index
pub fn crossed_under(values: &[f64], threshold: f64, index: u32) -> bool {
	let idx = index as usize;
	if idx == 0 || values.len() <= idx {
		return false;
	}
	values[idx - 1] >= threshold && values[idx] < threshold
}

/// Check if series A crossed over series B at given index
pub fn crossed_over_series(series_a: &[f64], series_b: &[f64], index: u32) -> bool {
	let idx = index as usize;
	if idx == 0 || series_a.len() <= idx || series_b.len() <= idx {
		return false;
	}
	series_a[idx - 1] <= series_b[idx - 1] && series_a[idx] > series_b[idx]
}

/// Check if series A crossed under series B at given index
pub fn crossed_under_series(series_a: &[f64], series_b: &[f64], index: u32) -> bool {
	let idx = index as usize;
	if idx == 0 || series_a.len() <= idx || series_b.len() <= idx {
		return false;
	}
	series_a[idx - 1] >= series_b[idx - 1] && series_a[idx] < series_b[idx]
}

pub fn consolidating(
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	i: usize,
	lookback: usize,
	threshold_pct: f64,
) -> bool {
	if i < lookback {
		return false;
	}

	let mut max_h = f64::NEG_INFINITY;
	let mut min_l = f64::INFINITY;

	for k in 0..lookback {
		let idx = i - k;
		if highs[idx] > max_h {
			max_h = highs[idx];
		}
		if lows[idx] < min_l {
			min_l = lows[idx];
		}
	}

	let range = (max_h - min_l) / closes[i];
	range <= threshold_pct
}

/// Check if value is above threshold
pub fn is_above(value: f64, threshold: f64) -> bool {
	value > threshold
}

/// Check if value is below threshold
pub fn is_below(value: f64, threshold: f64) -> bool {
	value < threshold
}

/// Check if value is between two thresholds (inclusive)
pub fn is_between(value: f64, lower: f64, upper: f64) -> bool {
	value >= lower && value <= upper
}
