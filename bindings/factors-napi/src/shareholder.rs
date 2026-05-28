use factors_core;
use napi_derive::napi;

#[napi]
pub fn shareholder_yield(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::shareholder_yield(fundamentals)
}

#[napi]
pub fn dividend_payout_ratio(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::dividend_payout_ratio(fundamentals)
}

#[napi]
pub fn dividend_coverage_ratio(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::dividend_coverage_ratio(fundamentals)
}

#[napi]
pub fn dividend_positive_10_years(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::dividend_positive_10_years(fundamentals)
}
