use crate::types::data::{FactorPoint, PredictionMarketPoint};

/// Prediction Market Odds: extracts implied probability `price` from each prediction market point.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn prediction_market_odds(prediction_data: Vec<PredictionMarketPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();

	let mut market_groups: std::collections::HashMap<String, Vec<&PredictionMarketPoint>> =
		std::collections::HashMap::new();
	for d in &prediction_data {
		market_groups
			.entry(d.market_id.clone())
			.or_default()
			.push(d);
	}

	for group in market_groups.values_mut() {
		group.sort_by(|a, b| {
			a.time
				.partial_cmp(&b.time)
				.unwrap_or(std::cmp::Ordering::Equal)
		});
		for point in group {
			results.push(FactorPoint {
				symbol: String::new(),
				date: point.time,
				value: point.price,
			});
		}
	}
	results
}

/// Prediction Market Odds Momentum: `(currentPrice - priceNPeriodsBack) / priceNPeriodsBack`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn odds_momentum(
	prediction_data: Vec<PredictionMarketPoint>,
	period: Option<u32>,
) -> Vec<FactorPoint> {
	let p = period.unwrap_or(1) as usize;
	let mut results = Vec::new();

	let mut market_groups: std::collections::HashMap<String, Vec<&PredictionMarketPoint>> =
		std::collections::HashMap::new();
	for d in &prediction_data {
		market_groups
			.entry(d.market_id.clone())
			.or_default()
			.push(d);
	}

	for group in market_groups.values_mut() {
		group.sort_by(|a, b| {
			a.time
				.partial_cmp(&b.time)
				.unwrap_or(std::cmp::Ordering::Equal)
		});
		if group.len() <= p {
			continue;
		}
		for i in p..group.len() {
			let cur = group[i];
			let prev = group[i - p];
			if prev.price != 0.0 {
				let momentum = (cur.price - prev.price) / prev.price;
				results.push(FactorPoint {
					symbol: String::new(),
					date: cur.time,
					value: momentum,
				});
			}
		}
	}
	results
}
