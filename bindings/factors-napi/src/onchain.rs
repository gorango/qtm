use factors_core;
use napi_derive::napi;

#[napi]
pub fn active_address_growth(
	on_chain_data: Vec<factors_core::OnChainDataPoint>,
	period: Option<f64>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::active_address_growth(on_chain_data, period)
}

#[napi]
pub fn exchange_flow_momentum(
	on_chain_data: Vec<factors_core::OnChainDataPoint>,
	period: Option<f64>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::exchange_flow_momentum(on_chain_data, period)
}

#[napi]
pub fn nvt_ratio(
	on_chain_data: Vec<factors_core::OnChainDataPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::nvt_ratio(on_chain_data)
}

#[napi]
pub fn staking_ratio(
	on_chain_data: Vec<factors_core::OnChainDataPoint>,
) -> Vec<factors_core::FactorPoint> {
	factors_core::staking_ratio(on_chain_data)
}
