use crate::types::data::{Bar, FactorPoint, FundamentalPoint};
use crate::utils::pricing::find_price_on_or_after;

/// Price to AFFO (Adjusted Funds From Operations): `price / affoPerShare`.
/// Uses closest price on/after filing date.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn price_to_affo(fundamentals: Vec<FundamentalPoint>, prices: Vec<Bar>) -> Vec<FactorPoint> {
	let mut prices_sorted = prices.clone();
	prices_sorted.sort_by(|a, b| {
		a.time
			.partial_cmp(&b.time)
			.unwrap_or(std::cmp::Ordering::Equal)
	});

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
			symbol: f.symbol.clone(),
			date: f.filing_date,
			value: price / affo,
		});
	}
	results
}

/// REIT Dividend Safety: `forwardAnnualDividendRate / affoPerShare` — the AFFO
/// payout ratio. Lower means the dividend is better covered by cash earnings.
/// `affoPerShare` is annual, matching `price_to_affo`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn reit_dividend_safety(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let div_rate = match f.data.forward_annual_dividend_rate {
			Some(v) => v,
			None => continue,
		};
		let affo = match f.data.affo_per_share {
			Some(v) if v > 0.0 => v,
			_ => continue,
		};
		results.push(FactorPoint {
			symbol: f.symbol.clone(),
			date: f.filing_date,
			value: div_rate / affo,
		});
	}
	results
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::types::data::FundamentalPointData;

	fn fp(affo: f64, dividend: f64) -> FundamentalPoint {
		FundamentalPoint {
			symbol: "PLD".to_string(),
			date: 1.0,
			filing_date: 1.0,
			period: "Q1".to_string(),
			data: FundamentalPointData {
				affo_per_share: Some(affo),
				forward_annual_dividend_rate: Some(dividend),
				..Default::default()
			},
		}
	}

	fn bar(time: f64) -> Bar {
		Bar {
			time,
			open: 1.0,
			high: 1.0,
			low: 1.0,
			close: 1.0,
			volume: 1.0,
		}
	}

	#[test]
	fn dividend_safety_is_the_affo_payout_ratio() {
		let out = reit_dividend_safety(vec![fp(2.3, 6.0)]);
		assert_eq!(out.len(), 1);
		assert!((out[0].value - 6.0 / 2.3).abs() < 1e-9);
	}

	#[test]
	fn dividend_safety_skips_non_positive_affo() {
		assert!(reit_dividend_safety(vec![fp(0.0, 6.0)]).is_empty());
		assert!(reit_dividend_safety(vec![fp(-1.0, 6.0)]).is_empty());
	}

	#[test]
	fn dividend_safety_requires_both_inputs() {
		let mut missing = fp(2.3, 6.0);
		missing.data.forward_annual_dividend_rate = None;
		assert!(reit_dividend_safety(vec![missing]).is_empty());
	}

	#[test]
	fn price_to_affo_uses_annual_affo() {
		let out = price_to_affo(vec![fp(2.3, 0.0)], vec![bar(1.0)]);
		assert_eq!(out.len(), 1);
		assert!((out[0].value - 1.0 / 2.3).abs() < 1e-9);
	}
}
