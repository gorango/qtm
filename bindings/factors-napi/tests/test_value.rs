use factors::{
	book_value_per_share, cash_to_market_cap, debt_service_coverage_ratio, dividend_yield,
	earnings_yield, enterprise_value_to_ebitda, free_cash_flow_margin, free_cash_flow_yield,
	margin_of_safety, market_cap_value, net_debt_to_ebitda, net_debt_to_ebitdar, owner_earnings,
	price_to_earnings, price_to_free_cash_flow, price_to_sales, wacc, Bar, FundamentalPoint,
	FundamentalPointData,
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

#[test]
fn test_price_to_earnings_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"Q1",
		FundamentalPointData {
			eps: Some(2.0),
			..Default::default()
		},
	);
	let prices = vec![make_bar(100.0, 150.0)];
	let result = price_to_earnings(vec![fp], prices);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 75.0).abs() < 1e-6);
}

#[test]
fn test_price_to_earnings_skips_non_positive_eps() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"Q1",
		FundamentalPointData {
			eps: Some(-1.0),
			..Default::default()
		},
	);
	let prices = vec![make_bar(100.0, 150.0)];
	let result = price_to_earnings(vec![fp], prices);
	assert!(result.is_empty());
}

#[test]
fn test_price_to_earnings_skips_no_eps() {
	let fp = make_fp("AAPL", 100.0, 100.0, "Q1", FundamentalPointData::default());
	let prices = vec![make_bar(100.0, 150.0)];
	let result = price_to_earnings(vec![fp], prices);
	assert!(result.is_empty());
}

#[test]
fn test_price_to_earnings_no_price_after_filing() {
	let fp = make_fp(
		"AAPL",
		100.0,
		200.0,
		"Q1",
		FundamentalPointData {
			eps: Some(2.0),
			..Default::default()
		},
	);
	let prices = vec![make_bar(100.0, 150.0)];
	let result = price_to_earnings(vec![fp], prices);
	assert!(result.is_empty());
}

#[test]
fn test_price_to_sales_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"Q1",
		FundamentalPointData {
			market_cap: Some(200_000.0),
			revenue: Some(50_000.0),
			..Default::default()
		},
	);
	let result = price_to_sales(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 4.0).abs() < 1e-6);
}

#[test]
fn test_price_to_sales_skips_no_revenue() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"Q1",
		FundamentalPointData {
			market_cap: Some(200_000.0),
			..Default::default()
		},
	);
	let result = price_to_sales(vec![fp]);
	assert!(result.is_empty());
}

#[test]
fn test_free_cash_flow_yield_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"Q1",
		FundamentalPointData {
			market_cap: Some(500_000.0),
			operating_cash_flow: Some(100_000.0),
			capital_expenditure: Some(20_000.0),
			..Default::default()
		},
	);
	let result = free_cash_flow_yield(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 0.16).abs() < 1e-6);
}

#[test]
fn test_earnings_yield_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"Q1",
		FundamentalPointData {
			eps: Some(5.0),
			market_cap: Some(1000.0),
			shares_outstanding: Some(100.0),
			..Default::default()
		},
	);
	let result = earnings_yield(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 0.5).abs() < 1e-6);
}

#[test]
fn test_dividend_yield_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"Q1",
		FundamentalPointData {
			dividends_per_share: Some(1.0),
			..Default::default()
		},
	);
	let prices = vec![make_bar(100.0, 50.0)];
	let result = dividend_yield(vec![fp], prices);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 0.02).abs() < 1e-6);
}

#[test]
fn test_margin_of_safety_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"Q1",
		FundamentalPointData {
			dcf: Some(200.0),
			..Default::default()
		},
	);
	let prices = vec![make_bar(100.0, 100.0)];
	let result = margin_of_safety(vec![fp], prices);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 1.0).abs() < 1e-6);
}

#[test]
fn test_enterprise_value_to_ebitda_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"Q1",
		FundamentalPointData {
			enterprise_value: Some(500_000.0),
			ebitda: Some(100_000.0),
			..Default::default()
		},
	);
	let result = enterprise_value_to_ebitda(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 5.0).abs() < 1e-6);
}

#[test]
fn test_owner_earnings_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"Q1",
		FundamentalPointData {
			operating_cash_flow: Some(100.0),
			capital_expenditure: Some(30.0),
			..Default::default()
		},
	);
	let result = owner_earnings(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 70.0).abs() < 1e-6);
}

#[test]
fn test_cash_to_market_cap_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"Q1",
		FundamentalPointData {
			cash_and_equivalents: Some(50.0),
			market_cap: Some(200.0),
			..Default::default()
		},
	);
	let result = cash_to_market_cap(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 0.25).abs() < 1e-6);
}

#[test]
fn test_market_cap_value_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"Q1",
		FundamentalPointData {
			market_cap: Some(500_000.0),
			..Default::default()
		},
	);
	let result = market_cap_value(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 500_000.0).abs() < 1e-6);
}

#[test]
fn test_market_cap_value_no_data() {
	let fp = make_fp("AAPL", 100.0, 100.0, "Q1", FundamentalPointData::default());
	let result = market_cap_value(vec![fp]);
	assert!(result.is_empty());
}

#[test]
fn test_net_debt_to_ebitda_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"Q1",
		FundamentalPointData {
			total_debt: Some(200.0),
			cash_and_equivalents: Some(50.0),
			ebitda: Some(30.0),
			..Default::default()
		},
	);
	let result = net_debt_to_ebitda(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 5.0).abs() < 1e-6);
}

#[test]
fn test_net_debt_to_ebitda_no_debt() {
	let fp = make_fp("AAPL", 100.0, 100.0, "Q1", FundamentalPointData::default());
	let result = net_debt_to_ebitda(vec![fp]);
	assert!(result.is_empty());
}

#[test]
fn test_book_value_per_share_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"Q1",
		FundamentalPointData {
			shareholders_equity: Some(500.0),
			shares_outstanding: Some(100.0),
			..Default::default()
		},
	);
	let result = book_value_per_share(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 5.0).abs() < 1e-6);
}

#[test]
fn test_wacc_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"Q1",
		FundamentalPointData {
			market_cap: Some(800.0),
			total_debt: Some(200.0),
			..Default::default()
		},
	);
	let result = wacc(vec![fp]);
	assert_eq!(result.len(), 1);
	// e_weight = 0.8, d_weight = 0.2
	// wacc = 0.8*0.08 + 0.2*0.04*(1-0.21) = 0.064 + 0.00632 = 0.07032
	assert!((result[0].value - 0.07032).abs() < 1e-6);
}

#[test]
fn test_debt_service_coverage_ratio_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"Q1",
		FundamentalPointData {
			operating_income: Some(100.0),
			interest_expense: Some(20.0),
			..Default::default()
		},
	);
	let result = debt_service_coverage_ratio(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 5.0).abs() < 1e-6);
}

#[test]
fn test_free_cash_flow_margin_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"Q1",
		FundamentalPointData {
			operating_cash_flow: Some(100.0),
			capital_expenditure: Some(20.0),
			revenue: Some(400.0),
			..Default::default()
		},
	);
	let result = free_cash_flow_margin(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 0.2).abs() < 1e-6);
}

#[test]
fn test_price_to_free_cash_flow_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"Q1",
		FundamentalPointData {
			market_cap: Some(1000.0),
			operating_cash_flow: Some(200.0),
			capital_expenditure: Some(50.0),
			..Default::default()
		},
	);
	let result = price_to_free_cash_flow(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 6.666666).abs() < 1e-3);
}

#[test]
fn test_net_debt_to_ebitdar_happy_path() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"Q1",
		FundamentalPointData {
			total_debt: Some(200.0),
			cash_and_equivalents: Some(50.0),
			operating_income: Some(80.0),
			ebitda: Some(100.0),
			..Default::default()
		},
	);
	let result = net_debt_to_ebitdar(vec![fp]);
	assert_eq!(result.len(), 1);
	assert!((result[0].value - 1.5).abs() < 1e-6);
}

#[test]
fn test_skips_when_no_prices() {
	let fp = make_fp(
		"AAPL",
		100.0,
		100.0,
		"Q1",
		FundamentalPointData {
			eps: Some(2.0),
			..Default::default()
		},
	);
	let result = price_to_earnings(vec![fp], vec![]);
	assert!(result.is_empty());
}

#[test]
fn test_empty_fundamentals() {
	let result = price_to_sales(vec![]);
	assert!(result.is_empty());

	let result = enterprise_value_to_ebitda(vec![]);
	assert!(result.is_empty());
}
