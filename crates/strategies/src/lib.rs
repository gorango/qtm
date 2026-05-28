pub mod error;
pub use error::{StrategyError, StrategyResult};

pub mod buy_and_hold;
pub mod composite;
pub mod fundamentals;
pub mod momentum;
pub mod patterns;
pub mod quantamentals;
pub mod registry;
pub mod statistics;
pub mod trend;
pub mod types;
pub mod utils;
pub mod volatility;
pub mod volume;

pub use buy_and_hold::*;
pub use composite::*;
pub use fundamentals::*;
pub use momentum::*;
pub use patterns::*;
pub use quantamentals::*;
pub use statistics::*;
pub use trend::*;
pub use volatility::*;
pub use volume::*;

pub use types::configs::*;
pub use types::results::*;
pub use utils::*;
