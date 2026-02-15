//! Global constants for the Scratch runtime.

/// Stage width in pixels (Scratch coordinate system).
pub const STAGE_WIDTH: usize = 480;

/// Stage height in pixels (Scratch coordinate system).
pub const STAGE_HEIGHT: usize = 360;

/// Default FPS for GUI mode when not in turbo.
pub const DEFAULT_FPS: f64 = 60.0;

/// Default vsync FPS.
pub const DEFAULT_VSYNC_FPS: usize = 60;

/// Default window scale multiplier.
pub const DEFAULT_WINDOW_SCALE: usize = 2;

/// Maximum step budget for control_forever loops (prevents infinite loops in CLI).
pub const DEFAULT_STEP_BUDGET: usize = 100_000;

/// Environment variable name for debug mode.
pub const ENV_SCRATCH_DEBUG: &str = "SCRATCH_DEBUG";

/// Environment variable name for step budget override.
pub const ENV_SCRATCH_STEP_BUDGET: &str = "SCRATCH_STEP_BUDGET";

/// Environment variable name for message breakpoints.
pub const ENV_SCRATCH_BREAK_ON_MESSAGE: &str = "SCRATCH_BREAK_ON_MESSAGE";
