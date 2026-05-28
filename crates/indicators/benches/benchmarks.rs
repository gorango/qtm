use criterion::{black_box, criterion_group, criterion_main, Criterion};
use indicators_core::{bb, ema_internal, macd, rsi, sma_internal};

fn bench_sma(c: &mut Criterion) {
	let values: Vec<f64> = (0..10_000).map(|i| i as f64).collect();
	c.bench_function("sma_10000_period_20", |b| {
		b.iter(|| sma_internal(black_box(&values), black_box(20)))
	});
}

fn bench_ema(c: &mut Criterion) {
	let values: Vec<f64> = (0..10_000).map(|i| i as f64).collect();
	c.bench_function("ema_10000_period_20", |b| {
		b.iter(|| ema_internal(black_box(&values), black_box(20)))
	});
}

fn bench_rsi(c: &mut Criterion) {
	let values: Vec<f64> = (0..10_000).map(|i| i as f64).collect();
	c.bench_function("rsi_10000_period_14", |b| {
		b.iter(|| rsi(black_box(&values), black_box(None)))
	});
}

fn bench_macd(c: &mut Criterion) {
	let values: Vec<f64> = (0..10_000).map(|i| i as f64).collect();
	c.bench_function("macd_10000_12_26_9", |b| {
		b.iter(|| macd(black_box(&values), black_box(None)))
	});
}

fn bench_bb(c: &mut Criterion) {
	let values: Vec<f64> = (0..10_000).map(|i| i as f64).collect();
	c.bench_function("bb_10000_period_20", |b| {
		b.iter(|| bb(black_box(&values), black_box(None)))
	});
}

criterion_group!(benches, bench_sma, bench_ema, bench_rsi, bench_macd, bench_bb);
criterion_main!(benches);
