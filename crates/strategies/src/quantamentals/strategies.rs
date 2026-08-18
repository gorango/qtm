#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};

use factors_core::{
	debt_to_equity, eps_growth_qo_q, exchange_flow_momentum, odds_momentum, price_to_book,
	price_to_earnings, price_to_free_cash_flow, return_on_equity,
};
use factors_core::{Bar, FactorPoint, FundamentalPoint, OnChainDataPoint, PredictionMarketPoint};

fn latest_factor_at_or_before(factors: &[FactorPoint], time: f64) -> Option<f64> {
	factors
		.iter()
		.filter(|f| f.date <= time)
		.max_by(|a, b| a.date.total_cmp(&b.date))
		.map(|f| f.value)
}

fn prices_from_bars(bars: &[Bar]) -> Vec<f64> {
	bars.iter().map(|b| b.close).collect()
}

fn highs_from_bars(bars: &[Bar]) -> Vec<f64> {
	bars.iter().map(|b| b.high).collect()
}

fn lows_from_bars(bars: &[Bar]) -> Vec<f64> {
	bars.iter().map(|b| b.low).collect()
}

fn opens_from_bars(bars: &[Bar]) -> Vec<f64> {
	bars.iter().map(|b| b.open).collect()
}

fn times_from_bars(bars: &[Bar]) -> Vec<f64> {
	bars.iter().map(|b| b.time).collect()
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct QarpConfig {
	pub roe_threshold: Option<f64>,
	pub de_threshold: Option<f64>,
	pub pe_threshold: Option<f64>,
	pub donchian_period: Option<u32>,
}

impl Default for QarpConfig {
	fn default() -> Self {
		Self {
			roe_threshold: Some(0.15),
			de_threshold: Some(1.0),
			pe_threshold: Some(20.0),
			donchian_period: Some(20),
		}
	}
}

/// Qarp
///
/// Generates buy/sell signals combining quantitative and fundamental factors.
pub fn qarp_strategy(
	fundamentals: Vec<FundamentalPoint>,
	prices: Vec<Bar>,
	config: Option<QarpConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let roe_thresh = cfg.roe_threshold.unwrap_or(0.15);
	let de_thresh = cfg.de_threshold.unwrap_or(1.0);
	let pe_thresh = cfg.pe_threshold.unwrap_or(20.0);
	let dc_period = cfg.donchian_period.unwrap_or(20);

	if fundamentals.is_empty() || prices.is_empty() {
		return vec![0; prices.len()];
	}

	let roe = return_on_equity(fundamentals.clone());
	let de = debt_to_equity(fundamentals.clone());
	let pe = price_to_earnings(fundamentals.clone(), prices.clone());

	let closes = prices_from_bars(&prices);
	let dc = match indicators_core::donchian_channel(&closes, Some(dc_period)) {
		Ok(d) => d,
		Err(_) => return vec![0; prices.len()],
	};

	let times = times_from_bars(&prices);
	let mut signals = vec![0i8; prices.len()];

	for i in 0..prices.len() {
		let t = times[i];
		let roe_val = latest_factor_at_or_before(&roe, t).unwrap_or(0.0);
		let de_val = latest_factor_at_or_before(&de, t).unwrap_or(f64::MAX);
		let pe_val = latest_factor_at_or_before(&pe, t).unwrap_or(f64::MAX);

		if roe_val > roe_thresh
			&& de_val < de_thresh
			&& pe_val < pe_thresh
			&& i >= dc_period as usize
			&& prices[i].close > dc.upper[i]
		{
			signals[i] = 1;
		}
	}

	signals
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MultiFactorValueConfig {
	pub pfcf_threshold: Option<f64>,
	pub pe_threshold: Option<f64>,
	pub pb_threshold: Option<f64>,
	pub super_trend_period: Option<u32>,
	pub super_trend_multiplier: Option<f64>,
	pub required_factors: Option<u32>,
}

impl Default for MultiFactorValueConfig {
	fn default() -> Self {
		Self {
			pfcf_threshold: Some(15.0),
			pe_threshold: Some(15.0),
			pb_threshold: Some(1.5),
			super_trend_period: Some(14),
			super_trend_multiplier: Some(3.0),
			required_factors: Some(2),
		}
	}
}

/// Multi Factor Value
///
/// Generates buy/sell signals combining quantitative and fundamental factors.
pub fn multi_factor_value_strategy(
	fundamentals: Vec<FundamentalPoint>,
	prices: Vec<Bar>,
	config: Option<MultiFactorValueConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let pfcf_thresh = cfg.pfcf_threshold.unwrap_or(15.0);
	let pe_thresh = cfg.pe_threshold.unwrap_or(15.0);
	let pb_thresh = cfg.pb_threshold.unwrap_or(1.5);
	let st_period = cfg.super_trend_period.unwrap_or(14);
	let st_mult = cfg.super_trend_multiplier.unwrap_or(3.0);
	let required = cfg.required_factors.unwrap_or(2) as usize;

	if fundamentals.is_empty() || prices.is_empty() {
		return vec![0; prices.len()];
	}

	let pfcf = price_to_free_cash_flow(fundamentals.clone());
	let pe = price_to_earnings(fundamentals.clone(), prices.clone());
	let pb = price_to_book(fundamentals.clone());

	let highs = highs_from_bars(&prices);
	let lows = lows_from_bars(&prices);
	let closes = prices_from_bars(&prices);
	let st = match indicators_core::super_trend(
		&highs,
		&lows,
		&closes,
		Some(st_period),
		Some(st_mult),
	) {
		Ok(s) => s,
		Err(_) => return vec![0; prices.len()],
	};

	let times = times_from_bars(&prices);
	let mut signals = vec![0i8; prices.len()];

	for i in 0..prices.len() {
		let t = times[i];
		let pfcf_val = latest_factor_at_or_before(&pfcf, t);
		let pe_val = latest_factor_at_or_before(&pe, t);
		let pb_val = latest_factor_at_or_before(&pb, t);

		let mut factors_passed = 0usize;
		if pfcf_val.is_some_and(|v| v < pfcf_thresh) {
			factors_passed += 1;
		}
		if pe_val.is_some_and(|v| v < pe_thresh) {
			factors_passed += 1;
		}
		if pb_val.is_some_and(|v| v < pb_thresh) {
			factors_passed += 1;
		}

		let super_trend_bullish = if i < st_period as usize {
			false
		} else {
			st.direction[i] > 0
		};

		if factors_passed >= required && super_trend_bullish {
			signals[i] = 1;
		}
	}

	signals
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AlternativeDataConfig {
	pub exchange_threshold: Option<f64>,
	pub exchange_period: Option<u32>,
	pub odds_threshold: Option<f64>,
	pub odds_period: Option<u32>,
}

impl Default for AlternativeDataConfig {
	fn default() -> Self {
		Self {
			exchange_threshold: Some(0.05),
			exchange_period: Some(7),
			odds_threshold: Some(0.02),
			odds_period: Some(3),
		}
	}
}

/// Alternative Data
///
/// Generates buy/sell signals combining quantitative and fundamental factors.
pub fn alternative_data_strategy(
	on_chain_data: Vec<OnChainDataPoint>,
	prediction_data: Vec<PredictionMarketPoint>,
	config: Option<AlternativeDataConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let ex_thresh = cfg.exchange_threshold.unwrap_or(0.05);
	let ex_period = cfg.exchange_period.unwrap_or(7);
	let odds_thresh = cfg.odds_threshold.unwrap_or(0.02);
	let odds_period = cfg.odds_period.unwrap_or(3);

	let ex_flow = exchange_flow_momentum(on_chain_data, Some(ex_period as f64));
	let odds = odds_momentum(prediction_data, Some(odds_period));

	let mut signals = vec![0i8; std::cmp::max(ex_flow.len(), odds.len())];
	for (i, s) in signals.iter_mut().enumerate() {
		let ex_ok = ex_flow.get(i).is_some_and(|f| f.value > ex_thresh);
		let odds_ok = odds.get(i).is_some_and(|f| f.value > odds_thresh);
		if ex_ok && odds_ok {
			*s = 1;
		}
	}

	signals
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EventDrivenConfig {
	pub odds_threshold: Option<f64>,
	pub sma_period: Option<u32>,
}

impl Default for EventDrivenConfig {
	fn default() -> Self {
		Self {
			odds_threshold: Some(0.7),
			sma_period: Some(20),
		}
	}
}

/// Event Driven
///
/// Generates buy/sell signals combining quantitative and fundamental factors.
pub fn event_driven_strategy(
	prediction_data: Vec<PredictionMarketPoint>,
	prices: Vec<Bar>,
	config: Option<EventDrivenConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let odds_thresh = cfg.odds_threshold.unwrap_or(0.7);
	let sma_period = cfg.sma_period.unwrap_or(20) as usize;

	if prices.is_empty() {
		return Vec::new();
	}

	let closes = prices_from_bars(&prices);
	let sma = match indicators_core::sma(&closes, Some(sma_period as u32)) {
		Ok(s) => s,
		Err(_) => return vec![0; prices.len()],
	};

	let mut market_groups: std::collections::HashMap<String, Vec<&PredictionMarketPoint>> =
		std::collections::HashMap::new();
	for d in &prediction_data {
		market_groups
			.entry(d.market_id.clone())
			.or_default()
			.push(d);
	}

	let mut odds_by_time: Vec<f64> = Vec::new();
	for group in market_groups.values_mut() {
		group.sort_by(|a, b| a.time.total_cmp(&b.time));
		for p in group {
			odds_by_time.push(p.price);
		}
	}
	odds_by_time.sort_by(f64::total_cmp);

	let mut signals = vec![0i8; prices.len()];
	let times = times_from_bars(&prices);

	for i in 0..prices.len() {
		if i < sma_period {
			continue;
		}
		let sma_val = sma[i];
		let above_sma = prices[i].close > sma_val;

		let odds_above = latest_odds_above_threshold(&prediction_data, times[i], odds_thresh);

		if odds_above && above_sma {
			signals[i] = 1;
		}
	}

	signals
}

fn latest_odds_above_threshold(
	prediction_data: &[PredictionMarketPoint],
	time: f64,
	threshold: f64,
) -> bool {
	let mut relevant: Vec<f64> = prediction_data
		.iter()
		.filter(|p| p.time <= time)
		.map(|p| p.price)
		.collect();
	relevant.sort_by(f64::total_cmp);
	relevant.last().copied().unwrap_or(0.0) > threshold
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OnChainConfirmationConfig {
	pub fast_ema: Option<u32>,
	pub slow_ema: Option<u32>,
	pub netflow_threshold: Option<f64>,
}

impl Default for OnChainConfirmationConfig {
	fn default() -> Self {
		Self {
			fast_ema: Some(12),
			slow_ema: Some(26),
			netflow_threshold: Some(0.1),
		}
	}
}

/// On Chain Confirmation
///
/// Generates buy/sell signals combining quantitative and fundamental factors.
pub fn on_chain_confirmation_strategy(
	on_chain_data: Vec<OnChainDataPoint>,
	prices: Vec<Bar>,
	config: Option<OnChainConfirmationConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let fast_period = cfg.fast_ema.unwrap_or(12) as usize;
	let slow_period = cfg.slow_ema.unwrap_or(26) as usize;
	let netflow_thresh = cfg.netflow_threshold.unwrap_or(0.1);

	if prices.is_empty() {
		return Vec::new();
	}

	let closes = prices_from_bars(&prices);
	let ema_fast = match indicators_core::ema(&closes, Some(fast_period as u32)) {
		Ok(e) => e,
		Err(_) => return vec![0; prices.len()],
	};
	let ema_slow = match indicators_core::ema(&closes, Some(slow_period as u32)) {
		Ok(e) => e,
		Err(_) => return vec![0; prices.len()],
	};

	let flows: Vec<&OnChainDataPoint> = on_chain_data
		.iter()
		.filter(|d| d.metric == "exchangeNetflow")
		.collect();

	let mut signals = vec![0i8; prices.len()];

	for i in 0..prices.len() {
		if i < slow_period {
			continue;
		}
		let ema_crossover = ema_fast[i] > ema_slow[i] && ema_fast[i - 1] <= ema_slow[i - 1];

		let latest_flow = flows
			.iter()
			.filter(|f| f.time <= prices[i].time)
			.max_by(|a, b| a.time.total_cmp(&b.time))
			.map(|f| f.value)
			.unwrap_or(0.0);

		if ema_crossover && latest_flow < -netflow_thresh {
			signals[i] = 1;
		}
	}

	signals
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ValueMomentumPatternConfig {
	pub pfcf_threshold: Option<f64>,
	pub pattern_min_distance: Option<u32>,
	pub pattern_tolerance: Option<f64>,
}

impl Default for ValueMomentumPatternConfig {
	fn default() -> Self {
		Self {
			pfcf_threshold: Some(15.0),
			pattern_min_distance: Some(5),
			pattern_tolerance: Some(0.02),
		}
	}
}

/// Value Momentum Pattern
///
/// Generates buy/sell signals combining quantitative and fundamental factors.
pub fn value_momentum_pattern_strategy(
	fundamentals: Vec<FundamentalPoint>,
	prices: Vec<Bar>,
	config: Option<ValueMomentumPatternConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let pfcf_thresh = cfg.pfcf_threshold.unwrap_or(15.0);
	let min_distance = cfg.pattern_min_distance.unwrap_or(5);
	let tolerance = cfg.pattern_tolerance.unwrap_or(0.02);

	if fundamentals.is_empty() || prices.is_empty() {
		return vec![0; prices.len()];
	}

	let pfcf = price_to_free_cash_flow(fundamentals);

	let opens = opens_from_bars(&prices);
	let highs = highs_from_bars(&prices);
	let lows = lows_from_bars(&prices);
	let closes = prices_from_bars(&prices);

	let hns = match indicators_core::head_and_shoulders(
		&opens,
		&highs,
		&lows,
		&closes,
		Some(min_distance),
		Some(tolerance),
		Some(0.005),
	) {
		Ok(h) => h,
		Err(_) => return vec![0; prices.len()],
	};

	let times = times_from_bars(&prices);
	let mut signals = vec![0i8; prices.len()];

	for i in 0..prices.len() {
		let t = times[i];
		let pfcf_val = latest_factor_at_or_before(&pfcf, t).unwrap_or(f64::MAX);
		let pfcf_ok = pfcf_val < pfcf_thresh;

		let inverse_hs = i < hns.len() && (hns[i] - 1.0).abs() < 0.5;

		if pfcf_ok && inverse_hs {
			signals[i] = 1;
		}
	}

	signals
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GrowthQualityConfig {
	pub min_eps_growth: Option<f64>,
	pub min_roe: Option<f64>,
	pub min_rsi: Option<f64>,
	pub rsi_period: Option<u32>,
}

impl Default for GrowthQualityConfig {
	fn default() -> Self {
		Self {
			min_eps_growth: Some(0.1),
			min_roe: Some(0.15),
			min_rsi: Some(50.0),
			rsi_period: Some(14),
		}
	}
}

/// Growth Quality
///
/// Generates buy/sell signals combining quantitative and fundamental factors.
pub fn growth_quality_strategy(
	fundamentals: Vec<FundamentalPoint>,
	prices: Vec<Bar>,
	config: Option<GrowthQualityConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let min_eps = cfg.min_eps_growth.unwrap_or(0.1);
	let min_roe = cfg.min_roe.unwrap_or(0.15);
	let min_rsi = cfg.min_rsi.unwrap_or(50.0);
	let rsi_period = cfg.rsi_period.unwrap_or(14);

	if fundamentals.is_empty() || prices.is_empty() {
		return vec![0; prices.len()];
	}

	let eps_growth = eps_growth_qo_q(fundamentals.clone());
	let roe = return_on_equity(fundamentals);

	let closes = prices_from_bars(&prices);
	let rsi_vals = indicators_core::rsi(
		&closes,
		Some(indicators_core::RSIConfig {
			period: Some(rsi_period),
		}),
	);

	let times = times_from_bars(&prices);
	let mut signals = vec![0i8; prices.len()];

	for i in 0..prices.len() {
		let t = times[i];
		let eps_ok = latest_factor_at_or_before(&eps_growth, t).unwrap_or(0.0) > min_eps;
		let roe_ok = latest_factor_at_or_before(&roe, t).unwrap_or(0.0) > min_roe;
		let rsi_ok = i < rsi_vals.len() && rsi_vals[i] > min_rsi;

		if eps_ok && roe_ok && rsi_ok {
			signals[i] = 1;
		}
	}

	signals
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CompositeValueMomentumConfig {
	pub pe_threshold: Option<f64>,
	pub rsi_threshold: Option<f64>,
	pub rsi_period: Option<u32>,
	pub ma_fast_period: Option<u32>,
	pub ma_slow_period: Option<u32>,
}

impl Default for CompositeValueMomentumConfig {
	fn default() -> Self {
		Self {
			pe_threshold: Some(20.0),
			rsi_threshold: Some(50.0),
			rsi_period: Some(14),
			ma_fast_period: Some(10),
			ma_slow_period: Some(30),
		}
	}
}

/// Composite Value Momentum
///
/// Generates buy/sell signals combining quantitative and fundamental factors.
pub fn composite_value_momentum_strategy(
	fundamentals: Vec<FundamentalPoint>,
	prices: Vec<Bar>,
	config: Option<CompositeValueMomentumConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let pe_thresh = cfg.pe_threshold.unwrap_or(20.0);
	let rsi_thresh = cfg.rsi_threshold.unwrap_or(50.0);
	let rsi_period = cfg.rsi_period.unwrap_or(14);
	let fast_period = cfg.ma_fast_period.unwrap_or(10) as usize;
	let slow_period = cfg.ma_slow_period.unwrap_or(30) as usize;

	if fundamentals.is_empty() || prices.is_empty() {
		return vec![0; prices.len()];
	}

	let pe = price_to_earnings(fundamentals, prices.clone());

	let closes = prices_from_bars(&prices);
	let rsi_vals = indicators_core::rsi(
		&closes,
		Some(indicators_core::RSIConfig {
			period: Some(rsi_period),
		}),
	);
	let sma_fast = match indicators_core::sma(&closes, Some(fast_period as u32)) {
		Ok(s) => s,
		Err(_) => return vec![0; prices.len()],
	};
	let sma_slow = match indicators_core::sma(&closes, Some(slow_period as u32)) {
		Ok(s) => s,
		Err(_) => return vec![0; prices.len()],
	};

	let times = times_from_bars(&prices);
	let mut signals = vec![0i8; prices.len()];

	for i in 0..prices.len() {
		let t = times[i];
		let pe_ok = latest_factor_at_or_before(&pe, t).unwrap_or(f64::MAX) < pe_thresh;
		let rsi_ok = i < rsi_vals.len() && rsi_vals[i] > rsi_thresh;

		let ma_cross = if i >= slow_period {
			sma_fast[i] > sma_slow[i] && sma_fast[i - 1] <= sma_slow[i - 1]
		} else {
			false
		};

		if pe_ok && rsi_ok && ma_cross {
			signals[i] = 1;
		}
	}

	signals
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct QuantamentalValueMomentumConfig {
	pub pe_threshold: Option<f64>,
	pub sma_period: Option<u32>,
}

impl Default for QuantamentalValueMomentumConfig {
	fn default() -> Self {
		Self {
			pe_threshold: Some(20.0),
			sma_period: Some(50),
		}
	}
}

/// Quantamental Value Momentum
///
/// Generates buy/sell signals combining quantitative and fundamental factors.
pub fn quantamental_value_momentum_strategy(
	fundamentals: Vec<FundamentalPoint>,
	prices: Vec<Bar>,
	config: Option<QuantamentalValueMomentumConfig>,
) -> Vec<i8> {
	let cfg = config.unwrap_or_default();
	let pe_thresh = cfg.pe_threshold.unwrap_or(20.0);
	let sma_period = cfg.sma_period.unwrap_or(50) as usize;

	if fundamentals.is_empty() || prices.is_empty() {
		return vec![0; prices.len()];
	}

	let pe = price_to_earnings(fundamentals, prices.clone());

	let closes = prices_from_bars(&prices);
	let sma = match indicators_core::sma(&closes, Some(sma_period as u32)) {
		Ok(s) => s,
		Err(_) => return vec![0; prices.len()],
	};

	let times = times_from_bars(&prices);
	let mut signals = vec![0i8; prices.len()];

	for i in 0..prices.len() {
		let t = times[i];
		let pe_ok = latest_factor_at_or_before(&pe, t).unwrap_or(f64::MAX) < pe_thresh;

		let above_sma = if i >= sma_period {
			prices[i].close > sma[i]
		} else {
			false
		};

		if pe_ok && above_sma {
			signals[i] = 1;
		}
	}

	signals
}

pub fn qarp_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "qarp",
		"name": "Quality at Reasonable Price",
		"category": "quantamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Combines quality filters (ROE>15%, D/E<1.0, PE<20) with Donchian breakout entry"
	})
}

pub fn multi_factor_value_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "multi-factor-value",
		"name": "Multi-Factor Value Strategy",
		"category": "quantamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Combines P/FCF<15, PE<15, PB<1.5 with SuperTrend bullish confirmation, requiredFactors:1-3"
	})
}

pub fn alternative_data_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "alternative-data",
		"name": "Alternative Data Strategy",
		"category": "quantamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Combines exchange flow momentum with prediction odds momentum signals"
	})
}

pub fn event_driven_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "event-driven",
		"name": "Event Driven Strategy",
		"category": "quantamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Enters when prediction market odds>0.7 and price is above SMA(20)"
	})
}

pub fn on_chain_confirmation_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "on-chain-confirmation",
		"name": "On-Chain Confirmation",
		"category": "quantamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Uses EMA(12/26) crossover with exchange net outflow threshold confirmation"
	})
}

pub fn value_momentum_pattern_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "value-momentum-pattern",
		"name": "Value + Momentum Pattern Strategy",
		"category": "quantamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Combines P/FCF<15 threshold with Inverse Head and Shoulders pattern detection"
	})
}

pub fn growth_quality_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "growth-quality",
		"name": "Growth Quality Strategy",
		"category": "quantamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Combines EPS growth QoQ>10%, ROE>15%, and RSI>50 momentum confirmation"
	})
}

pub fn composite_value_momentum_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "composite-value-momentum",
		"name": "Composite Value Momentum",
		"category": "quantamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Combines PE<20, RSI>50, and SMA crossover(10/30) for entry"
	})
}

pub fn quantamental_value_momentum_strategy_metadata() -> serde_json::Value {
	serde_json::json!({
		"id": "quantamental-value-momentum",
		"name": "Quantamental Value Momentum",
		"category": "quantamental",
		"default_timeframes": ["1d", "1w"],
		"description": "Combines PE<20 threshold with price above SMA(50) for entry"
	})
}

pub fn qarp_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"roeThreshold": 0.15,
			"deThreshold": 1.0,
			"peThreshold": 20.0,
			"donchianPeriod": 20
		},
		"optimization_bounds": [
			{"param_name": "roeThreshold", "min": 0.05, "max": 0.3, "step": 0.01},
			{"param_name": "deThreshold", "min": 0.5, "max": 2.0, "step": 0.1},
			{"param_name": "peThreshold", "min": 10.0, "max": 30.0, "step": 1.0},
			{"param_name": "donchianPeriod", "min": 5.0, "max": 50.0, "step": 1.0}
		]
	})
}

pub fn multi_factor_value_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"pfcfThreshold": 15.0,
			"peThreshold": 15.0,
			"pbThreshold": 1.5,
			"superTrendPeriod": 14,
			"superTrendMultiplier": 3.0,
			"requiredFactors": 2
		},
		"optimization_bounds": [
			{"param_name": "pfcfThreshold", "min": 5.0, "max": 30.0, "step": 1.0},
			{"param_name": "peThreshold", "min": 5.0, "max": 30.0, "step": 1.0},
			{"param_name": "pbThreshold", "min": 0.5, "max": 3.0, "step": 0.1},
			{"param_name": "requiredFactors", "min": 1.0, "max": 3.0, "step": 1.0}
		]
	})
}

pub fn alternative_data_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"exchangeThreshold": 0.05,
			"exchangePeriod": 7,
			"oddsThreshold": 0.02,
			"oddsPeriod": 3
		},
		"optimization_bounds": [
			{"param_name": "exchangeThreshold", "min": 0.01, "max": 0.2, "step": 0.01},
			{"param_name": "exchangePeriod", "min": 3.0, "max": 30.0, "step": 1.0},
			{"param_name": "oddsThreshold", "min": 0.01, "max": 0.1, "step": 0.01},
			{"param_name": "oddsPeriod", "min": 1.0, "max": 10.0, "step": 1.0}
		]
	})
}

pub fn event_driven_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"oddsThreshold": 0.7,
			"smaPeriod": 20
		},
		"optimization_bounds": [
			{"param_name": "oddsThreshold", "min": 0.3, "max": 0.95, "step": 0.05},
			{"param_name": "smaPeriod", "min": 5.0, "max": 100.0, "step": 5.0}
		]
	})
}

pub fn on_chain_confirmation_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"fastEma": 12,
			"slowEma": 26,
			"netflowThreshold": 0.1
		},
		"optimization_bounds": [
			{"param_name": "fastEma", "min": 5.0, "max": 50.0, "step": 1.0},
			{"param_name": "slowEma", "min": 10.0, "max": 100.0, "step": 1.0},
			{"param_name": "netflowThreshold", "min": 0.01, "max": 0.5, "step": 0.01}
		]
	})
}

pub fn value_momentum_pattern_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"pfcfThreshold": 15.0,
			"patternMinDistance": 5,
			"patternTolerance": 0.02
		},
		"optimization_bounds": [
			{"param_name": "pfcfThreshold", "min": 5.0, "max": 25.0, "step": 1.0},
			{"param_name": "patternMinDistance", "min": 3.0, "max": 10.0, "step": 1.0},
			{"param_name": "patternTolerance", "min": 0.005, "max": 0.05, "step": 0.005}
		]
	})
}

pub fn growth_quality_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"minEpsGrowth": 0.1,
			"minRoe": 0.15,
			"minRsi": 50.0,
			"rsiPeriod": 14
		},
		"optimization_bounds": [
			{"param_name": "minEpsGrowth", "min": 0.05, "max": 0.3, "step": 0.01},
			{"param_name": "minRoe", "min": 0.1, "max": 0.25, "step": 0.01},
			{"param_name": "minRsi", "min": 30.0, "max": 70.0, "step": 5.0},
			{"param_name": "rsiPeriod", "min": 7.0, "max": 21.0, "step": 1.0}
		]
	})
}

pub fn composite_value_momentum_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"peThreshold": 20.0,
			"rsiThreshold": 50.0,
			"rsiPeriod": 14,
			"maFastPeriod": 10,
			"maSlowPeriod": 30
		},
		"optimization_bounds": [
			{"param_name": "peThreshold", "min": 5.0, "max": 30.0, "step": 1.0},
			{"param_name": "rsiThreshold", "min": 30.0, "max": 70.0, "step": 5.0},
			{"param_name": "rsiPeriod", "min": 7.0, "max": 21.0, "step": 1.0},
			{"param_name": "maFastPeriod", "min": 5.0, "max": 30.0, "step": 1.0},
			{"param_name": "maSlowPeriod", "min": 10.0, "max": 100.0, "step": 5.0}
		]
	})
}

pub fn quantamental_value_momentum_strategy_defaults() -> serde_json::Value {
	serde_json::json!({
		"params": {
			"peThreshold": 20.0,
			"smaPeriod": 50
		},
		"optimization_bounds": [
			{"param_name": "peThreshold", "min": 5.0, "max": 30.0, "step": 1.0},
			{"param_name": "smaPeriod", "min": 10.0, "max": 200.0, "step": 5.0}
		]
	})
}

#[cfg(test)]
mod defaults_tests {
	use super::*;

	macro_rules! check_defaults {
		($($defaults_fn:ident => $cfg:ty),* $(,)?) => {
			$(
				#[test]
				fn $defaults_fn() {
					let defaults = super::$defaults_fn();
					let params = defaults["params"].clone();
					let cfg: $cfg = serde_json::from_value(params.clone())
						.expect("defaults params must deserialize");
					let canonical = serde_json::to_value(&cfg).unwrap();
					for (k, v) in params.as_object().unwrap() {
						let expected = canonical.get(k).unwrap_or(&serde_json::Value::Null);
						let matches = match (expected.as_f64(), v.as_f64()) {
							(Some(a), Some(b)) => a == b,
							_ => expected == v,
						};
						assert!(
							matches,
							"key `{k}` is not a recognized field of {}",
							stringify!($cfg)
						);
					}
				}
			)*
		};
	}

	check_defaults! {
		qarp_strategy_defaults => QarpConfig,
		multi_factor_value_strategy_defaults => MultiFactorValueConfig,
		alternative_data_strategy_defaults => AlternativeDataConfig,
		event_driven_strategy_defaults => EventDrivenConfig,
		on_chain_confirmation_strategy_defaults => OnChainConfirmationConfig,
		value_momentum_pattern_strategy_defaults => ValueMomentumPatternConfig,
		growth_quality_strategy_defaults => GrowthQualityConfig,
		composite_value_momentum_strategy_defaults => CompositeValueMomentumConfig,
		quantamental_value_momentum_strategy_defaults => QuantamentalValueMomentumConfig,
	}
}
