use factors::{
	active_address_growth, exchange_flow_momentum, nvt_ratio, odds_momentum,
	prediction_market_odds, staking_ratio, OnChainDataPoint, PredictionMarketPoint,
};

fn make_oc(time: f64, metric: &str, value: f64) -> OnChainDataPoint {
	OnChainDataPoint {
		time,
		metric: metric.to_string(),
		value,
	}
}

fn make_pred(time: f64, market_id: &str, price: f64, volume: f64) -> PredictionMarketPoint {
	PredictionMarketPoint {
		time,
		market_id: market_id.to_string(),
		price,
		volume,
	}
}

// --- On-Chain ---

#[test]
fn test_nvt_ratio_happy_path() {
	let data = vec![
		make_oc(100.0, "marketCap", 1000.0),
		make_oc(100.0, "transactionVolume", 200.0),
		make_oc(200.0, "marketCap", 1100.0),
		make_oc(200.0, "transactionVolume", 250.0),
	];
	let result = nvt_ratio(data);
	assert_eq!(result.len(), 2);
	assert!((result[0].value - 5.0).abs() < 1e-6);
	assert!((result[1].value - 4.4).abs() < 1e-6);
}

#[test]
fn test_nvt_ratio_skips_zero_volume() {
	let data = vec![
		make_oc(100.0, "marketCap", 1000.0),
		make_oc(100.0, "transactionVolume", 0.0),
	];
	let result = nvt_ratio(data);
	assert!(result.is_empty());
}

#[test]
fn test_nvt_ratio_no_matching_timestamps() {
	let data = vec![
		make_oc(100.0, "marketCap", 1000.0),
		make_oc(200.0, "transactionVolume", 500.0),
	];
	let result = nvt_ratio(data);
	assert!(result.is_empty());
}

#[test]
fn test_active_address_growth_happy_path() {
	let data = vec![
		make_oc(100.0, "activeAddresses", 1000.0),
		make_oc(200.0, "activeAddresses", 1100.0),
	];
	let result = active_address_growth(data, Some(30.0));
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 0.1).abs() < 1e-6);
}

#[test]
fn test_active_address_growth_skips_non_active_metric() {
	let data = vec![
		make_oc(100.0, "otherMetric", 1000.0),
		make_oc(200.0, "otherMetric", 1100.0),
	];
	let result = active_address_growth(data, Some(30.0));
	assert!(result.is_empty());
}

#[test]
fn test_exchange_flow_momentum_happy_path() {
	let data = vec![
		make_oc(100.0, "exchangeNetflow", 500.0),
		make_oc(200.0, "exchangeNetflow", 550.0),
	];
	let result = exchange_flow_momentum(data, Some(30.0));
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 0.1).abs() < 1e-6);
}

#[test]
fn test_staking_ratio_happy_path() {
	let data = vec![
		make_oc(100.0, "stakedSupply", 300.0),
		make_oc(100.0, "totalSupply", 1000.0),
	];
	let result = staking_ratio(data);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 0.3).abs() < 1e-6);
}

// --- Prediction ---

#[test]
fn test_prediction_market_odds_happy_path() {
	let data = vec![
		make_pred(100.0, "president2028", 0.65, 1_000_000.0),
		make_pred(200.0, "president2028", 0.70, 1_200_000.0),
	];
	let result = prediction_market_odds(data);
	assert_eq!(result.len(), 2);
	assert!((result[0].value - 0.65).abs() < 1e-6);
	assert!((result[1].value - 0.70).abs() < 1e-6);
}

#[test]
fn test_prediction_market_odds_multiple_markets() {
	let data = vec![
		make_pred(100.0, "market1", 0.5, 1000.0),
		make_pred(100.0, "market2", 0.3, 2000.0),
	];
	let result = prediction_market_odds(data);
	assert_eq!(result.len(), 2);
}

#[test]
fn test_odds_momentum_happy_path() {
	let data = vec![
		make_pred(100.0, "president2028", 0.50, 1_000_000.0),
		make_pred(200.0, "president2028", 0.60, 1_200_000.0),
	];
	let result = odds_momentum(data, Some(1));
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 0.2).abs() < 1e-6);
}

#[test]
fn test_odds_momentum_no_movement() {
	let data = vec![
		make_pred(100.0, "market", 0.50, 1000.0),
		make_pred(200.0, "market", 0.50, 1000.0),
	];
	let result = odds_momentum(data, Some(1));
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 0.0).abs() < 1e-6);
}

#[test]
fn test_odds_momentum_insufficient_data() {
	let data = vec![make_pred(100.0, "market", 0.50, 1000.0)];
	let result = odds_momentum(data, Some(1));
	assert!(result.is_empty());
}

#[test]
fn test_empty_data() {
	assert!(nvt_ratio(vec![]).is_empty());
	assert!(active_address_growth(vec![], None).is_empty());
	assert!(prediction_market_odds(vec![]).is_empty());
	assert!(odds_momentum(vec![], None).is_empty());
}
