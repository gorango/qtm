use factors_core;
use napi_derive::napi;

#[napi]
pub fn debt_to_assets(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::debt_to_assets(fundamentals)
}

#[napi]
pub fn current_ratio(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::current_ratio(fundamentals)
}

#[napi]
pub fn interest_coverage(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::interest_coverage(fundamentals)
}

#[napi]
pub fn tangible_asset_ratio(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::tangible_asset_ratio(fundamentals)
}
