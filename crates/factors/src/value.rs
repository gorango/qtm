use crate::types::data::{Bar, FactorPoint, FundamentalPoint, FundamentalPointData};
use crate::utils::pricing::find_price_on_or_after;

pub fn price_to_book_value(d: &FundamentalPointData) -> Option<f64> {
	let mcap = d.market_cap?;
	let equity = d.shareholders_equity?;
	if equity <= 0.0 { None } else { Some(mcap / equity) }
}

pub fn price_to_sales_value(d: &FundamentalPointData) -> Option<f64> {
	let mcap = d.market_cap?;
	let revenue = d.revenue?;
	if revenue <= 0.0 { None } else { Some(mcap / revenue) }
}

pub fn free_cash_flow_yield_value(d: &FundamentalPointData) -> Option<f64> {
	let mcap = d.market_cap?;
	if mcap <= 0.0 { return None; }
	let ocf = d.operating_cash_flow?;
	let capex = d.capital_expenditure.unwrap_or(0.0);
	Some((ocf - capex) / mcap)
}

pub fn free_cash_flow_margin_value(d: &FundamentalPointData) -> Option<f64> {
	let ocf = d.operating_cash_flow?;
	let capex = d.capital_expenditure.unwrap_or(0.0);
	let revenue = d.revenue?;
	if revenue <= 0.0 { None } else { Some((ocf - capex) / revenue) }
}

pub fn owner_earnings_value(d: &FundamentalPointData) -> Option<f64> {
	Some(d.operating_cash_flow? - d.capital_expenditure.unwrap_or(0.0))
}

pub fn earnings_yield_value(d: &FundamentalPointData) -> Option<f64> {
	let eps = d.eps?;
	let mcap = d.market_cap?;
	let shares = d.shares_outstanding?;
	if shares <= 0.0 { return None; }
	let price_per_share = mcap / shares;
	if price_per_share <= 0.0 { None } else { Some(eps / price_per_share) }
}

pub fn cash_to_market_cap_value(d: &FundamentalPointData) -> Option<f64> {
	let cash = d.cash_and_equivalents?;
	let mcap = d.market_cap?;
	if mcap <= 0.0 { None } else { Some(cash / mcap) }
}

pub fn book_value_per_share_value(d: &FundamentalPointData) -> Option<f64> {
	let equity = d.shareholders_equity?;
	let shares = d.shares_outstanding?;
	if shares <= 0.0 { None } else { Some(equity / shares) }
}

pub fn net_cash_value(d: &FundamentalPointData) -> Option<f64> {
	Some(d.cash_and_equivalents? - d.total_debt?)
}

pub fn cash_to_assets_value(d: &FundamentalPointData) -> Option<f64> {
	let cash = d.cash_and_equivalents?;
	let assets = d.total_assets?;
	if assets <= 0.0 { None } else { Some(cash / assets) }
}

pub fn cash_to_liabilities_value(d: &FundamentalPointData) -> Option<f64> {
	let cash = d.cash_and_equivalents?;
	let liab = d.current_liabilities?;
	if liab <= 0.0 { None } else { Some(cash / liab) }
}

pub fn dividend_coverage_ocf_value(d: &FundamentalPointData) -> Option<f64> {
	let ocf = d.operating_cash_flow?;
	let paid = d.dividends_paid?;
	if paid <= 0.0 { None } else { Some(ocf / paid) }
}

pub fn payout_ratio_value(d: &FundamentalPointData) -> Option<f64> {
	let paid = d.dividends_paid?;
	let ni = d.net_income?;
	if ni <= 0.0 { None } else { Some(paid / ni) }
}

pub fn quick_ratio_value(d: &FundamentalPointData) -> Option<f64> {
	let ca = d.current_assets?;
	let cl = d.current_liabilities?;
	if cl <= 0.0 { None } else { Some(ca / cl) }
}

pub fn ev_to_ebitda_value(d: &FundamentalPointData) -> Option<f64> {
	let ev = d.enterprise_value?;
	let ebitda = d.ebitda?;
	if ebitda <= 0.0 { None } else { Some(ev / ebitda) }
}

pub fn ev_to_revenue_value(d: &FundamentalPointData) -> Option<f64> {
	let ev = d.enterprise_value?;
	let revenue = d.revenue?;
	if revenue <= 0.0 { None } else { Some(ev / revenue) }
}

pub fn debt_service_coverage_value(d: &FundamentalPointData) -> Option<f64> {
	let oi = d.operating_income?;
	let debt = d.total_debt?;
	if debt <= 0.0 { None } else { Some(oi / debt) }
}

/// Price-to-Earnings (P/E) ratio.
///
/// Calculates `price / EPS` for each fundamental point using the closest price on or after
/// the filing date. Skips points with missing or non-positive EPS.
///
/// # Examples
/// ```
/// use factors_core::{price_to_earnings, FundamentalPoint, FundamentalPointData, Bar};
///
/// let fp = FundamentalPoint {
///     symbol: "AAPL".into(), date: 100.0, filing_date: 100.0,
///     period: "Q1".into(),
///     data: FundamentalPointData { eps: Some(2.0), ..Default::default() },
/// };
/// let bar = Bar { time: 100.0, open: 150.0, high: 150.0, low: 150.0, close: 150.0, volume: 1000.0 };
/// let result = price_to_earnings(vec![fp], vec![bar]);
/// assert_eq!(result.len(), 1);
/// assert!((result[0].value - 75.0).abs() < 1e-6);
/// ```
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn price_to_earnings(
	fundamentals: Vec<FundamentalPoint>,
	prices: Vec<Bar>,
) -> Vec<FactorPoint> {
	let mut prices_sorted = prices.clone();
	prices_sorted.sort_by(|a, b| {
		a.time
			.partial_cmp(&b.time)
			.expect("values should be comparable")
	});

	let mut results = Vec::new();
	for f in &fundamentals {
		let eps = match f.data.eps {
			Some(v) if v > 0.0 => v,
			_ => continue,
		};
		let price = match find_price_on_or_after(&prices_sorted, f.filing_date) {
			Some(p) => p,
			None => continue,
		};
		results.push(FactorPoint {
			symbol: f.symbol.clone(),
			date: f.filing_date,
			value: price / eps,
		});
	}
	results
}

/// Price-to-Book ratio: `marketCap / shareholdersEquity`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn price_to_book(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(value) = price_to_book_value(&f.data) {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value,
			});
		}
	}
	results
}

/// Price-to-Sales ratio: `marketCap / revenue`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn price_to_sales(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(value) = price_to_sales_value(&f.data) {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value,
			});
		}
	}
	results
}

/// Price-to-Free-Cash-Flow ratio: `marketCap / (operatingCashFlow - capitalExpenditure)`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn price_to_free_cash_flow(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let mcap = match f.data.market_cap {
			Some(v) => v,
			None => continue,
		};
		let ocf = match f.data.operating_cash_flow {
			Some(v) => v,
			None => continue,
		};
		let capex = f.data.capital_expenditure.unwrap_or(0.0);
		let fcf = ocf - capex;
		if fcf <= 0.0 {
			continue;
		}
		results.push(FactorPoint {
			symbol: f.symbol.clone(),
			date: f.filing_date,
			value: mcap / fcf,
		});
	}
	results
}

/// Enterprise-Value-to-EBITDA ratio: `enterpriseValue / ebitda`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn enterprise_value_to_ebitda(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(value) = ev_to_ebitda_value(&f.data) {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value,
			});
		}
	}
	results
}

/// Earnings Yield: `eps / (marketCap / sharesOutstanding)`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn earnings_yield(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(value) = earnings_yield_value(&f.data) {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value,
			});
		}
	}
	results
}

/// Dividend Yield: `dividendsPerShare / price`. Uses closest price on/after filing date.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn dividend_yield(fundamentals: Vec<FundamentalPoint>, prices: Vec<Bar>) -> Vec<FactorPoint> {
	let mut prices_sorted = prices.clone();
	prices_sorted.sort_by(|a, b| {
		a.time
			.partial_cmp(&b.time)
			.expect("values should be comparable")
	});

	let mut results = Vec::new();
	for f in &fundamentals {
		let dps = match f.data.dividends_per_share {
			Some(v) => v,
			None => continue,
		};
		let price = match find_price_on_or_after(&prices_sorted, f.filing_date) {
			Some(p) => p,
			None => continue,
		};
		if price <= 0.0 {
			continue;
		}
		results.push(FactorPoint {
			symbol: f.symbol.clone(),
			date: f.filing_date,
			value: dps / price,
		});
	}
	results
}

/// Free Cash Flow Yield: `freeCashFlow / marketCap`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn free_cash_flow_yield(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(value) = free_cash_flow_yield_value(&f.data) {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value,
			});
		}
	}
	results
}

/// Free Cash Flow Margin: `(operatingCashFlow - capitalExpenditure) / revenue`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn free_cash_flow_margin(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(value) = free_cash_flow_margin_value(&f.data) {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value,
			});
		}
	}
	results
}

/// Margin of Safety: `dcfValue / price - 1`. Positive means undervalued per DCF.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn margin_of_safety(fundamentals: Vec<FundamentalPoint>, prices: Vec<Bar>) -> Vec<FactorPoint> {
	let mut prices_sorted = prices.clone();
	prices_sorted.sort_by(|a, b| {
		a.time
			.partial_cmp(&b.time)
			.expect("values should be comparable")
	});

	let mut results = Vec::new();
	for f in &fundamentals {
		let dcf = match f.data.dcf {
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
			value: dcf / price - 1.0,
		});
	}
	results
}

/// Owner Earnings: `operatingCashFlow - capitalExpenditure` (Buffett's metric).
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn owner_earnings(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(value) = owner_earnings_value(&f.data) {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value,
			});
		}
	}
	results
}

/// Weighted Average Cost of Capital (simplified): `E/V*0.08 + D/V*0.04*(1-0.21)`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn wacc(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let mcap = match f.data.market_cap {
			Some(v) => v,
			None => continue,
		};
		let debt = match f.data.total_debt {
			Some(v) => v,
			None => continue,
		};
		let total_cap = mcap + debt;
		if total_cap <= 0.0 {
			continue;
		}
		let e_weight = mcap / total_cap;
		let d_weight = debt / total_cap;
		let cost_equity = 0.08;
		let cost_debt = 0.04;
		let tax_rate = 0.21;
		let wacc_val = e_weight * cost_equity + d_weight * cost_debt * (1.0 - tax_rate);
		results.push(FactorPoint {
			symbol: f.symbol.clone(),
			date: f.filing_date,
			value: wacc_val,
		});
	}
	results
}

/// Cash-to-Market-Cap ratio: `cashAndEquivalents / marketCap`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn cash_to_market_cap(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(value) = cash_to_market_cap_value(&f.data) {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value,
			});
		}
	}
	results
}

/// Extracts raw market capitalization value.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn market_cap_value(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(mcap) = f.data.market_cap {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value: mcap,
			});
		}
	}
	results
}

/// Net Debt to EBITDA: `(totalDebt - cashAndEquivalents) / ebitda`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn net_debt_to_ebitda(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let debt = match f.data.total_debt {
			Some(v) => v,
			None => continue,
		};
		let cash = f.data.cash_and_equivalents.unwrap_or(0.0);
		let ebitda = match f.data.ebitda {
			Some(v) if v > 0.0 => v,
			_ => continue,
		};
		results.push(FactorPoint {
			symbol: f.symbol.clone(),
			date: f.filing_date,
			value: (debt - cash) / ebitda,
		});
	}
	results
}

/// Net Debt to EBITDAR: `(totalDebt - cash) / (operatingIncome + depreciation)`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn net_debt_to_ebitdar(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let debt = match f.data.total_debt {
			Some(v) => v,
			None => continue,
		};
		let cash = f.data.cash_and_equivalents.unwrap_or(0.0);
		let oi = match f.data.operating_income {
			Some(v) => v,
			None => continue,
		};
		let da = f.data.ebitda.map(|e| (e - oi).max(0.0)).unwrap_or(0.0);
		let ebitdar = oi + da;
		if ebitdar <= 0.0 {
			continue;
		}
		results.push(FactorPoint {
			symbol: f.symbol.clone(),
			date: f.filing_date,
			value: (debt - cash) / ebitdar,
		});
	}
	results
}

/// Debt Service Coverage Ratio: `operatingIncome / interestExpense`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn debt_service_coverage_ratio(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let oi = match f.data.operating_income {
			Some(v) => v,
			None => continue,
		};
		let interest = match f.data.interest_expense {
			Some(v) if v > 0.0 => v,
			_ => continue,
		};
		results.push(FactorPoint {
			symbol: f.symbol.clone(),
			date: f.filing_date,
			value: oi / interest,
		});
	}
	results
}

/// Book Value Per Share: `shareholdersEquity / sharesOutstanding`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn book_value_per_share(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(value) = book_value_per_share_value(&f.data) {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value,
			});
		}
	}
	results
}

/// Price-to-Book ratio (alternate): `marketCap / shareholdersEquity`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn price_to_book_ratio(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(value) = price_to_book_value(&f.data) {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value,
			});
		}
	}
	results
}

/// Price-to-Earnings ratio (alternate): `(marketCap / sharesOutstanding) / eps`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn price_to_earnings_ratio(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let mcap = match f.data.market_cap {
			Some(v) => v,
			None => continue,
		};
		let shares = match f.data.shares_outstanding {
			Some(v) if v > 0.0 => v,
			_ => continue,
		};
		let eps = match f.data.eps {
			Some(v) if v > 0.0 => v,
			_ => continue,
		};
		let price = mcap / shares;
		results.push(FactorPoint {
			symbol: f.symbol.clone(),
			date: f.filing_date,
			value: price / eps,
		});
	}
	results
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::types::data::*;

	fn make_fp(
		symbol: &str,
		date: f64,
		filing: f64,
		period: &str,
		data: FundamentalPointData,
	) -> FundamentalPoint {
		FundamentalPoint {
			symbol: symbol.to_string(),
			date,
			filing_date: filing,
			period: period.to_string(),
			data,
		}
	}

	fn make_bar(time: f64, close: f64) -> Bar {
		Bar {
			time,
			open: close,
			high: close,
			low: close,
			close,
			volume: 1000.0,
		}
	}

	fn assert_approx_eq(a: f64, b: f64, epsilon: f64) {
		assert!((a - b).abs() < epsilon, "expected {} ≈ {}", a, b);
	}

	#[test]
	fn test_price_to_earnings_happy() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				eps: Some(2.0),
				..Default::default()
			},
		);
		let prices = vec![make_bar(99.0, 150.0)];
		let result = price_to_earnings(vec![fp], prices);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 75.0, 1e-6);
	}

	#[test]
	fn test_price_to_earnings_skips_negative_eps() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				eps: Some(-1.0),
				..Default::default()
			},
		);
		let result = price_to_earnings(vec![fp], vec![]);
		assert_eq!(result.len(), 0);
	}

	#[test]
	fn test_price_to_earnings_skips_missing_eps() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData::default(),
		);
		let result = price_to_earnings(vec![fp], vec![]);
		assert_eq!(result.len(), 0);
	}

	#[test]
	fn test_price_to_earnings_empty_fundamentals() {
		let result = price_to_earnings(vec![], vec![]);
		assert_eq!(result.len(), 0);
	}

	#[test]
	fn test_price_to_sales_happy() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				market_cap: Some(1_000_000_000.0),
				revenue: Some(500_000_000.0),
				..Default::default()
			},
		);
		let result = price_to_sales(vec![fp]);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 2.0, 1e-6);
	}

	#[test]
	fn test_price_to_sales_skips_missing_revenue() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				market_cap: Some(1e9),
				..Default::default()
			},
		);
		let result = price_to_sales(vec![fp]);
		assert_eq!(result.len(), 0);
	}

	#[test]
	fn test_free_cash_flow_yield_happy() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				market_cap: Some(2000.0),
				operating_cash_flow: Some(500.0),
				capital_expenditure: Some(100.0),
				..Default::default()
			},
		);
		let result = free_cash_flow_yield(vec![fp]);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 0.2, 1e-6);
	}

	#[test]
	fn test_free_cash_flow_yield_skips_when_no_data() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData::default(),
		);
		let result = free_cash_flow_yield(vec![fp]);
		assert_eq!(result.len(), 0);
	}

	#[test]
	fn test_earnings_yield_happy() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				eps: Some(5.0),
				market_cap: Some(10000.0),
				shares_outstanding: Some(1000.0),
				..Default::default()
			},
		);
		let result = earnings_yield(vec![fp]);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 0.5, 1e-6);
	}

	#[test]
	fn test_dividend_yield_happy() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				dividends_per_share: Some(2.0),
				..Default::default()
			},
		);
		let prices = vec![make_bar(99.0, 50.0)];
		let result = dividend_yield(vec![fp], prices);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 0.04, 1e-6);
	}

	#[test]
	fn test_dividend_yield_skips_no_dps() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData::default(),
		);
		let result = dividend_yield(vec![fp], vec![]);
		assert_eq!(result.len(), 0);
	}

	#[test]
	fn test_wacc_happy() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				market_cap: Some(800.0),
				total_debt: Some(200.0),
				..Default::default()
			},
		);
		let result = wacc(vec![fp]);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 0.07032, 1e-6);
	}

	#[test]
	fn test_book_value_per_share_happy() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				shareholders_equity: Some(1_000_000.0),
				shares_outstanding: Some(500_000.0),
				..Default::default()
			},
		);
		let result = book_value_per_share(vec![fp]);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 2.0, 1e-6);
	}

	#[test]
	fn test_market_cap_value_happy() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				market_cap: Some(2.5e9),
				..Default::default()
			},
		);
		let result = market_cap_value(vec![fp]);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 2.5e9, 1e-6);
	}

	#[test]
	fn test_market_cap_value_missing() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData::default(),
		);
		let result = market_cap_value(vec![fp]);
		assert_eq!(result.len(), 0);
	}

	#[test]
	fn test_empty_fundamentals_all_functions() {
		assert_eq!(price_to_earnings(vec![], vec![]).len(), 0);
		assert_eq!(price_to_sales(vec![]).len(), 0);
		assert_eq!(price_to_book(vec![]).len(), 0);
		assert_eq!(price_to_free_cash_flow(vec![]).len(), 0);
		assert_eq!(free_cash_flow_yield(vec![]).len(), 0);
		assert_eq!(free_cash_flow_margin(vec![]).len(), 0);
		assert_eq!(earnings_yield(vec![]).len(), 0);
		assert_eq!(dividend_yield(vec![], vec![]).len(), 0);
		assert_eq!(wacc(vec![]).len(), 0);
		assert_eq!(book_value_per_share(vec![]).len(), 0);
		assert_eq!(market_cap_value(vec![]).len(), 0);
		assert_eq!(owner_earnings(vec![]).len(), 0);
		assert_eq!(cash_to_market_cap(vec![]).len(), 0);
		assert_eq!(net_debt_to_ebitda(vec![]).len(), 0);
		assert_eq!(net_debt_to_ebitdar(vec![]).len(), 0);
		assert_eq!(debt_service_coverage_ratio(vec![]).len(), 0);
		assert_eq!(price_to_book_ratio(vec![]).len(), 0);
		assert_eq!(price_to_earnings_ratio(vec![]).len(), 0);
		assert_eq!(margin_of_safety(vec![], vec![]).len(), 0);
		assert_eq!(enterprise_value_to_ebitda(vec![]).len(), 0);
	}

	#[test]
	fn test_edge_zero_market_cap_skips_earnings_yield() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				market_cap: Some(0.0),
				eps: Some(5.0),
				shares_outstanding: Some(1000.0),
				..Default::default()
			},
		);
		let result = earnings_yield(vec![fp]);
		assert_eq!(result.len(), 0);
	}

	#[test]
	fn test_edge_zero_equity_skips_price_to_book() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				shareholders_equity: Some(0.0),
				market_cap: Some(1000.0),
				..Default::default()
			},
		);
		let result = price_to_book(vec![fp]);
		assert_eq!(result.len(), 0);
	}

	#[test]
	fn test_edge_negative_fcf_still_computes() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				market_cap: Some(2000.0),
				operating_cash_flow: Some(100.0),
				capital_expenditure: Some(500.0),
				..Default::default()
			},
		);
		let result = free_cash_flow_yield(vec![fp]);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, -0.2, 1e-6);
	}
}
