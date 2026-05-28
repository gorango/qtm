use crate::types::data::{Bar, FactorPoint, FundamentalPoint};
use crate::utils::pricing::find_price_on_or_after;

/// Analyst Rating Momentum: `currentRating - avgPastRating` over `period` filings.
/// Positive values indicate improving analyst sentiment.
pub fn analyst_rating_momentum(
	fundamentals: Vec<FundamentalPoint>,
	period: Option<u32>,
) -> Vec<FactorPoint> {
	let p = period.unwrap_or(90) as usize;
	let mut results = Vec::new();

	let mut sorted = fundamentals.clone();
	sorted.sort_by(|a, b| a.filing_date.partial_cmp(&b.filing_date).unwrap());

	let mut symbol_map: std::collections::HashMap<String, Vec<&FundamentalPoint>> =
		std::collections::HashMap::new();
	for f in &sorted {
		symbol_map.entry(f.symbol.clone()).or_default().push(f);
	}

	for sfs in symbol_map.values_mut() {
		sfs.sort_by(|a, b| a.filing_date.partial_cmp(&b.filing_date).unwrap());
		for i in p..sfs.len() {
			let cur = sfs[i];
			let rating = match cur.data.rating {
				Some(v) => v,
				None => continue,
			};
			let past_ratings: Vec<f64> =
				sfs[i - p..i].iter().filter_map(|f| f.data.rating).collect();
			if past_ratings.is_empty() {
				continue;
			}
			let avg = past_ratings.iter().sum::<f64>() / past_ratings.len() as f64;
			results.push(FactorPoint {
				date: cur.filing_date,
				value: rating - avg,
			});
		}
	}
	results
}

/// Analyst Target Upside: `(targetPrice - price) / price`.
/// Uses closest price on/after filing date.
pub fn analyst_target_upside(
	fundamentals: Vec<FundamentalPoint>,
	prices: Vec<Bar>,
) -> Vec<FactorPoint> {
	let mut prices_sorted = prices.clone();
	prices_sorted.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

	let mut results = Vec::new();
	for f in &fundamentals {
		let target = match f.data.analyst_target_price {
			Some(v) => v,
			None => continue,
		};
		let price = match find_price_on_or_after(&prices_sorted, f.filing_date) {
			Some(p) if p > 0.0 => p,
			_ => continue,
		};
		results.push(FactorPoint {
			date: f.filing_date,
			value: (target - price) / price,
		});
	}
	results
}
