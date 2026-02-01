//! Spec abstraction layer for spec system adapters.
//!
//! This module defines traits and domain types that abstract spec system operations.
//! Adapters implement these traits to provide task, story, context, and persistence
//! capabilities from their respective spec systems (OpenSpec, SpecKit, etc.).

mod traits;
mod types;
pub mod openspec;

pub use traits::*;
pub use types::*;
