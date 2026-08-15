//! Regression tests for the broken-strategy fixes (2026-08-15).
//!
//! Each strategy previously never emitted a signal; after the fix it must
//! fire on data where its documented behavior applies.  See
//! research/validation/traditional/BROKEN-STRATEGIES.md.

use strategies_core::{
	cup_and_handle_strategy, donchian_breakout_strategy, double_top_stochastic_strategy,
	elliott_wave_strategy, kst_strategy, super_trend_strategy, vwap_ema_rsi_strategy,
};

fn crash_series() -> Vec<f64> {
	let mut closes: Vec<f64> = (0..200).map(|i| 100.0 + i as f64 * 0.5).collect();
	for i in 200..400 {
		closes.push(200.0 - (i - 200) as f64 * 0.4);
	}
	closes
}

#[test]
fn super_trend_strategy_fires_on_downtrend() {
	let c = crash_series();
	let h: Vec<f64> = c.iter().map(|x| x + 1.0).collect();
	let l: Vec<f64> = c.iter().map(|x| x - 1.0).collect();
	let out = super_trend_strategy(&h, &l, &c, None).unwrap();
	let fired = out.iter().filter(|&&s| s != 0).count();
	assert!(
		fired >= 1,
		"super_trend strategy must fire on a downtrend, got all zeros"
	);
}

#[test]
fn donchian_breakout_strategy_fires_on_breakout() {
	// flat then a step up beyond the prior N-bar high
	let mut closes: Vec<f64> = std::iter::repeat(100.0).take(40).collect();
	closes.extend(std::iter::repeat(105.0).take(30));
	let out = donchian_breakout_strategy(&closes, None).unwrap();
	let buys = out.iter().filter(|&&s| s == 1).count();
	assert!(
		buys >= 1,
		"donchian_breakout must fire a buy on a step-up, got all zeros"
	);
}

#[test]
fn kst_strategy_fires_on_random_walk() {
	let mut closes: Vec<f64> = Vec::new();
	let mut x = 100.0;
	for i in 0..400 {
		x += (i as f64 * 0.37).sin() + (i as f64 * 0.011).cos() * 0.5;
		closes.push(x);
	}
	let out = kst_strategy(&closes, None).unwrap();
	let fired = out.iter().filter(|&&s| s != 0).count();
	assert!(
		fired >= 1,
		"kst strategy must fire on a trending random walk, got all zeros"
	);
}

#[test]
fn vwap_ema_rsi_strategy_fires_on_trend() {
	// flat then a sustained rally: EMA fast crosses above slow with price
	// above VWAP and RSI above 50
	let mut closes: Vec<f64> = std::iter::repeat(100.0).take(40).collect();
	for i in 0..60 {
		closes.push(100.0 + 0.8 * (i as f64 + 1.0));
	}
	let h: Vec<f64> = closes.iter().map(|x| x + 0.5).collect();
	let l: Vec<f64> = closes.iter().map(|x| x - 0.5).collect();
	let v: Vec<f64> = closes.iter().map(|_| 1.0).collect();
	let out = vwap_ema_rsi_strategy(&h, &l, &closes, &v, None).unwrap();
	let buys = out.iter().filter(|&&s| s == 1).count();
	assert!(
		buys >= 1,
		"vwap_ema_rsi must fire a buy on a rally, got all zeros"
	);
}

fn double_top_series() -> Vec<f64> {
	// rise to a top, dip, retest near the top (double top), then decline
	let mut c: Vec<f64> = (0..80).map(|i| 100.0 + 0.5 * i as f64).collect();
	c.extend((0..20).map(|i| 140.0 - 0.3 * i as f64)); // dip to 134
	c.extend((0..20).map(|i| 134.0 + 0.28 * i as f64)); // retest ~139.6
	c.extend((0..60).map(|i| 139.6 - 0.5 * i as f64)); // decline
	c
}

#[test]
fn double_top_stochastic_strategy_fires_on_double_top() {
	let c = double_top_series();
	let h: Vec<f64> = c.iter().map(|x| x + 0.3).collect();
	let l: Vec<f64> = c.iter().map(|x| x - 0.3).collect();
	let out = double_top_stochastic_strategy(&h, &l, &c, None).unwrap();
	let fired = out.iter().filter(|&&s| s != 0).count();
	assert!(
		fired >= 1,
		"double_top_stochastic must fire on a double top, got all zeros"
	);
}

fn cup_series() -> (Vec<f64>, Vec<f64>, Vec<f64>) {
	let mut cup: Vec<f64> = (0..100).map(|i| 100.0 - 14.8 * i as f64 / 99.0).collect();
	cup.push(85.1);
	cup.extend((0..100).map(|i| 85.0 + 15.0 * i as f64 / 99.0));
	let handle: Vec<f64> = (0..60).map(|i| 96.8 + 0.6 * ((i % 2) as f64)).collect();
	let breakout: Vec<f64> = (0..30).map(|i| 98.0 + 7.0 * i as f64 / 29.0).collect();
	let closes: Vec<f64> = cup.into_iter().chain(handle).chain(breakout).collect();
	let highs: Vec<f64> = closes.iter().map(|c| c + 0.1).collect();
	let lows: Vec<f64> = closes.iter().map(|c| c - 0.1).collect();
	(highs, lows, closes)
}

#[test]
fn cup_and_handle_strategy_fires_on_cup() {
	let (h, l, c) = cup_series();
	let o = c.iter().map(|x| x - 0.05).collect::<Vec<f64>>();
	// the strategy's default min_duration=20 only fits tiny cups; pass the
	// cup's actual duration (200) as the config
	let cfg = strategies_core::CupAndHandleConfig {
		cup_depth: Some(0.1),
		handle_retracement: Some(0.3),
		min_duration: Some(200),
	};
	let out = cup_and_handle_strategy(&o, &h, &l, &c, Some(cfg)).unwrap();
	let fired = out.iter().filter(|&&s| s != 0).count();
	assert!(
		fired >= 1,
		"cup_and_handle strategy must fire on a cup, got all zeros"
	);
}

fn impulse_series() -> (Vec<f64>, Vec<f64>, Vec<f64>) {
	let dip: Vec<f64> = (0..20).map(|i| 1.0 - 0.5 * i as f64 / 19.0).collect();
	let w1: Vec<f64> = (0..100).map(|i| 0.51 + 9.49 * i as f64 / 99.0).collect();
	let w2: Vec<f64> = (0..60).map(|i| 9.9 - 5.7 * i as f64 / 59.0).collect();
	let w3: Vec<f64> = (0..160).map(|i| 4.3 + 15.4 * i as f64 / 159.0).collect();
	let w4: Vec<f64> = (0..70).map(|i| 19.6 - 5.7 * i as f64 / 69.0).collect();
	let w5: Vec<f64> = (0..90).map(|i| 14.0 + 9.0 * i as f64 / 89.0).collect();
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
	(highs, lows, closes)
}

#[test]
fn elliott_wave_strategy_fires_on_impulse() {
	let (h, l, c) = impulse_series();
	let o = c.iter().map(|x| x - 0.05).collect::<Vec<f64>>();
	let out = elliott_wave_strategy(&o, &h, &l, &c, None).unwrap();
	let fired = out.iter().filter(|&&s| s != 0).count();
	assert!(
		fired >= 1,
		"elliott_wave strategy must fire on an impulse, got all zeros"
	);
}
