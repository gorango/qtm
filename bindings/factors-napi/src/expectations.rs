use factors_core;
use napi_derive::napi;

#[napi]
pub fn analyst_rating_momentum(
	fundamentals: Vec<factors_core::FundamentalPoint>,
	period: Option<u32>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::analyst_rating_momentum(fundamentals, period)
}

#[napi]
pub fn analyst_target_upside(
	fundamentals: Vec<factors_core::FundamentalPoint>,
	prices: Vec<factors_core::Bar>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::analyst_target_upside(fundamentals, prices)
}
