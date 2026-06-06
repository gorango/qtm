// ── Unified NAPI bindings ───────────────────────────
//
// This crate re-exports all bindings from the individual NAPI crates
// so that consumers can import a single `@quantamental/core` package.
//
// NAPI-rs 3.x discovers `#[napi]` annotations via linker sections, so
// functions in dependency crates are automatically registered when
// linked into this cdylib. The `pub use` below also prevents dead-code
// elimination from stripping the symbols.

extern crate factors;
extern crate indicators;
extern crate strategies;

pub use factors::*;
pub use indicators::*;
pub use strategies::*;
