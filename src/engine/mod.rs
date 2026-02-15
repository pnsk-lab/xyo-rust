//! Core execution engine for Scratch programs.
//!
//! This module contains:
//! - [`ir`] - Intermediate representation for Scratch blocks
//! - [`jit`] - JIT compiler that emits and executes LLVM IR
//! - [`runtime`] - Runtime state and host functions

pub mod ir;
pub mod jit;
pub mod runtime;
