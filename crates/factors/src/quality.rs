use crate::types::data::{FactorPoint, FundamentalPoint};

pub use self::quality_helpers::*;
mod quality_helpers {
	use crate::types::data::FundamentalPointData;

	pub fn roe_value(d: &FundamentalPointData) -> Option<f64> {
		let ni = d.net_income?;
		let equity = d.shareholders_equity?;
		if equity == 0.0 { None } else { Some(ni / equity) }
	}

	pub fn roa_value(d: &FundamentalPointData) -> Option<f64> {
		let ni = d.net_income?;
		let ta = d.total_assets?;
		if ta == 0.0 { None } else { Some(ni / ta) }
	}

	pub fn gross_margin_value(d: &FundamentalPointData) -> Option<f64> {
		let revenue = d.revenue?;
		if revenue == 0.0 { None } else { Some((revenue - d.cost_of_revenue.unwrap_or(0.0)) / revenue) }
	}

	pub fn net_margin_value(d: &FundamentalPointData) -> Option<f64> {
		let ni = d.net_income?;
		let revenue = d.revenue?;
		if revenue == 0.0 { None } else { Some(ni / revenue) }
	}

	pub fn operating_profit_margin_value(d: &FundamentalPointData) -> Option<f64> {
		let oi = d.operating_income?;
		let revenue = d.revenue?;
		if revenue == 0.0 { None } else { Some(oi / revenue) }
	}

	pub fn ebitda_margin_value(d: &FundamentalPointData) -> Option<f64> {
		let ebitda = d.ebitda.or(d.operating_income)?;
		let revenue = d.revenue?;
		if revenue == 0.0 { None } else { Some(ebitda / revenue) }
	}

	pub fn working_capital_value(d: &FundamentalPointData) -> Option<f64> {
		Some(d.current_assets? - d.current_liabilities?)
	}

	pub fn working_capital_turnover_value(d: &FundamentalPointData) -> Option<f64> {
		let wc = working_capital_value(d)?;
		if wc <= 0.0 { None } else { Some(d.revenue? / wc) }
	}

	pub fn debt_to_equity_value(d: &FundamentalPointData) -> Option<f64> {
		let liab = d.total_liabilities?;
		let equity = d.shareholders_equity?;
		if equity == 0.0 { None } else { Some(liab / equity) }
	}

	pub fn rnd_to_revenue_value(d: &FundamentalPointData) -> Option<f64> {
		let rd = d.research_and_development_expenses?;
		let revenue = d.revenue?;
		if revenue == 0.0 { None } else { Some(rd / revenue) }
	}

	pub fn net_debt_to_ebitda_value(d: &FundamentalPointData) -> Option<f64> {
		let e = d.ebitda?;
		if e == 0.0 { None } else { Some((d.total_debt? - d.cash_and_equivalents.unwrap_or(0.0)) / e) }
	}

	pub fn fcf_value(d: &FundamentalPointData) -> Option<f64> {
		Some(d.operating_cash_flow? - d.capital_expenditure?)
	}

	pub fn fcf_margin_value(d: &FundamentalPointData) -> Option<f64> {
		let r = d.revenue?;
		if r == 0.0 { None } else { Some(fcf_value(d)? / r) }
	}

	pub fn fcf_per_share_value(d: &FundamentalPointData) -> Option<f64> {
		let s = d.shares_outstanding?;
		if s == 0.0 { None } else { Some(fcf_value(d)? / s) }
	}

	pub fn interest_coverage_value(d: &FundamentalPointData) -> Option<f64> {
		let i = d.interest_expense?;
		if i == 0.0 { None } else { Some(d.operating_income? / i) }
	}

	pub fn pe_ratio_value(d: &FundamentalPointData) -> Option<f64> {
		let n = d.net_income?;
		if n == 0.0 { None } else { Some(d.market_cap? / n) }
	}

	pub fn current_ratio_value(d: &FundamentalPointData) -> Option<f64> {
		let l = d.current_liabilities?;
		if l == 0.0 { None } else { Some(d.current_assets? / l) }
	}

	pub fn roic_value(d: &FundamentalPointData) -> Option<f64> {
		let cap = d.total_assets? - d.cash_and_equivalents.unwrap_or(0.0) - d.current_liabilities?;
		if cap == 0.0 { None } else { Some(d.operating_income? / cap) }
	}

	pub fn ebitdar_value(d: &FundamentalPointData) -> Option<f64> {
		let oi = d.operating_income?;
		let da = if let Some(e) = d.ebitda { (e - oi).max(0.0) } else { 0.0 };
		Some(oi + da)
	}
}

/// Return on Equity: `netIncome / shareholdersEquity`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn return_on_equity(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(value) = roe_value(&f.data) {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value,
			});
		}
	}
	results
}

/// Return on Assets: `netIncome / totalAssets`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn return_on_assets(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(value) = roa_value(&f.data) {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value,
			});
		}
	}
	results
}

/// Return on Invested Capital: `(netIncome - totalDividends) / (totalLiabilities + shareholdersEquity)`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn return_on_invested_capital(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let ni = match f.data.net_income {
			Some(v) => v,
			None => continue,
		};
		let dps = f.data.dividends_per_share.unwrap_or(0.0);
		let shares = match f.data.shares_outstanding {
			Some(v) if v > 0.0 => v,
			_ => continue,
		};
		let total_liab = match f.data.total_liabilities {
			Some(v) => v,
			None => continue,
		};
		let equity = match f.data.shareholders_equity {
			Some(v) => v,
			None => continue,
		};
		let total_divs = dps * shares;
		let invested = total_liab + equity;
		if invested <= 0.0 {
			continue;
		}
		results.push(FactorPoint {
			symbol: f.symbol.clone(),
			date: f.filing_date,
			value: (ni - total_divs) / invested,
		});
	}
	results
}

/// Gross Margin: `(revenue - costOfRevenue) / revenue`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn gross_margin(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(value) = gross_margin_value(&f.data) {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value,
			});
		}
	}
	results
}

/// Net Profit Margin: `netIncome / revenue`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn net_margin(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(value) = net_margin_value(&f.data) {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value,
			});
		}
	}
	results
}

/// Operating Profit Margin: `operatingIncome / revenue`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn operating_profit_margin(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(value) = operating_profit_margin_value(&f.data) {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value,
			});
		}
	}
	results
}

/// EBITDA Margin: `ebitda / revenue`. Falls back to `operatingIncome / revenue`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn ebitda_margin(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(value) = ebitda_margin_value(&f.data) {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value,
			});
		}
	}
	results
}

/// Asset Turnover: `revenue / totalAssets`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn asset_turnover(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let revenue = match f.data.revenue {
			Some(v) => v,
			None => continue,
		};
		let ta = match f.data.total_assets {
			Some(v) if v > 0.0 => v,
			_ => continue,
		};
		results.push(FactorPoint {
			symbol: f.symbol.clone(),
			date: f.filing_date,
			value: revenue / ta,
		});
	}
	results
}

/// Working Capital: `currentAssets - currentLiabilities`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn working_capital(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(value) = working_capital_value(&f.data) {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value,
			});
		}
	}
	results
}

/// Working Capital Turnover: `revenue / (currentAssets - currentLiabilities)`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn working_capital_turnover(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(value) = working_capital_turnover_value(&f.data) {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value,
			});
		}
	}
	results
}

/// Quality of Earnings Index (0-1): composite of accruals, cash flow consistency,
/// revenue quality, and balance sheet strength.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn quality_of_earnings_index(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let ni = match f.data.net_income {
			Some(v) => v,
			None => continue,
		};
		let ocf = match f.data.operating_cash_flow {
			Some(v) => v,
			None => continue,
		};
		let ta = match f.data.total_assets {
			Some(v) => v,
			None => continue,
		};

		let mut quality = 0.0;
		let mut max_score = 0.0;

		let accruals = (ni - ocf) / ta;
		if accruals.is_finite() {
			let accrual_score = (1.0 - accruals.abs()).max(0.0);
			quality += accrual_score * 0.4;
			max_score += 0.4;
		}

		if ni != 0.0 {
			let cf_ratio = ocf / ni;
			let consistency = cf_ratio.clamp(0.0, 1.0);
			quality += consistency * 0.3;
			max_score += 0.3;
		} else if ocf == 0.0 {
			quality += 0.5 * 0.3;
			max_score += 0.3;
		}

		if let Some(revenue) = f.data.revenue {
			if revenue > 0.0 {
				if let Some(gp) = f.data.gross_profit {
					let gm = gp / revenue;
					let margin_score = gm.clamp(0.0, 1.0);
					quality += margin_score * 0.2;
					max_score += 0.2;
				}
			}
		}

		if let (Some(ca), Some(cl)) = (f.data.current_assets, f.data.current_liabilities) {
			if cl > 0.0 {
				let cr = ca / cl;
				let bs_score = ((cr - 1.0) / 2.0).clamp(0.0, 1.0);
				quality += bs_score * 0.1;
				max_score += 0.1;
			}
		}

		let index = if max_score > 0.0 {
			quality / max_score
		} else {
			0.0
		};
		results.push(FactorPoint {
			symbol: f.symbol.clone(),
			date: f.filing_date,
			value: index,
		});
	}
	results
}

/// Retained Earnings Per Share: `retainedEarnings / sharesOutstanding`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn retained_earnings(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let re = match f.data.retained_earnings {
			Some(v) => v,
			None => continue,
		};
		let shares = match f.data.shares_outstanding {
			Some(v) if v > 0.0 => v,
			_ => continue,
		};
		results.push(FactorPoint {
			symbol: f.symbol.clone(),
			date: f.filing_date,
			value: re / shares,
		});
	}
	results
}

/// R&D-to-Revenue ratio: `researchAndDevelopmentExpenses / revenue`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn r_and_d_to_revenue(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(value) = rnd_to_revenue_value(&f.data) {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value,
			});
		}
	}
	results
}

/// Placeholder — requires price data not available in current IFundamentalPoint.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn historical_volatility_vs_beta(_fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	Vec::new()
}

/// Debt-to-Equity ratio: `totalLiabilities / shareholdersEquity`.
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn debt_to_equity(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(value) = debt_to_equity_value(&f.data) {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value,
			});
		}
	}
	results
}

/// EBITDAR: `operatingIncome + depreciationAndAmortization` (adds back rent proxy).
#[cfg_attr(feature = "napi", ::napi_derive::napi)]
pub fn ebitdar(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		if let Some(value) = ebitdar_value(&f.data) {
			results.push(FactorPoint {
				symbol: f.symbol.clone(),
				date: f.filing_date,
				value,
			});
		}
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

	fn assert_approx_eq(a: f64, b: f64, epsilon: f64) {
		assert!((a - b).abs() < epsilon, "expected {} ≈ {}", a, b);
	}

	#[test]
	fn test_return_on_equity_happy() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				net_income: Some(50.0),
				shareholders_equity: Some(200.0),
				..Default::default()
			},
		);
		let result = return_on_equity(vec![fp]);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 0.25, 1e-6);
	}

	#[test]
	fn test_return_on_equity_skips_no_equity() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				net_income: Some(50.0),
				..Default::default()
			},
		);
		let result = return_on_equity(vec![fp]);
		assert_eq!(result.len(), 0);
	}

	#[test]
	fn test_debt_to_equity_happy() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				total_liabilities: Some(300.0),
				shareholders_equity: Some(200.0),
				..Default::default()
			},
		);
		let result = debt_to_equity(vec![fp]);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 1.5, 1e-6);
	}

	#[test]
	fn test_gross_margin_happy() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				revenue: Some(1000.0),
				cost_of_revenue: Some(600.0),
				..Default::default()
			},
		);
		let result = gross_margin(vec![fp]);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 0.4, 1e-6);
	}

	#[test]
	fn test_net_margin_happy() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				net_income: Some(200.0),
				revenue: Some(1000.0),
				..Default::default()
			},
		);
		let result = net_margin(vec![fp]);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 0.2, 1e-6);
	}

	#[test]
	fn test_operating_profit_margin_happy() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				operating_income: Some(300.0),
				revenue: Some(1000.0),
				..Default::default()
			},
		);
		let result = operating_profit_margin(vec![fp]);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 0.3, 1e-6);
	}

	#[test]
	fn test_working_capital_happy() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				current_assets: Some(500.0),
				current_liabilities: Some(300.0),
				..Default::default()
			},
		);
		let result = working_capital(vec![fp]);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 200.0, 1e-6);
	}

	#[test]
	fn test_working_capital_turnover_happy() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				revenue: Some(1000.0),
				current_assets: Some(500.0),
				current_liabilities: Some(200.0),
				..Default::default()
			},
		);
		let result = working_capital_turnover(vec![fp]);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 1000.0 / 300.0, 1e-6);
	}

	#[test]
	fn test_quality_of_earnings_index_happy() {
		let fp = make_fp(
			"AAPL",
			100.0,
			99.0,
			"annual",
			FundamentalPointData {
				net_income: Some(100.0),
				operating_cash_flow: Some(80.0),
				total_assets: Some(500.0),
				revenue: Some(1000.0),
				gross_profit: Some(400.0),
				current_assets: Some(300.0),
				current_liabilities: Some(200.0),
				..Default::default()
			},
		);
		let result = quality_of_earnings_index(vec![fp]);
		assert_eq!(result.len(), 1);
		assert_approx_eq(result[0].value, 0.729, 1e-6);
	}

	#[test]
	fn test_empty_fundamentals() {
		assert_eq!(return_on_equity(vec![]).len(), 0);
		assert_eq!(return_on_assets(vec![]).len(), 0);
		assert_eq!(return_on_invested_capital(vec![]).len(), 0);
		assert_eq!(gross_margin(vec![]).len(), 0);
		assert_eq!(net_margin(vec![]).len(), 0);
		assert_eq!(operating_profit_margin(vec![]).len(), 0);
		assert_eq!(ebitda_margin(vec![]).len(), 0);
		assert_eq!(asset_turnover(vec![]).len(), 0);
		assert_eq!(working_capital(vec![]).len(), 0);
		assert_eq!(working_capital_turnover(vec![]).len(), 0);
		assert_eq!(quality_of_earnings_index(vec![]).len(), 0);
		assert_eq!(debt_to_equity(vec![]).len(), 0);
		assert_eq!(ebitdar(vec![]).len(), 0);
		assert_eq!(retained_earnings(vec![]).len(), 0);
		assert_eq!(r_and_d_to_revenue(vec![]).len(), 0);
	}
}
