use crate::types::data::{Bar, FactorPoint, FundamentalPoint};
use crate::utils::pricing::find_price_on_or_after;

/// Price to AFFO (Adjusted Funds From Operations): `price / affoPerShare`.
/// Uses closest price on/after filing date.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn price_to_affo(fundamentals: Vec<FundamentalPoint>, prices: Vec<Bar>) -> Vec<FactorPoint> {
	let mut prices_sorted = prices.clone();
	prices_sorted.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

	let mut results = Vec::new();
	for f in &fundamentals {
		let affo = match f.data.affo_per_share {
			Some(v) => v,
			None => continue,
		};
		let price = match find_price_on_or_after(&prices_sorted, f.filing_date) {
			Some(p) if p > 0.0 => p,
			_ => continue,
		};
		results.push(FactorPoint {
			date: f.filing_date,
			value: price / affo,
		});
	}
	results
}

/// REIT Dividend Safety: `forwardAnnualDividendRate / (affoPerShare * 4)`.
/// Lower ratio indicates safer dividend coverage.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn reit_dividend_safety(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let div_rate = match f.data.forward_annual_dividend_rate {
			Some(v) => v,
			None => continue,
		};
		let affo = match f.data.affo_per_share {
			Some(v) => v,
			None => continue,
		};
		let quarterly_affo = affo * 4.0;
		if quarterly_affo <= 0.0 {
			continue;
		}
		results.push(FactorPoint {
			date: f.filing_date,
			value: div_rate / quarterly_affo,
		});
	}
	results
}
