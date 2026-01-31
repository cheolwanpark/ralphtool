//! Ralph abstraction layer for spec system adapters.
//!
//! This module defines traits and domain types that represent Ralph workflow concepts.
//! Adapters implement these traits to provide task, story, and progress data from
//! their respective spec systems (OpenSpec, SpecKit, etc.).

mod traits;
mod types;
pub mod openspec;

pub use traits::*;
pub use types::*;
