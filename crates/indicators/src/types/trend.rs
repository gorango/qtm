#[cfg_attr(feature = "napi", napi_derive::napi)]
pub enum Trend {
	Falling = -1,
	Stable = 0,
	Rising = 1,
}
