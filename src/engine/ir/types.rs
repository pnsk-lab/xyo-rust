//! Core IR types (Program, Script, Variables, Lists)

/// A complete Scratch program in IR form.
#[derive(Debug, Clone)]
pub struct Program {
    /// All scripts (hat blocks with bodies).
    pub scripts: Vec<Script>,
    /// Custom procedure definitions.
    pub procedures: Vec<Procedure>,
    /// Global and per-target variable definitions.
    pub variables: Vec<VariableDef>,
    /// Global and per-target list definitions.
    pub lists: Vec<ListDef>,
    /// Names of all targets (stage + sprites).
    pub target_names: Vec<String>,
    /// String interning table for string literals.
    pub strings: Vec<String>,
    /// Warnings generated during lowering (unsupported blocks, etc.).
    pub warnings: Vec<String>,
}

/// A variable definition with its initial value.
#[derive(Debug, Clone)]
pub struct VariableDef {
    /// Unique variable ID from the project.
    pub id: String,
    /// Display name of the variable.
    pub name: String,
    /// Owning target index in `Program::target_names`.
    pub target_index: usize,
    /// Initial scalar value.
    pub initial_value: ScalarValue,
}

/// A list definition with its initial values.
#[derive(Debug, Clone)]
pub struct ListDef {
    /// Unique list ID from the project.
    pub id: String,
    /// Display name of the list.
    pub name: String,
    /// Initial list contents.
    pub initial_values: Vec<ScalarValue>,
}

/// A scalar value that can be a number or a string (indexed).
#[derive(Debug, Clone)]
pub enum ScalarValue {
    Number(f64),
    String(usize),
}

/// A single script (hat block + body).
#[derive(Debug, Clone)]
pub struct Script {
    /// Human-readable script name for debugging.
    pub name: String,
    /// Name of the target this script belongs to.
    pub target_name: String,
    /// The event that triggers this script.
    pub trigger: ScriptTrigger,
    /// The script body (sequence of statements).
    pub body: Vec<super::Stmt>,
}

/// The event that triggers a script.
#[derive(Debug, Clone)]
pub enum ScriptTrigger {
    /// Triggered by clicking the green flag.
    GreenFlag,
    /// Triggered by receiving a broadcast message.
    Broadcast(String),
    /// Triggered by a key press.
    KeyPressed(String),
    /// Triggered when this sprite is cloned.
    CloneStart,
}

#[derive(Debug, Clone)]
pub struct Procedure {
    pub name: String,
    pub target_name: String,
    pub proccode: String,
    pub arg_names: Vec<String>,
    pub warp: bool,
    pub body: Vec<super::Stmt>,
}
