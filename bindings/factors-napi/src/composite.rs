use factors_core;
use napi_derive::napi;

#[napi]
pub fn altman_z_score(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::altman_z_score(fundamentals)
}

#[napi]
pub fn magic_formula(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::magic_formula(fundamentals)
}

#[napi]
pub fn piotroski_f_score(
	fundamentals: Vec<factors_core::FundamentalPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::piotroski_f_score(fundamentals)
}
