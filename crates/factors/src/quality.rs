use crate::types::data::{FactorPoint, FundamentalPoint};

/// Return on Equity: `netIncome / shareholdersEquity`.
pub fn return_on_equity(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let ni = match f.data.net_income {
			Some(v) => v,
			None => continue,
		};
		let equity = match f.data.shareholders_equity {
			Some(v) if v > 0.0 => v,
			_ => continue,
		};
		results.push(FactorPoint {
			date: f.filing_date,
			value: ni / equity,
		});
	}
	results
}

/// Return on Assets: `netIncome / totalAssets`.
pub fn return_on_assets(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let ni = match f.data.net_income {
			Some(v) => v,
			None => continue,
		};
		let ta = match f.data.total_assets {
			Some(v) if v > 0.0 => v,
			_ => continue,
		};
		results.push(FactorPoint {
			date: f.filing_date,
			value: ni / ta,
		});
	}
	results
}

/// Return on Invested Capital: `(netIncome - totalDividends) / (totalLiabilities + shareholdersEquity)`.
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
			date: f.filing_date,
			value: (ni - total_divs) / invested,
		});
	}
	results
}

/// Gross Margin: `(revenue - costOfRevenue) / revenue`.
pub fn gross_margin(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let revenue = match f.data.revenue {
			Some(v) if v > 0.0 => v,
			_ => continue,
		};
		let cor = f.data.cost_of_revenue.unwrap_or(0.0);
		results.push(FactorPoint {
			date: f.filing_date,
			value: (revenue - cor) / revenue,
		});
	}
	results
}

/// Net Profit Margin: `netIncome / revenue`.
pub fn net_margin(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let ni = match f.data.net_income {
			Some(v) => v,
			None => continue,
		};
		let revenue = match f.data.revenue {
			Some(v) if v > 0.0 => v,
			_ => continue,
		};
		results.push(FactorPoint {
			date: f.filing_date,
			value: ni / revenue,
		});
	}
	results
}

/// Operating Profit Margin: `operatingIncome / revenue`.
pub fn operating_profit_margin(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let oi = match f.data.operating_income {
			Some(v) => v,
			None => continue,
		};
		let revenue = match f.data.revenue {
			Some(v) if v > 0.0 => v,
			_ => continue,
		};
		results.push(FactorPoint {
			date: f.filing_date,
			value: oi / revenue,
		});
	}
	results
}

/// EBITDA Margin: `ebitda / revenue`. Falls back to `operatingIncome / revenue`.
pub fn ebitda_margin(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let ebitda = match f.data.ebitda {
			Some(v) => v,
			None => match f.data.operating_income {
				Some(v) => v,
				None => continue,
			},
		};
		let revenue = match f.data.revenue {
			Some(v) if v > 0.0 => v,
			_ => continue,
		};
		results.push(FactorPoint {
			date: f.filing_date,
			value: ebitda / revenue,
		});
	}
	results
}

/// Asset Turnover: `revenue / totalAssets`.
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
			date: f.filing_date,
			value: revenue / ta,
		});
	}
	results
}

/// Working Capital: `currentAssets - currentLiabilities`.
pub fn working_capital(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let ca = match f.data.current_assets {
			Some(v) => v,
			None => continue,
		};
		let cl = match f.data.current_liabilities {
			Some(v) => v,
			None => continue,
		};
		results.push(FactorPoint {
			date: f.filing_date,
			value: ca - cl,
		});
	}
	results
}

/// Working Capital Turnover: `revenue / (currentAssets - currentLiabilities)`.
pub fn working_capital_turnover(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let revenue = match f.data.revenue {
			Some(v) => v,
			None => continue,
		};
		let ca = match f.data.current_assets {
			Some(v) => v,
			None => continue,
		};
		let cl = match f.data.current_liabilities {
			Some(v) => v,
			None => continue,
		};
		let wc = ca - cl;
		if wc <= 0.0 {
			continue;
		}
		results.push(FactorPoint {
			date: f.filing_date,
			value: revenue / wc,
		});
	}
	results
}

/// Quality of Earnings Index (0-1): composite of accruals, cash flow consistency,
/// revenue quality, and balance sheet strength.
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
			date: f.filing_date,
			value: index,
		});
	}
	results
}

/// Retained Earnings Per Share: `retainedEarnings / sharesOutstanding`.
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
			date: f.filing_date,
			value: re / shares,
		});
	}
	results
}

/// R&D-to-Revenue ratio: `researchAndDevelopmentExpenses / revenue`.
pub fn r_and_d_to_revenue(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let rd = match f.data.research_and_development_expenses {
			Some(v) => v,
			None => continue,
		};
		let revenue = match f.data.revenue {
			Some(v) if v > 0.0 => v,
			_ => continue,
		};
		results.push(FactorPoint {
			date: f.filing_date,
			value: rd / revenue,
		});
	}
	results
}

/// Placeholder — requires price data not available in current IFundamentalPoint.
pub fn historical_volatility_vs_beta(_fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	Vec::new()
}

/// Debt-to-Equity ratio: `totalLiabilities / shareholdersEquity`.
pub fn debt_to_equity(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let liab = match f.data.total_liabilities {
			Some(v) => v,
			None => continue,
		};
		let equity = match f.data.shareholders_equity {
			Some(v) if v > 0.0 => v,
			_ => continue,
		};
		results.push(FactorPoint {
			date: f.filing_date,
			value: liab / equity,
		});
	}
	results
}

/// EBITDAR: `operatingIncome + depreciationAndAmortization` (adds back rent proxy).
pub fn ebitdar(fundamentals: Vec<FundamentalPoint>) -> Vec<FactorPoint> {
	let mut results = Vec::new();
	for f in &fundamentals {
		let oi = match f.data.operating_income {
			Some(v) => v,
			None => continue,
		};
		let da = if let Some(e) = f.data.ebitda {
			(e - oi).max(0.0)
		} else {
			0.0
		};
		results.push(FactorPoint {
			date: f.filing_date,
			value: oi + da,
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
