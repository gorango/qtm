use crate::utils::validation::validate_multiple_arrays;
use crate::IndicatorResult;

// ── Candle primitives ────────────────────────────

fn body(o: f64, c: f64) -> f64 {
	(c - o).abs()
}

fn full_range(h: f64, l: f64) -> f64 {
	h - l
}

fn upper_shadow(h: f64, o: f64, c: f64) -> f64 {
	h - o.max(c)
}

fn lower_shadow(l: f64, o: f64, c: f64) -> f64 {
	o.min(c) - l
}

fn is_bull(o: f64, c: f64) -> bool {
	c > o
}

fn is_bear(o: f64, c: f64) -> bool {
	c < o
}

/// Small-bodied candle (doji-like): body is at most `threshold` of the range.
fn is_small_body(o: f64, h: f64, l: f64, c: f64, threshold: f64) -> bool {
	let r = full_range(h, l);
	r > 0.0 && body(o, c) / r <= threshold
}

/// Strong candle: closes in the direction of the move with a body of at least
/// `min_body_ratio` of the range.
fn is_strong_bull(o: f64, h: f64, l: f64, c: f64, min_body_ratio: f64) -> bool {
	is_bull(o, c) && full_range(h, l) > 0.0 && body(o, c) / full_range(h, l) >= min_body_ratio
}

fn is_strong_bear(o: f64, h: f64, l: f64, c: f64, min_body_ratio: f64) -> bool {
	is_bear(o, c) && full_range(h, l) > 0.0 && body(o, c) / full_range(h, l) >= min_body_ratio
}

/// Trend context over `trend_bars` bars ending at `i - 1`.
fn is_downtrend(closes: &[f64], i: usize, trend_bars: usize) -> bool {
	i > trend_bars && closes[i - 1] < closes[i - trend_bars - 1]
}

fn is_uptrend(closes: &[f64], i: usize, trend_bars: usize) -> bool {
	i > trend_bars && closes[i - 1] > closes[i - trend_bars - 1]
}

// ── Single-candle patterns ───────────────────────

/// Hammer: small body near the top of the range with a long lower shadow,
/// appearing after a decline. Bullish.
pub fn hammer(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	body_ratio: Option<f64>,
	shadow_multiplier: Option<f64>,
	trend_bars: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	single_candle_signal(
		opens,
		highs,
		lows,
		closes,
		body_ratio,
		shadow_multiplier,
		trend_bars,
		true,
		true,
	)
}

/// Inverted hammer: small body near the bottom with a long upper shadow after
/// a decline. Bullish.
pub fn inverted_hammer(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	body_ratio: Option<f64>,
	shadow_multiplier: Option<f64>,
	trend_bars: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	single_candle_signal(
		opens,
		highs,
		lows,
		closes,
		body_ratio,
		shadow_multiplier,
		trend_bars,
		true,
		false,
	)
}

/// Hanging man: hammer shape (small body, long lower shadow) appearing after
/// an advance. Bearish.
pub fn hanging_man(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	body_ratio: Option<f64>,
	shadow_multiplier: Option<f64>,
	trend_bars: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	single_candle_signal(
		opens,
		highs,
		lows,
		closes,
		body_ratio,
		shadow_multiplier,
		trend_bars,
		false,
		true,
	)
}

/// Shooting star: inverted-hammer shape appearing after an advance. Bearish.
pub fn shooting_star(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	body_ratio: Option<f64>,
	shadow_multiplier: Option<f64>,
	trend_bars: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	single_candle_signal(
		opens,
		highs,
		lows,
		closes,
		body_ratio,
		shadow_multiplier,
		trend_bars,
		false,
		false,
	)
}

/// Spinning top: tiny body with shadows on both sides; signals indecision.
/// Direction follows the prior trend.
pub fn spinning_top(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	body_ratio: Option<f64>,
	trend_bars: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let body_ratio = body_ratio.unwrap_or(0.3);
	let trend_bars = trend_bars.unwrap_or(5) as usize;

	let mut results = vec![0.0; opens.len()];
	for i in 1..opens.len() {
		if !is_small_body(opens[i], highs[i], lows[i], closes[i], body_ratio) {
			continue;
		}
		if upper_shadow(highs[i], opens[i], closes[i]) < body(opens[i], closes[i])
			|| lower_shadow(lows[i], opens[i], closes[i]) < body(opens[i], closes[i])
		{
			continue;
		}
		if is_downtrend(closes, i, trend_bars) {
			results[i] = 1.0;
		} else if is_uptrend(closes, i, trend_bars) {
			results[i] = -1.0;
		}
	}
	Ok(results)
}

/// Doji with long shadows on both sides; signals a potential reversal.
pub fn long_legged_doji(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	body_ratio: Option<f64>,
	shadow_multiplier: Option<f64>,
	trend_bars: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let body_ratio = body_ratio.unwrap_or(0.1);
	let shadow_multiplier = shadow_multiplier.unwrap_or(2.0);
	let trend_bars = trend_bars.unwrap_or(5) as usize;

	let mut results = vec![0.0; opens.len()];
	for i in 1..opens.len() {
		if !is_small_body(opens[i], highs[i], lows[i], closes[i], body_ratio) {
			continue;
		}
		let b = body(opens[i], closes[i]);
		if upper_shadow(highs[i], opens[i], closes[i]) < shadow_multiplier * b
			|| lower_shadow(lows[i], opens[i], closes[i]) < shadow_multiplier * b
		{
			continue;
		}
		if is_downtrend(closes, i, trend_bars) {
			results[i] = 1.0;
		} else if is_uptrend(closes, i, trend_bars) {
			results[i] = -1.0;
		}
	}
	Ok(results)
}

/// Dragonfly doji: doji with a long lower shadow and negligible upper shadow.
/// Bullish after a decline.
pub fn dragonfly_doji(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	body_ratio: Option<f64>,
	shadow_multiplier: Option<f64>,
	trend_bars: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let body_ratio = body_ratio.unwrap_or(0.1);
	let shadow_multiplier = shadow_multiplier.unwrap_or(2.0);
	let trend_bars = trend_bars.unwrap_or(5) as usize;

	let mut results = vec![0.0; opens.len()];
	for i in 1..opens.len() {
		if !is_small_body(opens[i], highs[i], lows[i], closes[i], body_ratio) {
			continue;
		}
		let b = body(opens[i], closes[i]);
		if lower_shadow(lows[i], opens[i], closes[i]) < shadow_multiplier * b
			|| upper_shadow(highs[i], opens[i], closes[i]) > b
		{
			continue;
		}
		if is_downtrend(closes, i, trend_bars) {
			results[i] = 1.0;
		}
	}
	Ok(results)
}

/// Gravestone doji: doji with a long upper shadow and negligible lower shadow.
/// Bearish after an advance.
pub fn gravestone_doji(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	body_ratio: Option<f64>,
	shadow_multiplier: Option<f64>,
	trend_bars: Option<u32>,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let body_ratio = body_ratio.unwrap_or(0.1);
	let shadow_multiplier = shadow_multiplier.unwrap_or(2.0);
	let trend_bars = trend_bars.unwrap_or(5) as usize;

	let mut results = vec![0.0; opens.len()];
	for i in 1..opens.len() {
		if !is_small_body(opens[i], highs[i], lows[i], closes[i], body_ratio) {
			continue;
		}
		let b = body(opens[i], closes[i]);
		if upper_shadow(highs[i], opens[i], closes[i]) < shadow_multiplier * b
			|| lower_shadow(lows[i], opens[i], closes[i]) > b
		{
			continue;
		}
		if is_uptrend(closes, i, trend_bars) {
			results[i] = -1.0;
		}
	}
	Ok(results)
}

/// Shared single-candle detector.
///
/// `bullish` selects the prior-trend context (downtrend for bullish patterns,
/// uptrend for bearish). `lower_shadow` selects which shadow is the long one
/// (`true` = hammer/shooting-star family, `false` = inverted-hammer family).
fn single_candle_signal(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	body_ratio: Option<f64>,
	shadow_multiplier: Option<f64>,
	trend_bars: Option<u32>,
	bullish: bool,
	long_lower_shadow: bool,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let body_ratio = body_ratio.unwrap_or(0.3);
	let shadow_multiplier = shadow_multiplier.unwrap_or(2.0);
	let trend_bars = trend_bars.unwrap_or(5) as usize;

	let mut results = vec![0.0; opens.len()];
	for i in 1..opens.len() {
		if !is_small_body(opens[i], highs[i], lows[i], closes[i], body_ratio) {
			continue;
		}
		let b = body(opens[i], closes[i]);
		let long_shadow = if long_lower_shadow {
			lower_shadow(lows[i], opens[i], closes[i])
		} else {
			upper_shadow(highs[i], opens[i], closes[i])
		};
		let short_shadow = if long_lower_shadow {
			upper_shadow(highs[i], opens[i], closes[i])
		} else {
			lower_shadow(lows[i], opens[i], closes[i])
		};
		if long_shadow < shadow_multiplier * b || short_shadow > b {
			continue;
		}

		if bullish && is_downtrend(closes, i, trend_bars) {
			results[i] = 1.0;
		} else if !bullish && is_uptrend(closes, i, trend_bars) {
			results[i] = -1.0;
		}
	}
	Ok(results)
}

// ── Two-candle patterns ──────────────────────────

/// Bullish harami: a small bullish candle fully inside the body of a prior
/// bearish candle.
pub fn bullish_harami(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	body_ratio: Option<f64>,
) -> IndicatorResult<Vec<f64>> {
	harami_internal(opens, highs, lows, closes, body_ratio, true)
}

/// Bearish harami: a small bearish candle fully inside the body of a prior
/// bullish candle.
pub fn bearish_harami(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	body_ratio: Option<f64>,
) -> IndicatorResult<Vec<f64>> {
	harami_internal(opens, highs, lows, closes, body_ratio, false)
}

fn harami_internal(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	body_ratio: Option<f64>,
	bullish: bool,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let body_ratio = body_ratio.unwrap_or(0.5);

	let mut results = vec![0.0; opens.len()];
	for i in 1..opens.len() {
		let prev_open = opens[i - 1];
		let prev_close = closes[i - 1];
		let curr_open = opens[i];
		let curr_close = closes[i];

		let prev_body = body(prev_open, prev_close);
		if prev_body <= 0.0 {
			continue;
		}

		let valid = if bullish {
			is_bear(prev_open, prev_close)
				&& is_bull(curr_open, curr_close)
				&& curr_open >= prev_close
				&& curr_close <= prev_open
		} else {
			is_bull(prev_open, prev_close)
				&& is_bear(curr_open, curr_close)
				&& curr_open <= prev_close
				&& curr_close >= prev_open
		};

		if valid && body(curr_open, curr_close) <= prev_body * body_ratio {
			results[i] = if bullish { 1.0 } else { -1.0 };
		}
	}
	Ok(results)
}

/// Piercing line: a strong bullish candle that opens below the prior low and
/// closes above the midpoint of the prior bearish body. Bullish.
pub fn piercing_line(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
) -> IndicatorResult<Vec<f64>> {
	piercing_internal(opens, highs, lows, closes, true)
}

/// Dark cloud cover: a strong bearish candle that opens above the prior high
/// and closes below the midpoint of the prior bullish body. Bearish.
pub fn dark_cloud_cover(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
) -> IndicatorResult<Vec<f64>> {
	piercing_internal(opens, highs, lows, closes, false)
}

fn piercing_internal(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	bullish: bool,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let mut results = vec![0.0; opens.len()];
	for i in 1..opens.len() {
		let prev_open = opens[i - 1];
		let prev_close = closes[i - 1];
		let curr_open = opens[i];
		let curr_close = closes[i];

		let midpoint = (prev_open + prev_close) / 2.0;

		let valid = if bullish {
			is_bear(prev_open, prev_close)
				&& curr_open < lows[i - 1]
				&& curr_close > midpoint
				&& curr_close < prev_open
		} else {
			is_bull(prev_open, prev_close)
				&& curr_open > highs[i - 1]
				&& curr_close < midpoint
				&& curr_close > prev_open
		};

		if valid {
			results[i] = if bullish { 1.0 } else { -1.0 };
		}
	}
	Ok(results)
}

/// Tweezer bottom: two candles with (near-)equal lows after a decline.
/// Bullish.
pub fn tweezer_bottom(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	shadow_tolerance: Option<f64>,
) -> IndicatorResult<Vec<f64>> {
	tweezer_internal(opens, highs, lows, closes, shadow_tolerance, true)
}

/// Tweezer top: two candles with (near-)equal highs after an advance. Bearish.
pub fn tweezer_top(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	shadow_tolerance: Option<f64>,
) -> IndicatorResult<Vec<f64>> {
	tweezer_internal(opens, highs, lows, closes, shadow_tolerance, false)
}

fn tweezer_internal(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	shadow_tolerance: Option<f64>,
	bullish: bool,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let shadow_tolerance = shadow_tolerance.unwrap_or(0.001);

	let mut results = vec![0.0; opens.len()];
	for i in 2..opens.len() {
		let level = if bullish {
			(lows[i] - lows[i - 1]).abs() / lows[i].max(1e-9)
		} else {
			(highs[i] - highs[i - 1]).abs() / highs[i].max(1e-9)
		};

		if level > shadow_tolerance {
			continue;
		}

		let prior_direction = if bullish {
			is_downtrend(closes, i, 3)
		} else {
			is_uptrend(closes, i, 3)
		};

		if prior_direction {
			results[i] = if bullish { 1.0 } else { -1.0 };
		}
	}
	Ok(results)
}

// ── Three-candle patterns ────────────────────────

/// Three white soldiers: three consecutive strong bullish candles with rising
/// closes, each opening within the prior body. Bullish.
pub fn three_white_soldiers(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	min_body_ratio: Option<f64>,
) -> IndicatorResult<Vec<f64>> {
	soldiers_internal(opens, highs, lows, closes, min_body_ratio, true)
}

/// Three black crows: three consecutive strong bearish candles with falling
/// closes. Bearish.
pub fn three_black_crows(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	min_body_ratio: Option<f64>,
) -> IndicatorResult<Vec<f64>> {
	soldiers_internal(opens, highs, lows, closes, min_body_ratio, false)
}

fn soldiers_internal(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	min_body_ratio: Option<f64>,
	bullish: bool,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let min_body_ratio = min_body_ratio.unwrap_or(0.5);

	let mut results = vec![0.0; opens.len()];
	for (i, out) in results.iter_mut().enumerate().take(opens.len()).skip(2) {
		let mut valid = true;
		for k in 0..3 {
			let idx = i - 2 + k;
			let strong = if bullish {
				is_strong_bull(
					opens[idx],
					highs[idx],
					lows[idx],
					closes[idx],
					min_body_ratio,
				)
			} else {
				is_strong_bear(
					opens[idx],
					highs[idx],
					lows[idx],
					closes[idx],
					min_body_ratio,
				)
			};
			if !strong {
				valid = false;
				break;
			}
			if k > 0 {
				let inside = if bullish {
					opens[idx] > opens[idx - 1] && opens[idx] < closes[idx - 1]
				} else {
					opens[idx] < opens[idx - 1] && opens[idx] > closes[idx - 1]
				};
				let progressing = if bullish {
					closes[idx] > closes[idx - 1]
				} else {
					closes[idx] < closes[idx - 1]
				};
				if !inside || !progressing {
					valid = false;
					break;
				}
			}
		}
		if valid {
			*out = if bullish { 1.0 } else { -1.0 };
		}
	}
	Ok(results)
}

/// Three inside up: bearish candle, small bullish candle inside it, then a
/// bullish candle closing above the first candle's open. Bullish.
pub fn three_inside_up(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	min_body_ratio: Option<f64>,
) -> IndicatorResult<Vec<f64>> {
	inside_internal(opens, highs, lows, closes, min_body_ratio, true)
}

/// Three inside down: bullish candle, small bearish candle inside it, then a
/// bearish candle closing below the first candle's close. Bearish.
pub fn three_inside_down(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	min_body_ratio: Option<f64>,
) -> IndicatorResult<Vec<f64>> {
	inside_internal(opens, highs, lows, closes, min_body_ratio, false)
}

fn inside_internal(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	min_body_ratio: Option<f64>,
	bullish: bool,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let min_body_ratio = min_body_ratio.unwrap_or(0.3);

	let mut results = vec![0.0; opens.len()];
	for i in 2..opens.len() {
		let first_open = opens[i - 2];
		let first_close = closes[i - 2];
		let second_open = opens[i - 1];
		let second_close = closes[i - 1];
		let third_open = opens[i];
		let third_close = closes[i];

		let second_inside = if bullish {
			second_open >= first_close && second_close <= first_open
		} else {
			second_open <= first_close && second_close >= first_open
		};

		let third_strong = if bullish {
			is_strong_bull(third_open, highs[i], lows[i], third_close, min_body_ratio)
				&& third_close > first_open
		} else {
			is_strong_bear(third_open, highs[i], lows[i], third_close, min_body_ratio)
				&& third_close < first_close
		};

		if second_inside && third_strong {
			results[i] = if bullish { 1.0 } else { -1.0 };
		}
	}
	Ok(results)
}

/// Three outside up: bearish candle, strong bullish candle engulfing it, then
/// a bullish candle closing higher. Bullish.
pub fn three_outside_up(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	min_body_ratio: Option<f64>,
) -> IndicatorResult<Vec<f64>> {
	outside_internal(opens, highs, lows, closes, min_body_ratio, true)
}

/// Three outside down: bullish candle, strong bearish candle engulfing it,
/// then a bearish candle closing lower. Bearish.
pub fn three_outside_down(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	min_body_ratio: Option<f64>,
) -> IndicatorResult<Vec<f64>> {
	outside_internal(opens, highs, lows, closes, min_body_ratio, false)
}

fn outside_internal(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	min_body_ratio: Option<f64>,
	bullish: bool,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let min_body_ratio = min_body_ratio.unwrap_or(0.5);

	let mut results = vec![0.0; opens.len()];
	for i in 2..opens.len() {
		let engulf = if bullish {
			is_bear(opens[i - 2], closes[i - 2])
				&& opens[i - 1] <= closes[i - 2]
				&& closes[i - 1] >= opens[i - 2]
				&& is_strong_bull(
					opens[i - 1],
					highs[i - 1],
					lows[i - 1],
					closes[i - 1],
					min_body_ratio,
				)
		} else {
			is_bull(opens[i - 2], closes[i - 2])
				&& opens[i - 1] >= closes[i - 2]
				&& closes[i - 1] <= opens[i - 2]
				&& is_strong_bear(
					opens[i - 1],
					highs[i - 1],
					lows[i - 1],
					closes[i - 1],
					min_body_ratio,
				)
		};

		let confirmation = if bullish {
			is_bull(opens[i], closes[i]) && closes[i] > closes[i - 1]
		} else {
			is_bear(opens[i], closes[i]) && closes[i] < closes[i - 1]
		};

		if engulf && confirmation {
			results[i] = if bullish { 1.0 } else { -1.0 };
		}
	}
	Ok(results)
}

/// Abandoned baby: a strong candle, a gap, a doji star, a gap back, then a
/// strong candle in the opposite direction. Bullish (down-gap) and bearish
/// (up-gap) variants are both detected.
pub fn abandoned_baby(
	opens: &[f64],
	highs: &[f64],
	lows: &[f64],
	closes: &[f64],
	body_ratio: Option<f64>,
	min_body_ratio: Option<f64>,
) -> IndicatorResult<Vec<f64>> {
	validate_multiple_arrays(&[opens, highs, lows, closes])?;

	let body_ratio = body_ratio.unwrap_or(0.1);
	let min_body_ratio = min_body_ratio.unwrap_or(0.3);

	let mut results = vec![0.0; opens.len()];
	for i in 2..opens.len() {
		let first_open = opens[i - 2];
		let first_close = closes[i - 2];
		let star_open = opens[i - 1];
		let star_close = closes[i - 1];
		let third_open = opens[i];
		let third_close = closes[i];

		if !is_small_body(star_open, highs[i - 1], lows[i - 1], star_close, body_ratio) {
			continue;
		}

		let bullish = is_strong_bear(
			first_open,
			highs[i - 2],
			lows[i - 2],
			first_close,
			min_body_ratio,
		) && highs[i - 1] < first_close
			&& lows[i] > highs[i - 1]
			&& is_strong_bull(third_open, highs[i], lows[i], third_close, min_body_ratio);

		let bearish = is_strong_bull(
			first_open,
			highs[i - 2],
			lows[i - 2],
			first_close,
			min_body_ratio,
		) && lows[i - 1] > first_close
			&& highs[i] < lows[i - 1]
			&& is_strong_bear(third_open, highs[i], lows[i], third_close, min_body_ratio);

		if bullish {
			results[i] = 1.0;
		} else if bearish {
			results[i] = -1.0;
		}
	}
	Ok(results)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::patterns::helpers::test_helpers::{bar, ohlc};

	/// Four declining closes (9.8 -> 9.5 -> 9.2 -> 9.0) then `pattern` at
	/// index 4 and a neutral continuation bar. With `trend_bars = 3` the
	/// signal fires at index 4.
	fn after_downtrend(pattern: [f64; 4]) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
		let mut bars = vec![
			bar(10.0, 10.5, 9.5, 9.8),
			bar(9.8, 10.2, 9.3, 9.5),
			bar(9.5, 9.9, 9.0, 9.2),
			bar(9.2, 9.6, 8.8, 9.0),
		];
		bars.push(pattern);
		bars.push(bar(9.2, 9.6, 9.0, 9.45));
		ohlc(&bars)
	}

	/// Four rising closes (10.0 -> 10.4 -> 10.6 -> 10.9) then `pattern` at
	/// index 4 and a neutral continuation bar. The continuation bar is a strong
	/// body (ratio 0.6) so it can never itself be a small-bodied pattern.
	fn after_uptrend(pattern: [f64; 4]) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
		let mut bars = vec![
			bar(9.8, 10.3, 9.6, 10.0),
			bar(10.0, 10.5, 9.8, 10.4),
			bar(10.4, 10.9, 10.2, 10.6),
			bar(10.6, 11.0, 10.4, 10.9),
		];
		bars.push(pattern);
		bars.push(bar(11.0, 11.4, 10.9, 11.3));
		ohlc(&bars)
	}

	fn signal_at(signals: &[f64], idx: usize, dir: f64) {
		let mut fired: Vec<usize> = (0..signals.len()).filter(|&i| signals[i] != 0.0).collect();
		fired.sort_unstable();
		assert_eq!(
			fired,
			vec![idx],
			"expected a single signal at bar {idx}, got {fired:?}"
		);
		assert!(
			(signals[idx] - dir).abs() < 0.5,
			"expected direction {dir} at bar {idx}, got {}",
			signals[idx]
		);
	}

	#[test]
	fn single_candle_patterns() {
		// Hammer after a decline -> bullish.
		// (upper shadow 0.05 is clearly below the 0.1 body so the float
		// comparison `short_shadow > b` can't misfire.)
		let (o, h, l, c) = after_downtrend(bar(9.1, 9.25, 8.5, 9.2));
		signal_at(
			&hammer(&o, &h, &l, &c, None, None, Some(3)).unwrap(),
			4,
			1.0,
		);

		// Inverted hammer after a decline -> bullish.
		let (o, h, l, c) = after_downtrend(bar(9.1, 9.9, 9.05, 9.2));
		signal_at(
			&inverted_hammer(&o, &h, &l, &c, None, None, Some(3)).unwrap(),
			4,
			1.0,
		);

		// Hanging man after an advance -> bearish.
		let (o, h, l, c) = after_uptrend(bar(10.8, 10.87, 10.5, 10.85));
		signal_at(
			&hanging_man(&o, &h, &l, &c, None, None, Some(3)).unwrap(),
			4,
			-1.0,
		);

		// Shooting star after an advance -> bearish.
		let (o, h, l, c) = after_uptrend(bar(10.8, 11.7, 10.75, 10.86));
		signal_at(
			&shooting_star(&o, &h, &l, &c, None, None, Some(3)).unwrap(),
			4,
			-1.0,
		);

		// Spinning top after a decline -> bullish.
		let (o, h, l, c) = after_downtrend(bar(9.1, 9.8, 8.85, 9.2));
		signal_at(
			&spinning_top(&o, &h, &l, &c, None, Some(3)).unwrap(),
			4,
			1.0,
		);

		// Long-legged doji after a decline -> bullish.
		let (o, h, l, c) = after_downtrend(bar(9.05, 9.95, 8.9, 9.1));
		signal_at(
			&long_legged_doji(&o, &h, &l, &c, None, None, Some(3)).unwrap(),
			4,
			1.0,
		);

		// Dragonfly doji after a decline -> bullish.
		let (o, h, l, c) = after_downtrend(bar(9.08, 9.11, 8.9, 9.1));
		signal_at(
			&dragonfly_doji(&o, &h, &l, &c, None, None, Some(3)).unwrap(),
			4,
			1.0,
		);

		// Gravestone doji after an advance -> bearish.
		let (o, h, l, c) = after_uptrend(bar(9.10, 9.30, 9.095, 9.09));
		signal_at(
			&gravestone_doji(&o, &h, &l, &c, None, None, Some(3)).unwrap(),
			4,
			-1.0,
		);
	}

	#[test]
	fn two_candle_patterns() {
		// Bullish harami: small bull inside a prior bear body.
		let (o, h, l, c) = ohlc(&[bar(10.0, 10.5, 9.5, 9.6), bar(9.6, 9.75, 9.55, 9.7)]);
		signal_at(&bullish_harami(&o, &h, &l, &c, None).unwrap(), 1, 1.0);

		// Bearish harami: small bear inside a prior bull body.
		let (o, h, l, c) = ohlc(&[bar(9.5, 10.0, 9.5, 9.9), bar(9.9, 10.1, 9.75, 9.85)]);
		signal_at(&bearish_harami(&o, &h, &l, &c, None).unwrap(), 1, -1.0);

		// Piercing line.
		let (o, h, l, c) = ohlc(&[bar(10.0, 10.3, 9.4, 9.5), bar(9.3, 9.8, 9.2, 9.85)]);
		signal_at(&piercing_line(&o, &h, &l, &c).unwrap(), 1, 1.0);

		// Dark cloud cover.
		let (o, h, l, c) = ohlc(&[bar(9.5, 9.8, 9.2, 10.0), bar(10.3, 10.4, 9.6, 9.7)]);
		signal_at(&dark_cloud_cover(&o, &h, &l, &c).unwrap(), 1, -1.0);

		// Tweezer bottom after a decline.
		let (o, h, l, c) = ohlc(&[
			bar(9.6, 10.0, 9.3, 9.8),
			bar(9.5, 9.8, 9.25, 9.6),
			bar(9.4, 9.6, 9.25, 9.45),
			bar(9.3, 9.5, 9.2, 9.35),
			bar(9.35, 9.6, 9.2005, 9.4),
		]);
		signal_at(&tweezer_bottom(&o, &h, &l, &c, None).unwrap(), 4, 1.0);

		// Tweezer top after an advance.
		let (o, h, l, c) = ohlc(&[
			bar(9.6, 10.0, 9.5, 9.8),
			bar(9.9, 10.2, 9.8, 10.1),
			bar(10.2, 10.4, 10.1, 10.3),
			bar(10.4, 10.6, 10.3, 10.5),
			bar(10.5, 10.6005, 10.4, 10.55),
		]);
		signal_at(&tweezer_top(&o, &h, &l, &c, None).unwrap(), 4, -1.0);
	}

	#[test]
	fn three_candle_patterns() {
		// Three white soldiers.
		let (o, h, l, c) = ohlc(&[
			bar(9.5, 10.2, 9.4, 10.0),
			bar(9.9, 10.7, 9.85, 10.5),
			bar(10.4, 11.2, 10.35, 11.0),
		]);
		signal_at(&three_white_soldiers(&o, &h, &l, &c, None).unwrap(), 2, 1.0);

		// Three black crows.
		let (o, h, l, c) = ohlc(&[
			bar(10.5, 10.6, 9.8, 10.0),
			bar(10.1, 10.2, 9.3, 9.5),
			bar(9.6, 9.7, 8.8, 9.0),
		]);
		signal_at(&three_black_crows(&o, &h, &l, &c, None).unwrap(), 2, -1.0);

		// Three inside up.
		let (o, h, l, c) = ohlc(&[
			bar(10.0, 10.4, 9.5, 9.6),
			bar(9.7, 9.9, 9.6, 9.85),
			bar(9.9, 10.4, 9.85, 10.3),
		]);
		signal_at(&three_inside_up(&o, &h, &l, &c, None).unwrap(), 2, 1.0);

		// Three inside down.
		let (o, h, l, c) = ohlc(&[
			bar(9.5, 9.7, 9.3, 9.6),
			bar(9.55, 9.62, 9.4, 9.5),
			bar(9.4, 9.47, 8.9, 9.0),
		]);
		signal_at(&three_inside_down(&o, &h, &l, &c, None).unwrap(), 2, -1.0);

		// Three outside up.
		let (o, h, l, c) = ohlc(&[
			bar(10.0, 10.3, 9.6, 9.7),
			bar(9.6, 10.5, 9.5, 10.4),
			bar(10.3, 10.7, 10.2, 10.6),
		]);
		signal_at(&three_outside_up(&o, &h, &l, &c, None).unwrap(), 2, 1.0);

		// Three outside down.
		let (o, h, l, c) = ohlc(&[
			bar(9.5, 9.6, 9.3, 9.6),
			bar(9.6, 9.65, 8.8, 9.0),
			bar(9.1, 9.15, 8.7, 8.8),
		]);
		signal_at(&three_outside_down(&o, &h, &l, &c, None).unwrap(), 2, -1.0);

		// Abandoned baby (bullish): gap down, doji star, gap up.
		let (o, h, l, c) = ohlc(&[
			bar(10.0, 10.2, 9.4, 9.5),
			bar(9.30, 9.35, 9.22, 9.29),
			bar(9.6, 10.1, 9.55, 10.0),
		]);
		signal_at(&abandoned_baby(&o, &h, &l, &c, None, None).unwrap(), 2, 1.0);

		// Abandoned baby (bearish): gap up, doji star, gap down.
		let (o, h, l, c) = ohlc(&[
			bar(9.4, 9.8, 9.3, 9.7),
			bar(9.78, 9.83, 9.72, 9.775),
			bar(9.6, 9.62, 9.1, 9.2),
		]);
		signal_at(
			&abandoned_baby(&o, &h, &l, &c, None, None).unwrap(),
			2,
			-1.0,
		);
	}
}
