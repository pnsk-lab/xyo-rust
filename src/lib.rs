//! Scratch Native Runtime - A native Scratch project execution engine.
//!
//! This library provides functionality to load, compile, and execute Scratch
//! projects (`.sb3` files) by compiling them to native code via LLVM IR.
//!
//! # Architecture
//!
//! - **Project Loading** ([`project`]) - Parses `.sb3` files and extracts project metadata
//! - **IR Generation** ([`engine::ir`]) - Lowers Scratch blocks to an intermediate representation
//! - **JIT Compilation** ([`engine::jit`]) - Emits LLVM IR and compiles to native code
//! - **Runtime** ([`engine::runtime`]) - Provides execution context and host functions
//! - **GUI Frontend** ([`frontend::gui`]) - Real-time stage rendering using egui
//!
//! # Example
//!
//! ```rust,no_run
//! use scratch_native_runtime::{project::sb3, engine::{ir, jit, runtime}};
//!
//! // Load a Scratch project
//! let project = sb3::load_project_from_sb3("project.sb3")?;
//!
//! // Lower to IR
//! let program = ir::lower_project(&project);
//!
//! // Compile and execute
//! let module = jit::compile_program(&program)?;
//! let mut state = runtime::RuntimeState::new(/* config */);
//! // ... execute scripts
//! # Ok::<(), anyhow::Error>(())
//! ```

pub mod constants;
pub mod engine;
pub mod frontend;
pub mod project;
pub mod utils;

pub use constants::*;
