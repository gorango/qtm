use factors_core;
use napi_derive::napi;

#[napi]
pub fn price_to_earnings(
	fundamentals: Vec<factors_core::FundamentalPoint>,
	prices: Vec<factors_core::Bar>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::price_to_earnings(fundamentals, prices)
}

#[napi]
pub fn price_to_book(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::price_to_book(fundamentals)
}

#[napi]
pub fn price_to_sales(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::price_to_sales(fundamentals)
}

#[napi]
pub fn price_to_free_cash_flow(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::price_to_free_cash_flow(fundamentals)
}

#[napi]
pub fn enterprise_value_to_ebitda(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::enterprise_value_to_ebitda(fundamentals)
}

#[napi]
pub fn earnings_yield(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::earnings_yield(fundamentals)
}

#[napi]
pub fn dividend_yield(
	fundamentals: Vec<factors_core::FundamentalPoint>,
	prices: Vec<factors_core::Bar>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::dividend_yield(fundamentals, prices)
}

#[napi]
pub fn free_cash_flow_yield(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::free_cash_flow_yield(fundamentals)
}

#[napi]
pub fn free_cash_flow_margin(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::free_cash_flow_margin(fundamentals)
}

#[napi]
pub fn margin_of_safety(
	fundamentals: Vec<factors_core::FundamentalPoint>,
	prices: Vec<factors_core::Bar>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::margin_of_safety(fundamentals, prices)
}

#[napi]
pub fn owner_earnings(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::owner_earnings(fundamentals)
}

#[napi]
pub fn wacc(fundamentals: Vec<factors_core::FundamentalPoint>) -> Vec<factors_core::FactorPoint> {
	factors_core::wacc(fundamentals)
}

#[napi]
pub fn cash_to_market_cap(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::cash_to_market_cap(fundamentals)
}

#[napi]
pub fn market_cap_value(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::market_cap_value(fundamentals)
}

#[napi]
pub fn net_debt_to_ebitda(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::net_debt_to_ebitda(fundamentals)
}

#[napi]
pub fn net_debt_to_ebitdar(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::net_debt_to_ebitdar(fundamentals)
}

#[napi]
pub fn debt_service_coverage_ratio(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::debt_service_coverage_ratio(fundamentals)
}

#[napi]
pub fn book_value_per_share(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::book_value_per_share(fundamentals)
}

#[napi]
pub fn price_to_book_ratio(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::price_to_book_ratio(fundamentals)
}

#[napi]
pub fn price_to_earnings_ratio(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::price_to_earnings_ratio(fundamentals)
}
