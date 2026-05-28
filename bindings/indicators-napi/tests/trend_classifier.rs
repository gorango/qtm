use indicators::classify_market_trend;
use indicators_core::{get_trend_volatility, Bar, TrendAnalysis, TrendVolatility};

fn create_market_data(prices: Vec<f64>) -> Vec<Bar> {
	prices
		.into_iter()
		.map(|price| Bar {
			timestamp: None,
			open: price,
			high: price * 1.01, // Slight high
			low: price * 0.99,  // Slight low
			close: price,
			volume: Some(1000.0),
		})
		.collect()
}

#[test]
fn test_trend_volatility_enum_values() {
	// Since it's an enum with serde rename, we can check string representations if needed
	// But for simplicity, test that the variants exist and match expected strings via serde
	let json = serde_json::to_string(&TrendVolatility::BullishLow).unwrap();
	assert_eq!(json, "\"bullish-low\"");

	let json = serde_json::to_string(&TrendVolatility::BullishMedium).unwrap();
	assert_eq!(json, "\"bullish-medium\"");

	let json = serde_json::to_string(&TrendVolatility::BullishHigh).unwrap();
	assert_eq!(json, "\"bullish-high\"");

	let json = serde_json::to_string(&TrendVolatility::BearishLow).unwrap();
	assert_eq!(json, "\"bearish-low\"");

	let json = serde_json::to_string(&TrendVolatility::BearishMedium).unwrap();
	assert_eq!(json, "\"bearish-medium\"");

	let json = serde_json::to_string(&TrendVolatility::BearishHigh).unwrap();
	assert_eq!(json, "\"bearish-high\"");

	let json = serde_json::to_string(&TrendVolatility::SidewaysLow).unwrap();
	assert_eq!(json, "\"sideways-low\"");

	let json = serde_json::to_string(&TrendVolatility::SidewaysMedium).unwrap();
	assert_eq!(json, "\"sideways-medium\"");

	let json = serde_json::to_string(&TrendVolatility::SidewaysHigh).unwrap();
	assert_eq!(json, "\"sideways-high\"");
}

#[test]
fn test_classify_market_trend_insufficient_data() {
	let market_data = create_market_data(vec![100.0, 101.0, 102.0]); // Only 3 bars

	let result = classify_market_trend(market_data, None);

	assert_eq!(result.trend, "sideways");
	assert_eq!(result.volatility, "medium");
	assert_eq!(result.regime, TrendVolatility::SidewaysMedium);
	assert_eq!(result.change, 0.0);
	assert_eq!(result.intensity, 0.5);
	assert_eq!(result.confidence, 0.3);
}

#[test]
fn test_classify_market_trend_bullish_strong_gain() {
	let prices: Vec<f64> = (0..20).map(|i| 100.0 + i as f64 * 2.0).collect(); // 40% gain over 20 days
	let market_data = create_market_data(prices);

	let result = classify_market_trend(market_data, None);

	assert_eq!(result.trend, "bullish");
	assert_eq!(result.regime, TrendVolatility::BullishLow); // Steady gain has low volatility
	assert!(result.change > 0.0);
	assert!(result.confidence > 0.5);
}

#[test]
fn test_classify_market_trend_bearish_strong_loss() {
	let prices: Vec<f64> = (0..20).map(|i| 100.0 - i as f64 * 2.0).collect(); // 40% loss over 20 days
	let market_data = create_market_data(prices);

	let result = classify_market_trend(market_data, None);

	assert_eq!(result.trend, "bearish");
	assert_eq!(result.regime, TrendVolatility::BearishLow);
	assert!(result.change < 0.0);
	assert!(result.confidence > 0.5);
}

#[test]
fn test_classify_market_trend_sideways_minimal_movement() {
	let prices: Vec<f64> = (0..20).map(|_| 100.0).collect(); // No variation
	let market_data = create_market_data(prices);

	let result = classify_market_trend(market_data, None);

	assert_eq!(result.trend, "sideways");
	assert_eq!(result.regime, TrendVolatility::SidewaysLow); // Low volatility for no movement
	assert!(result.confidence > 0.7);
}

#[test]
fn test_classify_market_trend_low_volatility_stable_prices() {
	let prices: Vec<f64> = (0..20).map(|_| 100.0).collect(); // No variation
	let market_data = create_market_data(prices);

	let result = classify_market_trend(market_data, None);

	assert_eq!(result.volatility, "low");
	assert_eq!(result.regime, TrendVolatility::SidewaysLow);
	assert!(result.confidence >= 0.8);
}

#[test]
fn test_classify_market_trend_medium_volatility_moderate_variation() {
	let prices = vec![
		100.0, 101.0, 99.0, 100.5, 99.5, 101.2, 98.8, 100.8, 99.2, 101.5, 98.5, 100.3, 99.7, 101.8,
		98.2, 100.1, 99.9, 101.3, 98.7, 100.4,
	];
	let market_data = create_market_data(prices);

	let result = classify_market_trend(market_data, None);

	assert_eq!(result.volatility, "medium");
	assert_eq!(result.regime, TrendVolatility::SidewaysMedium);
	assert!(result.confidence > 0.7);
}

#[test]
fn test_classify_market_trend_high_volatility_large_variation() {
	let prices = vec![
		100.0, 110.0, 95.0, 105.0, 90.0, 115.0, 85.0, 120.0, 80.0, 125.0, 75.0, 130.0, 70.0, 135.0,
		65.0, 140.0, 60.0, 145.0, 55.0, 150.0,
	];
	let market_data = create_market_data(prices);

	let result = classify_market_trend(market_data, None);

	assert_eq!(result.volatility, "high");
	assert_eq!(result.regime, TrendVolatility::SidewaysHigh);
	assert!(result.confidence > 0.5);
}

#[test]
fn test_classify_market_trend_confidence_range() {
	let prices: Vec<f64> = (0..20).map(|i| 100.0 + i as f64 * 0.1).collect(); // Small gradual increase
	let market_data = create_market_data(prices);

	let result = classify_market_trend(market_data, None);

	assert!(result.confidence >= 0.0);
	assert!(result.confidence <= 1.0);
}

#[test]
fn test_classify_market_trend_empty_array() {
	let result = classify_market_trend(vec![], None);

	assert_eq!(result.trend, "sideways");
	assert_eq!(result.volatility, "medium");
	assert_eq!(result.regime, TrendVolatility::SidewaysMedium);
	assert_eq!(result.change, 0.0);
	assert_eq!(result.intensity, 0.5);
	assert_eq!(result.confidence, 0.3);
}

#[test]
fn test_get_trend_volatility() {
	let analysis = TrendAnalysis {
		regime: TrendVolatility::BullishLow,
		trend: "bullish".to_string(),
		change: 0.1,
		volatility: "low".to_string(),
		intensity: 0.2,
		confidence: 0.9,
	};

	let regime = get_trend_volatility(analysis);
	assert_eq!(regime, TrendVolatility::BullishLow);
}
