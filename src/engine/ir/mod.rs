//! Intermediate Representation (IR) for Scratch programs.
//!
//! This module lowers Scratch blocks from the `project::sb3` representation
//! into a simplified typed IR that is easier to compile to native code.
//!
//! # Overview
//!
//! The lowering process:
//! 1. Parses and validates block structures
//! 2. Resolves variable/list references
//! 3. Inlines simple expressions
//! 4. Flattens nested block trees into statement sequences
//!
//! # Main Types
//!
//! - [`Program`] - The complete IR program containing all scripts and data
//! - [`Script`] - A single hat block and its body (event trigger + statements)
//! - [`Stmt`] - A statement (block that performs an action)
//! - [`Expr`] - An expression (block that returns a value)

mod types;
mod stmt;
mod expr;
mod builder;

// Re-export public types
pub use types::*;
pub use stmt::*;
pub use expr::*;
pub use builder::lower_project;
