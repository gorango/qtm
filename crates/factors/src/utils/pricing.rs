use crate::types::data::Bar;

pub fn find_price_on_or_after(prices: &[Bar], timestamp: f64) -> Option<f64> {
	prices.iter().find(|p| p.time >= timestamp).map(|p| p.close)
}

#[allow(dead_code)]
pub fn find_price_index(prices: &[Bar], timestamp: f64) -> Option<usize> {
	prices.iter().position(|p| p.time >= timestamp)
}
