//! Regression tests for the broken-indicator fixes (2026-08-15).
//!
//! Each test asserts behavior that was previously impossible: the fixed
//! indicator must fire on data where it used to return all zeros / NaN.
//! See research/validation/traditional/BROKEN-STRATEGIES.md for the
//! investigation.

use indicators_core::{cup_and_handle, elliott_wave, kst, sma_internal, super_trend, triangles};

fn crash_series() -> Vec<f64> {
	// rise for 200 bars, then crash ~40% and keep sliding — a supertrend
	// must flip to -1 during the decline.
	let mut closes: Vec<f64> = (0..200).map(|i| 100.0 + i as f64 * 0.5).collect();
	for i in 200..400 {
		let t = (i - 200) as f64;
		closes.push(200.0 - t * 0.4);
	}
	closes
}

#[test]
fn super_trend_direction_flips_on_downtrend() {
	let h: Vec<f64> = crash_series().iter().map(|c| c + 1.0).collect();
	let l: Vec<f64> = crash_series().iter().map(|c| c - 1.0).collect();
	let c = crash_series();
	let res = super_trend(&h, &l, &c, Some(14), Some(3.0)).unwrap();
	let has_up = res.direction.iter().any(|&d| d == 1);
	let has_down = res.direction.iter().any(|&d| d == -1);
	assert!(
		has_up && has_down,
		"direction must contain both 1 and -1, got {:?}",
		res.direction
	);
}

#[test]
fn sma_internal_recovers_after_nan_warmup() {
	// NaN in the input must not poison the running sum forever.
	let mut values = vec![f64::NAN; 10];
	values.extend(std::iter::repeat(1.0).take(30));
	let out = sma_internal(&values, 5);
	assert!(out[9].is_nan());
	// once the NaN leaves the window (i >= 14) the SMA is exactly 1.0
	for &v in &out[14..] {
		assert_eq!(v, 1.0, "SMA must recover after the NaN leaves the window");
	}
}

#[test]
fn kst_has_finite_values_and_crossovers() {
	// KST used to be all-NaN (NaN-poisoned SMA) so crossovers never fired.
	let mut closes: Vec<f64> = Vec::new();
	let mut x = 100.0;
	for i in 0..400 {
		x += (i as f64 * 0.37).sin() + (i as f64 * 0.011).cos() * 0.5;
		closes.push(x);
	}
	let res = kst(&closes, None);
	let finite = res.kst.iter().filter(|v| v.is_finite()).count();
	assert!(
		finite > 300,
		"kst should be finite after warmup, got {finite}/{}",
		res.kst.len()
	);
	// at least one crossover of kst over/under signal
	let mut crosses = 0;
	for i in 1..res.kst.len() {
		let a = res.kst[i - 1] - res.signal[i - 1];
		let b = res.kst[i] - res.signal[i];
		if a.is_finite() && b.is_finite() && (a <= 0.0) != (b <= 0.0) {
			crosses += 1;
		}
	}
	assert!(crosses >= 3, "kst must cross its signal, got {crosses}");
}

fn noisy_ascending_triangle() -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
	// 250 bars of a noisy ascending triangle (flat highs, rising lows),
	// then a 60-bar breakout.  The end-anchored detector never fired here.
	// Deterministic noise via a fixed LCG.
	let mut seed = 42u64;
	let mut noise = || {
		seed = seed
			.wrapping_mul(6364136223846793005)
			.wrapping_add(1442695040888963407);
		(seed >> 33) as f64 / (1u64 << 31) as f64 - 1.0
	};
	let mut highs = Vec::with_capacity(310);
	let mut lows = Vec::with_capacity(310);
	let mut closes = Vec::with_capacity(310);
	for i in 0..250 {
		highs.push(105.0 + noise() * 0.08);
		let low = 100.0 + 4.0 * i as f64 / 249.0 + noise() * 0.1;
		lows.push(low);
		closes.push((highs[i] + lows[i]) / 2.0 + noise() * 0.1);
	}
	for i in 0..60 {
		let bo = 105.5 + 4.5 * i as f64 / 59.0;
		highs.push(bo + 0.3);
		lows.push(bo - 0.5);
		closes.push(bo);
	}
	let opens = closes.clone();
	(opens, highs, lows, closes)
}

#[test]
fn triangles_fires_on_sliding_window() {
	let (o, h, l, c) = noisy_ascending_triangle();
	let out = triangles(&o, &h, &l, &c, Some(4), Some(0.01), Some(0.001)).unwrap();
	let fired = out.iter().filter(|&&v| v != 0.0).count();
	assert!(
		fired >= 1,
		"triangles must fire on a mid-series triangle, got all zeros"
	);
	assert!(out.iter().any(|&v| v == 1.0), "breakout should be bullish");
}

fn cup_series() -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
	// cup: 100 -> 85 -> 100 over 200 bars (~15% depth); handle: pullback to
	// 97 then a TIGHT consolidation (97-97.9); breakout up to 105.
	let mut cup: Vec<f64> = (0..100).map(|i| 100.0 - 14.8 * i as f64 / 99.0).collect();
	// bottom bar dips below both neighbors so it is a strict local min
	cup.push(85.1);
	cup.extend((0..100).map(|i| 85.0 + 15.0 * i as f64 / 99.0));
	// handle: TIGHT low consolidation right off the rim (96.8-97.4) — the
	// detector's handle metric requires the whole handle window to stay well
	// below the right-shoulder price
	let handle: Vec<f64> = (0..60).map(|i| 96.8 + 0.6 * ((i % 2) as f64)).collect();
	let breakout: Vec<f64> = (0..30).map(|i| 98.0 + 7.0 * i as f64 / 29.0).collect();
	let closes: Vec<f64> = cup.into_iter().chain(handle).chain(breakout).collect();
	let highs: Vec<f64> = closes.iter().map(|c| c + 0.1).collect();
	let lows: Vec<f64> = closes.iter().map(|c| c - 0.1).collect();
	let opens: Vec<f64> = closes.iter().map(|c| c - 0.05).collect();
	(opens, highs, lows, closes)
}

#[test]
fn cup_and_handle_fires_on_mid_history_cup() {
	let (o, h, l, c) = cup_series();
	// min_duration ~= the cup's own duration so the shoulder windows span the
	// cup rims, and the handle (60 bars) fits inside min_duration/4.
	let out = cup_and_handle(&o, &h, &l, &c, Some(0.1), Some(0.3), Some(200)).unwrap();
	let fired = out.iter().filter(|&&v| v != 0.0).count();
	assert!(
		fired >= 1,
		"cup_and_handle must fire on a mid-history cup, got all zeros"
	);
}

fn impulse_series() -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
	// 5-wave impulse with a LEADING dip (w1 start = detected trough) and
	// tapered segment starts (every turn a strict local extremum).  Values
	// chosen so the retracement/extension ratios land inside the gates:
	//   w1 0.5->10, w2 retraces 0.618 of w1 (low ~4.2), w3 extends 1.7x
	//   (top ~19.7), w4 retraces 0.382 of w3 (low ~13.9), w5 closes above
	//   the w1 top.
	let dip: Vec<f64> = (0..20).map(|i| 1.0 - 0.5 * i as f64 / 19.0).collect();
	let w1: Vec<f64> = (0..100).map(|i| 0.51 + 9.49 * i as f64 / 99.0).collect();
	let w2: Vec<f64> = (0..60).map(|i| 9.9 - 5.7 * i as f64 / 59.0).collect();
	let w3: Vec<f64> = (0..160).map(|i| 4.3 + 15.4 * i as f64 / 159.0).collect();
	let w4: Vec<f64> = (0..70).map(|i| 19.6 - 5.7 * i as f64 / 69.0).collect();
	let w5: Vec<f64> = (0..90).map(|i| 14.0 + 9.0 * i as f64 / 89.0).collect();
	// tail decline so the w5 top (index 499) is a strict detected peak
	let tail: Vec<f64> = (0..10).map(|i| 23.0 - 0.8 * i as f64 / 9.0).collect();
	let mut seg: Vec<f64> = Vec::new();
	seg.extend(dip);
	seg.extend(w1);
	seg.extend(w2);
	seg.extend(w3);
	seg.extend(w4);
	seg.extend(w5);
	seg.extend(tail);
	let closes: Vec<f64> = seg.iter().map(|s| 100.0 + s).collect();
	let highs: Vec<f64> = closes.iter().map(|c| c + 0.1).collect();
	let lows: Vec<f64> = closes.iter().map(|c| c - 0.1).collect();
	let opens: Vec<f64> = closes.iter().map(|c| c - 0.05).collect();
	(opens, highs, lows, closes)
}

#[test]
fn elliott_wave_fires_on_clean_impulse() {
	let (o, h, l, c) = impulse_series();
	let out = elliott_wave(
		&o,
		&h,
		&l,
		&c,
		Some(0.618),
		Some(0.382),
		Some(1.618),
		Some(5),
		Some(2),
		Some(0.1),
	)
	.unwrap();
	let fired = out.iter().filter(|&&v| v != 0.0).count();
	assert!(
		fired >= 1,
		"elliott_wave must fire on a clean impulse, got all zeros"
	);
	assert!(
		out.iter().any(|&v| v == 1.0),
		"impulse breakout should be bullish (1.0)"
	);
}
