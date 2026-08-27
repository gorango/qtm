use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Bar {
	pub timestamp: Option<f64>,
	pub open: f64,
	pub high: f64,
	pub low: f64,
	pub close: f64,
	pub volume: Option<f64>,
}

#[cfg_attr(feature = "napi", napi_derive::napi)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TrendVolatility {
	#[serde(rename = "bullish-low")]
	BullishLow,
	#[serde(rename = "bullish-medium")]
	BullishMedium,
	#[serde(rename = "bullish-high")]
	BullishHigh,
	#[serde(rename = "bearish-low")]
	BearishLow,
	#[serde(rename = "bearish-medium")]
	BearishMedium,
	#[serde(rename = "bearish-high")]
	BearishHigh,
	#[serde(rename = "sideways-low")]
	SidewaysLow,
	#[serde(rename = "sideways-medium")]
	SidewaysMedium,
	#[serde(rename = "sideways-high")]
	SidewaysHigh,
}

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TrendAnalysis {
	/// Combined regime enum.
	pub regime: TrendVolatility,
	/// Trend label: `bullish` | `bearish` | `sideways`.
	pub trend: String,
	/// Total price change `(last-first)/first`.
	pub change: f64,
	/// Volatility label: `low` | `medium` | `high`.
	pub volatility: String,
	/// Z-score derived intensity clamped to [0, 1].
	pub intensity: f64,
	/// Combined trend × volatility confidence in [0, 1].
	pub confidence: f64,
}

/// Classifies market regime from a slice of bars.
///
/// Computes full-sample return/volatility to get a z-score for trend, and trailing-window
/// volatility for the vol bucket. Returns `TrendVolatility` and confidence scores.
/// Heuristic; no single canonical definition. Used for labeling only.
pub fn classify_market_trend(
	market_data: Vec<Bar>,
	trailing_period_length: Option<u32>,
) -> TrendAnalysis {
	if market_data.len() < 10 {
		return TrendAnalysis {
			trend: "sideways".to_string(),
			volatility: "medium".to_string(),
			regime: TrendVolatility::SidewaysMedium,
			change: 0.0,
			intensity: 0.5,
			confidence: 0.3,
		};
	}

	let prices: Vec<f64> = market_data.iter().map(|bar| bar.close).collect();
	let bars_count = prices.len();

	let mut full_returns: Vec<f64> = Vec::new();
	for i in 1..prices.len() {
		full_returns.push((prices[i] - prices[i - 1]) / prices[i - 1]);
	}
	let full_avg_return = full_returns.iter().sum::<f64>() / full_returns.len() as f64;
	let full_variance = full_returns
		.iter()
		.map(|&r| (r - full_avg_return).powi(2))
		.sum::<f64>()
		/ full_returns.len() as f64;
	let full_vol = full_variance.sqrt();

	let start_price = prices[0];
	let end_price = prices[prices.len() - 1];
	let price_change = (end_price - start_price) / start_price;

	let expected_range = full_vol * (bars_count as f64).sqrt();
	let safe_range = if expected_range == 0.0 {
		0.0001
	} else {
		expected_range
	};
	let z_score = price_change / safe_range;

	let (trend, trend_confidence) = if z_score.abs() < 1.0 {
		("sideways".to_string(), 1.0 - z_score.abs())
	} else {
		let trend_str = if z_score > 0.0 { "bullish" } else { "bearish" };
		let raw_conf = 0.5 + (z_score.abs() - 1.0) / 4.0;
		(trend_str.to_string(), raw_conf.min(1.0))
	};

	let default_trailing = (market_data.len() as f64 * 0.25).floor() as usize;
	let trailing_len = trailing_period_length
		.map(|p| p as usize)
		.unwrap_or(default_trailing.max(10));
	let trailing_prices = prices[prices.len().saturating_sub(trailing_len)..].to_vec();

	if trailing_prices.len() < 2 {
		return TrendAnalysis {
			trend: "sideways".to_string(),
			volatility: "medium".to_string(),
			regime: TrendVolatility::SidewaysMedium,
			change: 0.0,
			intensity: 0.5,
			confidence: 0.3,
		};
	}

	let mut trailing_returns: Vec<f64> = Vec::new();
	for i in 1..trailing_prices.len() {
		trailing_returns
			.push((trailing_prices[i] - trailing_prices[i - 1]) / trailing_prices[i - 1]);
	}
	let trailing_avg_return = trailing_returns.iter().sum::<f64>() / trailing_returns.len() as f64;
	let trailing_variance = trailing_returns
		.iter()
		.map(|&r| (r - trailing_avg_return).powi(2))
		.sum::<f64>()
		/ trailing_returns.len() as f64;
	let trailing_vol = trailing_variance.sqrt();

	let ratio = if full_vol == 0.0 {
		0.0
	} else {
		trailing_vol / full_vol
	};
	let (volatility, volatility_confidence) = if ratio < 0.7 {
		("low".to_string(), (1.0 - ratio).min(1.0) * 2.0)
	} else if ratio < 1.3 {
		("medium".to_string(), 0.8)
	} else {
		("high".to_string(), (ratio * 0.5).min(1.0))
	};

	let intensity = calculate_volatility_intensity(trailing_vol, full_vol);
	let regime = match (trend.as_str(), volatility.as_str()) {
		("bullish", "low") => TrendVolatility::BullishLow,
		("bullish", "medium") => TrendVolatility::BullishMedium,
		("bullish", "high") => TrendVolatility::BullishHigh,
		("bearish", "low") => TrendVolatility::BearishLow,
		("bearish", "medium") => TrendVolatility::BearishMedium,
		("bearish", "high") => TrendVolatility::BearishHigh,
		("sideways", "low") => TrendVolatility::SidewaysLow,
		("sideways", "medium") => TrendVolatility::SidewaysMedium,
		("sideways", "high") => TrendVolatility::SidewaysHigh,
		_ => TrendVolatility::SidewaysMedium,
	};
	let overall_confidence = trend_confidence.min(volatility_confidence);

	TrendAnalysis {
		trend,
		volatility,
		regime,
		change: price_change,
		intensity,
		confidence: overall_confidence,
	}
}

fn calculate_volatility_intensity(trailing_vol: f64, full_vol: f64) -> f64 {
	let ratio = if full_vol == 0.0 {
		0.0
	} else {
		trailing_vol / full_vol
	};
	let min_ratio = 0.1;
	let max_ratio = 3.0;
	((ratio - min_ratio) / (max_ratio - min_ratio)).clamp(0.0, 1.0)
}

pub fn get_trend_volatility(analysis: TrendAnalysis) -> TrendVolatility {
	analysis.regime
}
