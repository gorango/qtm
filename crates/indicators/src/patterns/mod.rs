// Pattern detectors deliberately take a flat list of optional tuning
// parameters (one `Option` per JSON-schema property in the registry). This is
// what lets the registry and bindings map config objects 1:1 onto the public
// API, so the argument count is an intentional part of the design.
#![allow(clippy::too_many_arguments)]

pub mod break_of_structure;
pub mod broadening;
pub mod bump_and_run;
pub mod candlesticks;
pub mod channels;
pub mod cup_and_handle;
pub mod diamond;
pub mod double_bottom;
pub mod double_top;
pub mod elliott_wave;
pub mod engulfing;
pub mod flags_pennants;
pub mod head_and_shoulders;
pub mod helpers;
pub mod island_reversal;
pub mod power_of_three;
pub mod rectangle;
pub mod rounding;
pub mod stars;
pub mod triangles;
pub mod triple_top_bottom;
pub mod wedges;

pub use break_of_structure::*;
pub use broadening::*;
pub use bump_and_run::*;
pub use candlesticks::*;
pub use channels::*;
pub use cup_and_handle::*;
pub use diamond::*;
pub use double_bottom::*;
pub use double_top::*;
pub use elliott_wave::*;
pub use engulfing::*;
pub use flags_pennants::*;
pub use head_and_shoulders::*;
pub use helpers::*;
pub use island_reversal::*;
pub use power_of_three::*;
pub use rectangle::*;
pub use rounding::*;
pub use stars::*;
pub use triangles::*;
pub use triple_top_bottom::*;
pub use wedges::*;
