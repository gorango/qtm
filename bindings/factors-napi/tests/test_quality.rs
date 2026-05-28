use factors::{
	debt_to_equity, ebitdar, gross_margin, net_margin, operating_profit_margin,
	quality_of_earnings_index, r_and_d_to_revenue, retained_earnings, return_on_assets,
	return_on_equity, return_on_invested_capital, working_capital, working_capital_turnover,
	FundamentalPoint, FundamentalPointData,
};

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

#[test]
fn test_return_on_equity_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"FY",
		FundamentalPointData {
			net_income: Some(50.0),
			shareholders_equity: Some(250.0),
			..Default::default()
		},
	);
	let result = return_on_equity(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 0.2).abs() < 1e-6);
}

#[test]
fn test_return_on_equity_skips_no_equity() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"FY",
		FundamentalPointData {
			net_income: Some(50.0),
			..Default::default()
		},
	);
	let result = return_on_equity(vec![fp]);
	assert!(result.is_empty());
}

#[test]
fn test_return_on_assets_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"FY",
		FundamentalPointData {
			net_income: Some(30.0),
			total_assets: Some(300.0),
			..Default::default()
		},
	);
	let result = return_on_assets(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 0.1).abs() < 1e-6);
}

#[test]
fn test_gross_margin_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"FY",
		FundamentalPointData {
			revenue: Some(1000.0),
			cost_of_revenue: Some(600.0),
			..Default::default()
		},
	);
	let result = gross_margin(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 0.4).abs() < 1e-6);
}

#[test]
fn test_net_margin_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"FY",
		FundamentalPointData {
			net_income: Some(200.0),
			revenue: Some(1000.0),
			..Default::default()
		},
	);
	let result = net_margin(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 0.2).abs() < 1e-6);
}

#[test]
fn test_operating_profit_margin_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"FY",
		FundamentalPointData {
			operating_income: Some(250.0),
			revenue: Some(1000.0),
			..Default::default()
		},
	);
	let result = operating_profit_margin(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 0.25).abs() < 1e-6);
}

#[test]
fn test_quality_of_earnings_index_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"FY",
		FundamentalPointData {
			net_income: Some(100.0),
			operating_cash_flow: Some(120.0),
			total_assets: Some(1000.0),
			revenue: Some(500.0),
			gross_profit: Some(300.0),
			current_assets: Some(400.0),
			current_liabilities: Some(200.0),
			..Default::default()
		},
	);
	let result = quality_of_earnings_index(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!(result[0].value > 0.0 && result[0].value <= 1.0);
}

#[test]
fn test_quality_of_earnings_index_no_data() {
	let fp = make_fp("AAPL", 100.0, 100.0, "FY", FundamentalPointData::default());
	let result = quality_of_earnings_index(vec![fp]);
	assert!(result.is_empty());
}

#[test]
fn test_debt_to_equity_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"FY",
		FundamentalPointData {
			total_liabilities: Some(300.0),
			shareholders_equity: Some(100.0),
			..Default::default()
		},
	);
	let result = debt_to_equity(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 3.0).abs() < 1e-6);
}

#[test]
fn test_working_capital_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"FY",
		FundamentalPointData {
			current_assets: Some(500.0),
			current_liabilities: Some(300.0),
			..Default::default()
		},
	);
	let result = working_capital(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 200.0).abs() < 1e-6);
}

#[test]
fn test_working_capital_turnover_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"FY",
		FundamentalPointData {
			revenue: Some(1000.0),
			current_assets: Some(500.0),
			current_liabilities: Some(300.0),
			..Default::default()
		},
	);
	let result = working_capital_turnover(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 5.0).abs() < 1e-6);
}

#[test]
fn test_retained_earnings_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"FY",
		FundamentalPointData {
			retained_earnings: Some(500.0),
			shares_outstanding: Some(100.0),
			..Default::default()
		},
	);
	let result = retained_earnings(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 5.0).abs() < 1e-6);
}

#[test]
fn test_r_and_d_to_revenue_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"FY",
		FundamentalPointData {
			research_and_development_expenses: Some(50.0),
			revenue: Some(500.0),
			..Default::default()
		},
	);
	let result = r_and_d_to_revenue(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 0.1).abs() < 1e-6);
}

#[test]
fn test_return_on_invested_capital_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"FY",
		FundamentalPointData {
			net_income: Some(100.0),
			dividends_per_share: Some(2.0),
			shares_outstanding: Some(10.0),
			total_liabilities: Some(400.0),
			shareholders_equity: Some(600.0),
			..Default::default()
		},
	);
	let result = return_on_invested_capital(vec![fp]);
	assert_eq!(result.len(), 1);
	// (100 - 20) / (400 + 600) = 80 / 1000 = 0.08
	assert!((result[0].value - 0.08).abs() < 1e-6);
}

#[test]
fn test_ebitdar_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"FY",
		FundamentalPointData {
			operating_income: Some(100.0),
			ebitda: Some(130.0),
			..Default::default()
		},
	);
	let result = ebitdar(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 130.0).abs() < 1e-6);
}

#[test]
fn test_ebitdar_falls_back_without_ebitda() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"FY",
		FundamentalPointData {
			operating_income: Some(100.0),
			..Default::default()
		},
	);
	let result = ebitdar(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 100.0).abs() < 1e-6);
}

#[test]
fn test_empty_fundamentals_quality() {
	assert!(return_on_equity(vec![]).is_empty());
	assert!(return_on_assets(vec![]).is_empty());
	assert!(gross_margin(vec![]).is_empty());
	assert!(net_margin(vec![]).is_empty());
	assert!(debt_to_equity(vec![]).is_empty());
}
