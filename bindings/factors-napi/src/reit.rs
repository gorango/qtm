use factors_core;
use napi_derive::napi;

#[napi]
pub fn price_to_affo(
	fundamentals: Vec<factors_core::FundamentalPoint>,
	prices: Vec<factors_core::Bar>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::price_to_affo(fundamentals, prices)
}

#[napi]
pub fn reit_dividend_safety(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::reit_dividend_safety(fundamentals)
}
