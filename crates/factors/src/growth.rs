use crate::types::data::{FactorPoint, FundamentalPoint};
use crate::utils::growth::create_growth_factor;

/// Year-over-Year Revenue Growth: `(currentRevenue - previousRevenue) / |previousRevenue|`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn revenue_growth_yo_y(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	create_growth_factor(&fundamentals, "revenue")
}

/// Revenue CAGR: `(endValue / startValue)^(1/period) - 1` over `period` filings.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn revenue_growth_cagr(
	fundamentals: Vec<FundamentalPoint>,
	period: Option<u32>,
) -> Vec<FactorPoint> {
	let p = period.unwrap_or(5) as usize;
	let mut results = Vec::new();

	let mut symbol_groups: std::collections::HashMap<String, Vec<&FundamentalPoint>> =
		std::collections::HashMap::new();
	for f in &fundamentals {
		symbol_groups.entry(f.symbol.clone()).or_default().push(f);
	}

	for group in symbol_groups.values_mut() {
		group.sort_by(|a, b| {
			a.filing_date
				.partial_cmp(&b.filing_date)
				.unwrap_or(std::cmp::Ordering::Equal)
		});
		if group.len() <= p {
			continue;
		}
		for i in p..group.len() {
			let cur = group[i];
			let start = group[i - p];
			let end_val = match cur.data.revenue {
				Some(v) => v,
				None => continue,
			};
			let start_val = match start.data.revenue {
				Some(v) if v > 0.0 => v,
				_ => continue,
			};
			let cagr = (end_val / start_val).powf(1.0 / p as f64) - 1.0;
			results.push(FactorPoint {
				symbol: cur.symbol.clone(),
				date: cur.filing_date,
				value: cagr,
			});
		}
	}
	results
}

/// Revenue Seasonality Index: `(maxQuarterRevenue - minQuarterRevenue) / annualAvgRevenue`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn revenue_seasonality(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();

	let mut symbol_groups: std::collections::HashMap<String, Vec<&FundamentalPoint>> =
		std::collections::HashMap::new();
	for f in &fundamentals {
		symbol_groups.entry(f.symbol.clone()).or_default().push(f);
	}

	for group in symbol_groups.values_mut() {
		group.sort_by(|a, b| {
			a.date
				.partial_cmp(&b.date)
				.unwrap_or(std::cmp::Ordering::Equal)
		});
		if group.len() < 8 {
			continue;
		}

		let mut q_revenues: std::collections::HashMap<&str, Vec<f64>> =
			std::collections::HashMap::new();
		q_revenues.insert("Q1", Vec::new());
		q_revenues.insert("Q2", Vec::new());
		q_revenues.insert("Q3", Vec::new());
		q_revenues.insert("Q4", Vec::new());

		for f in group.iter() {
			if let Some(rev) = f.data.revenue {
				if let Some(v) = q_revenues.get_mut(f.period.as_str()) {
					v.push(rev);
				}
			}
		}

		let mut avg_revenues = Vec::new();
		let mut total = 0.0;
		let mut count = 0;
		for vals in q_revenues.values() {
			if !vals.is_empty() {
				let avg = vals.iter().sum::<f64>() / vals.len() as f64;
				avg_revenues.push(avg);
				total += avg;
				count += 1;
			}
		}
		if count < 4 {
			continue;
		}
		let annual_avg = total / count as f64;
		let highest = avg_revenues.iter().cloned().fold(f64::NAN, f64::max);
		let lowest = avg_revenues.iter().cloned().fold(f64::NAN, f64::min);
		let seasonality = (highest - lowest) / annual_avg;

		let latest = group[group.len() - 1];
		results.push(FactorPoint {
			symbol: latest.symbol.clone(),
			date: latest.date,
			value: seasonality,
		});
	}
	results
}

/// 5-Year Revenue Per Share Growth: `(rpsEnd / rpsStart)^(1/5) - 1`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn five_y_revenue_growth_per_share(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	let mut symbol_groups: std::collections::HashMap<String, Vec<&FundamentalPoint>> =
		std::collections::HashMap::new();
	for f in &fundamentals {
		symbol_groups.entry(f.symbol.clone()).or_default().push(f);
	}

	for group in symbol_groups.values_mut() {
		group.sort_by(|a, b| {
			a.filing_date
				.partial_cmp(&b.filing_date)
				.unwrap_or(std::cmp::Ordering::Equal)
		});
		if group.len() < 6 {
			continue;
		}
		for i in 5..group.len() {
			let cur = group[i];
			let start = group[i - 5];
			if let (Some(rev_cur), Some(rev_start), Some(shares_cur), Some(shares_start)) = (
				cur.data.revenue,
				start.data.revenue,
				cur.data.shares_outstanding,
				start.data.shares_outstanding,
			) {
				if shares_cur > 0.0 && shares_start > 0.0 && rev_start > 0.0 {
					let rps_cur = rev_cur / shares_cur;
					let rps_start = rev_start / shares_start;
					let growth = (rps_cur / rps_start).powf(1.0 / 5.0) - 1.0;
					results.push(FactorPoint {
						symbol: cur.symbol.clone(),
						date: cur.filing_date,
						value: growth,
					});
				}
			}
		}
	}
	results
}

/// Year-over-Year EPS Growth: `(currentEPS - previousEPS) / |previousEPS|`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn epsgrowth(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	create_growth_factor(&fundamentals, "eps")
}

/// Quarter-over-Quarter EPS Growth: sequential `(currentEPS - previousEPS) / |previousEPS|`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn eps_growth_qo_q(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();

	let mut symbol_groups: std::collections::HashMap<String, Vec<&FundamentalPoint>> =
		std::collections::HashMap::new();
	for f in &fundamentals {
		symbol_groups.entry(f.symbol.clone()).or_default().push(f);
	}

	for group in symbol_groups.values_mut() {
		group.sort_by(|a, b| {
			a.filing_date
				.partial_cmp(&b.filing_date)
				.unwrap_or(std::cmp::Ordering::Equal)
		});
		for i in 1..group.len() {
			let cur = group[i];
			let prev = group[i - 1];
			if let (Some(eps_c), Some(eps_p)) = (cur.data.eps, prev.data.eps) {
				if eps_p != 0.0 {
					results.push(FactorPoint {
						symbol: cur.symbol.clone(),
						date: cur.filing_date,
						value: (eps_c - eps_p) / eps_p.abs(),
					});
				}
			}
		}
	}
	results
}

/// 10-Year EPS Growth: point-to-point `(epsLast / epsFirst)^(1/years) - 1`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn eps_growth_10_year(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();

	let mut symbol_groups: std::collections::HashMap<String, Vec<&FundamentalPoint>> =
		std::collections::HashMap::new();
	for f in &fundamentals {
		symbol_groups.entry(f.symbol.clone()).or_default().push(f);
	}

	for group in symbol_groups.values_mut() {
		group.sort_by(|a, b| {
			a.filing_date
				.partial_cmp(&b.filing_date)
				.unwrap_or(std::cmp::Ordering::Equal)
		});
		if group.len() < 2 {
			continue;
		}
		let first = group[0];
		let last = group[group.len() - 1];
		if let (Some(eps_first), Some(eps_last)) = (first.data.eps, last.data.eps) {
			if eps_first > 0.0 {
				let years =
					(last.filing_date - first.filing_date) / (365.25 * 24.0 * 60.0 * 60.0 * 1000.0);
				if years > 0.0 {
					let growth = (eps_last / eps_first).powf(1.0 / years) - 1.0;
					results.push(FactorPoint {
						symbol: last.symbol.clone(),
						date: last.filing_date,
						value: growth,
					});
				}
			}
		}
	}
	results
}

/// EPS CAGR: `(endEPS / startEPS)^(1/period) - 1` over `period` filings.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn eps_growth_cagr(
	fundamentals: Vec<FundamentalPoint>,
	period: Option<u32>,
) -> Vec<FactorPoint> {
	let p = period.unwrap_or(5) as usize;
	let mut results = Vec::new();

	let mut symbol_groups: std::collections::HashMap<String, Vec<&FundamentalPoint>> =
		std::collections::HashMap::new();
	for f in &fundamentals {
		symbol_groups.entry(f.symbol.clone()).or_default().push(f);
	}

	for group in symbol_groups.values_mut() {
		group.sort_by(|a, b| {
			a.filing_date
				.partial_cmp(&b.filing_date)
				.unwrap_or(std::cmp::Ordering::Equal)
		});
		if group.len() <= p {
			continue;
		}
		for i in p..group.len() {
			let cur = group[i];
			let start = group[i - p];
			if let (Some(eps_c), Some(eps_s)) = (cur.data.eps, start.data.eps) {
				if eps_s > 0.0 {
					let cagr = (eps_c / eps_s).powf(1.0 / p as f64) - 1.0;
					results.push(FactorPoint {
						symbol: cur.symbol.clone(),
						date: cur.filing_date,
						value: cagr,
					});
				}
			}
		}
	}
	results
}

/// Rolling Average EPS: mean EPS over last `periods` filings.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn eps_avg(fundamentals: Vec<FundamentalPoint>, periods: Option<u32>) -> Vec<FactorPoint> {
	let p = periods.unwrap_or(4) as usize;
	let mut results = Vec::new();

	let mut symbol_groups: std::collections::HashMap<String, Vec<&FundamentalPoint>> =
		std::collections::HashMap::new();
	for f in &fundamentals {
		symbol_groups.entry(f.symbol.clone()).or_default().push(f);
	}

	for group in symbol_groups.values_mut() {
		group.sort_by(|a, b| {
			a.filing_date
				.partial_cmp(&b.filing_date)
				.unwrap_or(std::cmp::Ordering::Equal)
		});
		if group.len() < p {
			continue;
		}
		for i in (p - 1)..group.len() {
			let mut sum = 0.0;
			let mut count = 0;
			for item in group.iter().take(i + 1).skip(i + 1 - p) {
				if let Some(eps) = item.data.eps {
					sum += eps;
					count += 1;
				}
			}
			if count > 0 {
				results.push(FactorPoint {
					symbol: group[i].symbol.clone(),
					date: group[i].filing_date,
					value: sum / count as f64,
				});
			}
		}
	}
	results
}

/// Count of quarters with positive EPS over last `periods` filings.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn eps_positive_count(
	fundamentals: Vec<FundamentalPoint>,
	periods: Option<u32>,
) -> Vec<FactorPoint> {
	let p = periods.unwrap_or(10) as usize;
	let mut results = Vec::new();

	let mut symbol_groups: std::collections::HashMap<String, Vec<&FundamentalPoint>> =
		std::collections::HashMap::new();
	for f in &fundamentals {
		symbol_groups.entry(f.symbol.clone()).or_default().push(f);
	}

	for group in symbol_groups.values_mut() {
		group.sort_by(|a, b| {
			a.filing_date
				.partial_cmp(&b.filing_date)
				.unwrap_or(std::cmp::Ordering::Equal)
		});
		if group.len() < p {
			continue;
		}
		for i in (p - 1)..group.len() {
			let count = ((i + 1 - p)..=i)
				.filter(|&j| group[j].data.eps.is_some_and(|v| v > 0.0))
				.count() as f64;
			results.push(FactorPoint {
				symbol: group[i].symbol.clone(),
				date: group[i].filing_date,
				value: count,
			});
		}
	}
	results
}

/// Sequential EPS Growth: `(currentEPS - previousEPS) / |previousEPS|`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn growth_eps(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();

	let mut symbol_groups: std::collections::HashMap<String, Vec<&FundamentalPoint>> =
		std::collections::HashMap::new();
	for f in &fundamentals {
		symbol_groups.entry(f.symbol.clone()).or_default().push(f);
	}

	for group in symbol_groups.values_mut() {
		group.sort_by(|a, b| {
			a.filing_date
				.partial_cmp(&b.filing_date)
				.unwrap_or(std::cmp::Ordering::Equal)
		});
		for i in 1..group.len() {
			let cur = group[i];
			let prev = group[i - 1];
			if let (Some(eps_c), Some(eps_p)) = (cur.data.eps, prev.data.eps) {
				if eps_p != 0.0 {
					results.push(FactorPoint {
						symbol: cur.symbol.clone(),
						date: cur.filing_date,
						value: (eps_c - eps_p) / eps_p.abs(),
					});
				}
			}
		}
	}
	results
}

/// Sequential Free Cash Flow Growth: `(fcfCurrent - fcfPrevious) / |fcfPrevious|`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn free_cash_flow_growth(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();

	let mut symbol_groups: std::collections::HashMap<String, Vec<&FundamentalPoint>> =
		std::collections::HashMap::new();
	for f in &fundamentals {
		symbol_groups.entry(f.symbol.clone()).or_default().push(f);
	}

	for group in symbol_groups.values_mut() {
		group.sort_by(|a, b| {
			a.filing_date
				.partial_cmp(&b.filing_date)
				.unwrap_or(std::cmp::Ordering::Equal)
		});
		for i in 1..group.len() {
			let cur = group[i];
			let prev = group[i - 1];
			let fcf_c = match (cur.data.operating_cash_flow, cur.data.capital_expenditure) {
				(Some(ocf), _) => {
					let capex = cur.data.capital_expenditure.unwrap_or(0.0);
					ocf - capex
				}
				_ => continue,
			};
			let fcf_p = match (prev.data.operating_cash_flow, prev.data.capital_expenditure) {
				(Some(ocf), _) => {
					let capex = prev.data.capital_expenditure.unwrap_or(0.0);
					ocf - capex
				}
				_ => continue,
			};
			if fcf_p != 0.0 {
				results.push(FactorPoint {
					symbol: cur.symbol.clone(),
					date: cur.filing_date,
					value: (fcf_c - fcf_p) / fcf_p.abs(),
				});
			}
		}
	}
	results
}

/// Year-over-Year Cost Growth: `(currentCost - previousCost) / |previousCost|`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn cost_growth_yo_y(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	create_growth_factor(&fundamentals, "cost_and_expenses")
}

/// Share Count CAGR: `(sharesLast / sharesFirst)^(1/years) - 1`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn share_count_growth(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();

	let mut symbol_groups: std::collections::HashMap<String, Vec<&FundamentalPoint>> =
		std::collections::HashMap::new();
	for f in &fundamentals {
		symbol_groups.entry(f.symbol.clone()).or_default().push(f);
	}

	for group in symbol_groups.values_mut() {
		group.sort_by(|a, b| {
			a.filing_date
				.partial_cmp(&b.filing_date)
				.unwrap_or(std::cmp::Ordering::Equal)
		});
		if group.len() < 2 {
			continue;
		}
		let first = group[0];
		let last = group[group.len() - 1];
		if let (Some(shares_first), Some(shares_last)) =
			(first.data.shares_outstanding, last.data.shares_outstanding)
		{
			if shares_first > 0.0 {
				let years =
					(last.filing_date - first.filing_date) / (365.25 * 24.0 * 60.0 * 60.0 * 1000.0);
				if years > 0.0 {
					let growth = (shares_last / shares_first).powf(1.0 / years) - 1.0;
					results.push(FactorPoint {
						symbol: last.symbol.clone(),
						date: last.filing_date,
						value: growth,
					});
				}
			}
		}
	}
	results
}
