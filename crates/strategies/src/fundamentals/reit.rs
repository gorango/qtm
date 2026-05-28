#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};

use factors_core::{FundamentalPoint, FundamentalPointData};

fn price_to_affo(d: &FundamentalPointData) -> Option<f64> {
	let affo = d.affo_per_share?;
	if affo == 0.0 {
		None
	} else {
		Some((d.market_cap? / d.shares_outstanding?) / affo)
	}
}

// ── Configs ──────────────────────────────────────────────

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HighYieldReitConfig {
	pub price_to_affo_threshold: Option<f64>,
	pub min_dividend_yield: Option<f64>,
	pub max_affo_payout_ratio: Option<f64>,
	pub min_dividend_growth: Option<f64>,
	pub dividend_growth_years: Option<u32>,
	pub exit_price_to_affo_threshold: Option<f64>,
}

impl Default for HighYieldReitConfig {
	fn default() -> Self {
		Self {
			price_to_affo_threshold: Some(15.0),
			min_dividend_yield: Some(0.06),
			max_affo_payout_ratio: Some(0.95),
			min_dividend_growth: Some(0.0),
			dividend_growth_years: Some(3),
			exit_price_to_affo_threshold: Some(20.0),
		}
	}
}

// ── Strategies ───────────────────────────────────────────

/// High-yield REIT: P/AFFO < 15, yield > 6%, AFFO payout < 95%, dividend growth >= 0%
pub fn high_yield_reit_strategy(
	points: Vec<FundamentalPoint>,
	config: Option<HighYieldReitConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let p_affo = cfg.price_to_affo_threshold.unwrap_or(15.0);
	let min_yield = cfg.min_dividend_yield.unwrap_or(0.06);
	let max_payout = cfg.max_affo_payout_ratio.unwrap_or(0.95);

	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let val_ok = price_to_affo(d).map(|v| v < p_affo).unwrap_or(false);
			let yield_ok = d.dividend_yield.map(|v| v > min_yield).unwrap_or(false);
			let safety_ok = d.payout_ratio_ffo.map(|v| v < max_payout).unwrap_or(false);
			let growth_ok = d.dividend_growth_3y.map(|v| v >= 0.0).unwrap_or(false);
			if val_ok && yield_ok && safety_ok && growth_ok {
				1
			} else {
				0
			}
		})
		.collect()
}

/// High-yield REIT with entry + exit signals (returns 1=enter, -1=exit, 0=hold)
pub fn high_yield_reit_signals(
	points: Vec<FundamentalPoint>,
	config: Option<HighYieldReitConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let p_affo = cfg.price_to_affo_threshold.unwrap_or(15.0);
	let min_yield = cfg.min_dividend_yield.unwrap_or(0.06);
	let max_payout = cfg.max_affo_payout_ratio.unwrap_or(0.95);
	let exit_p_affo = cfg.exit_price_to_affo_threshold.unwrap_or(20.0);

	points
		.iter()
		.map(|p| {
			let d = &p.data;
			let val_ok = price_to_affo(d).map(|v| v < p_affo).unwrap_or(false);
			let yield_ok = d.dividend_yield.map(|v| v > min_yield).unwrap_or(false);
			let safety_ok = d.payout_ratio_ffo.map(|v| v < max_payout).unwrap_or(false);
			let growth_ok = d.dividend_growth_3y.map(|v| v >= 0.0).unwrap_or(false);
			let expensive = price_to_affo(d).map(|v| v > exit_p_affo).unwrap_or(false);

			if val_ok && yield_ok && safety_ok && growth_ok {
				1
			} else if expensive {
				-1
			} else {
				0
			}
		})
		.collect()
}

// ── Metadata ─────────────────────────────────────────────

pub fn high_yield_reit_strategy_metadata() -> serde_json::Value {
	serde_json::json!({"id": "high-yield-reit","name": "High Yield REIT Fundamental","category": "fundamental","default_timeframes": ["1d","1w"],"description": "REITs with attractive P/AFFO valuations and high dividend yields"})
}
pub fn high_yield_reit_strategy_defaults() -> serde_json::Value {
	serde_json::json!({"params": {"price_to_affo_threshold": 15,"min_dividend_yield": 0.06,"max_affo_payout_ratio": 0.95,"min_dividend_growth": 0,"dividend_growth_years": 3,"exit_price_to_affo_threshold": 20},"optimization_bounds": []})
}
