//! External runtime function modules organized by category

pub mod core;
pub mod motion;
pub mod variables;
pub mod sensing;
pub mod looks;
pub mod pen;
pub mod music;
pub mod control;
pub mod operators;
pub mod data;
pub mod events;
pub mod loops;

// Re-export all extern "C" functions for backward compatibility
pub use core::*;
pub use motion::*;
pub use variables::*;
pub use sensing::*;
pub use looks::*;
pub use pen::*;
pub use music::*;
pub use control::*;
pub use operators::*;
pub use data::*;
pub use events::*;
pub use loops::*;
