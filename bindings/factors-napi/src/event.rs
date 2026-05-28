use factors_core;
use napi_derive::napi;

#[napi]
pub fn earnings_surprise(
	reports: Vec<factors_core::EarningsReportPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::earnings_surprise(reports)
}
