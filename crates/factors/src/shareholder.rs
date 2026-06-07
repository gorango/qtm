use crate::types::data::{FactorPoint, FundamentalPoint};

pub fn shareholder_yield_value(d: &crate::types::data::FundamentalPointData) -> Option<f64> {
	let mcap = d.market_cap?;
	if mcap <= 0.0 {
		return None;
	}
	let shares = d.shares_outstanding?;
	if shares <= 0.0 {
		return None;
	}
	let dps = d.dividends_per_share?;
	Some((dps * shares) / mcap)
}

pub fn dividend_payout_ratio_value(d: &crate::types::data::FundamentalPointData) -> Option<f64> {
	let dps = d.dividends_per_share?;
	let eps = d.eps?;
	if eps <= 0.0 {
		return None;
	}
	Some(dps / eps)
}

pub fn dividend_coverage_ratio_value(d: &crate::types::data::FundamentalPointData) -> Option<f64> {
	let ni = d.net_income?;
	let dps = d.dividends_per_share?;
	let shares = d.shares_outstanding?;
	if shares <= 0.0 {
		return None;
	}
	let total_divs = dps * shares;
	if total_divs <= 0.0 {
		return None;
	}
	Some(ni / total_divs)
}

/// Shareholder Yield: `(dividendsPerShare * sharesOutstanding) / marketCap`.
/// Captures dividend component of total shareholder return.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn shareholder_yield(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(value) = shareholder_yield_value(&f.data) {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value,
			});
		}
	}
	results
}

/// Dividend Payout Ratio: `dividendsPerShare / eps`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn dividend_payout_ratio(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(value) = dividend_payout_ratio_value(&f.data) {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value,
			});
		}
	}
	results
}

/// Dividend Coverage Ratio: `netIncome / (dividendsPerShare * sharesOutstanding)`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn dividend_coverage_ratio(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(value) = dividend_coverage_ratio_value(&f.data) {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value,
			});
		}
	}
	results
}

/// Dividend Positive for 10 Years (binary): 1 if all 40 trailing quarters had positive dividends.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn dividend_positive_10_years(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();

	let mut symbol_groups: std::collections::HashMap<String, Vec<&FundamentalPoint>> =
		std::collections::HashMap::new();
	for f in &fundamentals {
		symbol_groups.entry(f.symbol.clone()).or_default().push(f);
	}

	for group in symbol_groups.values_mut() {
		group.sort_by(|a, b| a.filing_date.partial_cmp(&b.filing_date).unwrap());

		let mut recent = group.iter().rev().take(40);
		let all_positive = recent.all(|f| f.data.dividends_per_share.is_some_and(|v| v > 0.0));

		if let Some(latest) = group.last() {
			results.push(FactorPoint {
				symbol: latest.symbol.clone(),
				date: latest.filing_date,
				value: if all_positive { 1.0 } else { 0.0 },
			});
		}
	}
	results
}
