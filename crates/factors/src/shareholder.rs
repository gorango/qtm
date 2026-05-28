use crate::types::data::{FactorPoint, FundamentalPoint};

/// Shareholder Yield: `(dividendsPerShare * sharesOutstanding) / marketCap`.
/// Captures dividend component of total shareholder return.
pub fn shareholder_yield(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let dps = match f.data.dividends_per_share {
			Some(v) => v,
			None => continue,
		};
		let mcap = match f.data.market_cap {
			Some(v) => v,
			None => continue,
		};
		let shares = match f.data.shares_outstanding {
			Some(v) if v > 0.0 => v,
			_ => continue,
		};
		if mcap <= 0.0 {
			continue;
		}
		let div_yield = (dps * shares) / mcap;
		results.push(FactorPoint {
			date: f.filing_date,
			value: div_yield,
		});
	}
	results
}

/// Dividend Payout Ratio: `dividendsPerShare / eps`.
pub fn dividend_payout_ratio(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let dps = match f.data.dividends_per_share {
			Some(v) => v,
			None => continue,
		};
		let eps = match f.data.eps {
			Some(v) if v > 0.0 => v,
			_ => continue,
		};
		results.push(FactorPoint {
			date: f.filing_date,
			value: dps / eps,
		});
	}
	results
}

/// Dividend Coverage Ratio: `netIncome / (dividendsPerShare * sharesOutstanding)`.
pub fn dividend_coverage_ratio(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let ni = match f.data.net_income {
			Some(v) => v,
			None => continue,
		};
		let dps = match f.data.dividends_per_share {
			Some(v) => v,
			None => continue,
		};
		let shares = match f.data.shares_outstanding {
			Some(v) if v > 0.0 => v,
			_ => continue,
		};
		let total_divs = dps * shares;
		if total_divs <= 0.0 {
			continue;
		}
		results.push(FactorPoint {
			date: f.filing_date,
			value: ni / total_divs,
		});
	}
	results
}

/// Dividend Positive for 10 Years (binary): 1 if all 40 trailing quarters had positive dividends.
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
				date: latest.filing_date,
				value: if all_positive { 1.0 } else { 0.0 },
			});
		}
	}
	results
}
