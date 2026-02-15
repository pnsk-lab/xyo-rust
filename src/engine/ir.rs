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

use crate::project::sb3::{Block, Project, Target};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

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
    /// Initial numeric value (strings are converted to 0.0).
    pub initial_value: f64,
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
    pub body: Vec<Stmt>,
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
pub enum CloneTarget {
    Myself,
    Target(usize),
}

#[derive(Debug, Clone, Copy)]
pub enum PenColorParam {
    Color,
    Saturation,
    Brightness,
    Transparency,
}

#[derive(Debug, Clone, Copy)]
pub enum ControlStopMode {
    ThisScript,
    All,
    OtherScriptsInTarget,
}

#[derive(Debug, Clone)]
pub struct Procedure {
    pub name: String,
    pub target_name: String,
    pub proccode: String,
    pub arg_names: Vec<String>,
    pub warp: bool,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    MotionMoveSteps(Expr),
    MotionSetDirection(Expr),
    MotionChangeX(Expr),
    MotionChangeY(Expr),
    MotionSetX(Expr),
    MotionSetY(Expr),
    MotionGoToXY {
        x: Expr,
        y: Expr,
    },
    DataSetVariable {
        variable_index: usize,
        value: Expr,
    },
    DataChangeVariable {
        variable_index: usize,
        delta: Expr,
    },
    DataReplaceListItem {
        list_index: usize,
        index: Expr,
        item: Expr,
    },
    DataAddToList {
        list_index: usize,
        item: Expr,
    },
    DataDeleteListItem {
        list_index: usize,
        index: Expr,
    },
    DataDeleteAllOfList {
        list_index: usize,
    },
    LooksSwitchCostumeTo(Expr),
    LooksSetSize(Expr),
    LooksShow,
    LooksSay(SayExpr),
    SoundPlay,
    ControlRepeat {
        times: Expr,
        body: Vec<Stmt>,
    },
    ControlWait {
        duration: Expr,
    },
    ControlWaitUntil {
        condition: Expr,
    },
    ControlForEach {
        variable_index: usize,
        count: Expr,
        body: Vec<Stmt>,
    },
    ControlForever {
        body: Vec<Stmt>,
    },
    ControlRepeatUntil {
        condition: Expr,
        body: Vec<Stmt>,
    },
    ControlWhile {
        condition: Expr,
        body: Vec<Stmt>,
    },
    ControlIf {
        condition: Expr,
        then_body: Vec<Stmt>,
        else_body: Vec<Stmt>,
    },
    MotionSetRotationStyle,
    DataShowVariable,
    ControlStop {
        mode: ControlStopMode,
    },
    ControlCreateCloneOf {
        target: CloneTarget,
    },
    ControlDeleteThisClone,
    SensingAskAndWait {
        question: Expr,
    },
    MusicSetTempo {
        tempo: Expr,
    },
    SensingResetTimer,
    LooksHide,
    SensingSetDragMode,
    TextToSpeechSpeakAndWait(Expr),
    PenDown,
    PenUp,
    PenClear,
    PenSetSize(Expr),
    PenSetColor(Expr),
    PenStamp,
    PenSetColorParam {
        param: PenColorParam,
        value: Expr,
    },
    EventBroadcast {
        message: Expr,
        wait: bool,
    },
    ProcedureCall {
        procedure_index: usize,
        args: Vec<Expr>,
    },
    ProcedureReturn {
        value: Expr,
    },
}

#[derive(Debug, Clone)]
pub enum SayExpr {
    Text(String),
    Numeric(Expr),
}

#[derive(Debug, Clone)]
pub enum MathOp {
    Abs,
    Floor,
    Ceil,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Ln,
    Log,
    Exp,
    Exp10,
}

#[derive(Debug, Clone, Copy)]
pub enum SensingCurrentMenu {
    Year,
    Month,
    Date,
    DayOfWeek,
    Hour,
    Minute,
    Second,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    StringLiteral(usize),
    MotionXPosition,
    MotionYPosition,
    SensingMouseX,
    SensingMouseY,
    SensingMouseDown,
    LooksCostumeNumber,
    LooksCostumeName,
    SensingOf {
        object: Box<Expr>,
        property: usize,
    },
    SensingCurrent(SensingCurrentMenu),
    Variable(usize),
    ProcedureArg(usize),
    ProcedureCall {
        procedure_index: usize,
        args: Vec<Expr>,
    },
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    GreaterThan(Box<Expr>, Box<Expr>),
    LessThan(Box<Expr>, Box<Expr>),
    Equals(Box<Expr>, Box<Expr>),
    Random(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    StringLength(Box<Expr>),
    StringJoin(Box<Expr>, Box<Expr>),
    StringContains(Box<Expr>, Box<Expr>),
    Round(Box<Expr>),
    LetterOf {
        letter: Box<Expr>,
        string: Box<Expr>,
    },
    ListItem {
        list_index: usize,
        index: Box<Expr>,
    },
    ListItemNum {
        list_index: usize,
        item: Box<Expr>,
    },
    ListLength {
        list_index: usize,
    },
    ListContainsItem {
        list_index: usize,
        item: Box<Expr>,
    },
    KeyPressed(Box<Expr>),
    SensingAnswer,
    SensingTimer,
    SensingDaysSince2000,
    SensingTouchingObject(Box<Expr>),
    MathOp {
        op: MathOp,
        value: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
struct ProcedureDefSite {
    procedure_index: usize,
    target_index: usize,
    definition_id: String,
}

pub fn lower_project(project: &Project) -> Program {
    ProgramBuilder::new(project).build()
}

struct ProgramBuilder<'a> {
    project: &'a Project,
    variables: Vec<VariableDef>,
    variable_index: HashMap<String, usize>,
    lists: Vec<ListDef>,
    list_index: HashMap<String, usize>,
    target_index_by_name: HashMap<String, usize>,
    procedures: Vec<Procedure>,
    procedure_index_by_key: HashMap<String, usize>,
    procedure_index_by_fuzzy_key: HashMap<String, usize>,
    procedure_sites: Vec<ProcedureDefSite>,
    strings: Vec<String>,
    string_index: HashMap<String, usize>,
    warnings: Vec<String>,
    seen_warnings: HashSet<String>,
    current_proc_args: Option<HashMap<String, usize>>,
}

impl<'a> ProgramBuilder<'a> {
    fn new(project: &'a Project) -> Self {
        Self {
            project,
            variables: Vec::new(),
            variable_index: HashMap::new(),
            lists: Vec::new(),
            list_index: HashMap::new(),
            target_index_by_name: project
                .targets
                .iter()
                .enumerate()
                .map(|(index, target)| (target.name.clone(), index))
                .collect(),
            procedures: Vec::new(),
            procedure_index_by_key: HashMap::new(),
            procedure_index_by_fuzzy_key: HashMap::new(),
            procedure_sites: Vec::new(),
            strings: vec![String::new()],
            string_index: {
                let mut map = HashMap::new();
                map.insert(String::new(), 0);
                map
            },
            warnings: Vec::new(),
            seen_warnings: HashSet::new(),
            current_proc_args: None,
        }
    }

    fn build(mut self) -> Program {
        self.collect_variables();
        self.collect_lists();
        self.collect_procedure_signatures();
        self.lower_procedure_bodies();

        let mut scripts = Vec::new();
        for (target_index, target) in self.project.targets.iter().enumerate() {
            let mut hats: Vec<String> = target
                .blocks
                .iter()
                .filter(|(_, block)| {
                    block.top_level
                        && (block.opcode == "event_whenflagclicked"
                            || block.opcode == "event_whenbroadcastreceived"
                            || block.opcode == "event_whenkeypressed"
                            || block.opcode == "event_whencloned")
                })
                .map(|(id, _)| id.clone())
                .collect();
            hats.sort();

            for (index, hat_id) in hats.iter().enumerate() {
                if let Some(script) = self.lower_hat_script(target, target_index, hat_id, index) {
                    scripts.push(script);
                }
            }
        }

        Program {
            scripts,
            procedures: self.procedures,
            variables: self.variables,
            lists: self.lists,
            target_names: self
                .project
                .targets
                .iter()
                .map(|target| target.name.clone())
                .collect(),
            strings: self.strings,
            warnings: self.warnings,
        }
    }

    fn collect_variables(&mut self) {
        for (target_index, target) in self.project.targets.iter().enumerate() {
            for (var_id, raw) in &target.variables {
                if self.variable_index.contains_key(var_id) {
                    continue;
                }
                let (name, initial_value) = parse_variable(raw);
                let index = self.variables.len();
                self.variables.push(VariableDef {
                    id: var_id.clone(),
                    name,
                    target_index,
                    initial_value,
                });
                self.variable_index.insert(var_id.clone(), index);
            }
        }
    }

    fn collect_lists(&mut self) {
        for target in &self.project.targets {
            for (list_id, raw) in &target.lists {
                if self.list_index.contains_key(list_id) {
                    continue;
                }

                let (name, initial_values) = self.parse_list(raw);
                let index = self.lists.len();
                self.lists.push(ListDef {
                    id: list_id.clone(),
                    name,
                    initial_values,
                });
                self.list_index.insert(list_id.clone(), index);
            }
        }
    }

    fn collect_procedure_signatures(&mut self) {
        for (target_index, target) in self.project.targets.iter().enumerate() {
            let mut definitions: Vec<(String, &Block)> = target
                .blocks
                .iter()
                .filter(|(_, block)| block.opcode == "procedures_definition")
                .map(|(id, block)| (id.clone(), block))
                .collect();
            definitions.sort_by(|(a, _), (b, _)| a.cmp(b));

            for (definition_id, definition) in definitions {
                let Some(prototype_id) = self
                    .input_payload(definition, "custom_block")
                    .and_then(Value::as_str)
                else {
                    self.warn_once(format!(
                        "procedure definition {} missing custom_block input",
                        definition_id
                    ));
                    continue;
                };

                let Some(prototype) = target.blocks.get(prototype_id) else {
                    self.warn_once(format!(
                        "procedure definition {} references missing prototype {}",
                        definition_id, prototype_id
                    ));
                    continue;
                };

                let Some(mutation) = prototype.mutation.as_ref() else {
                    self.warn_once(format!(
                        "procedure prototype {} missing mutation",
                        prototype_id
                    ));
                    continue;
                };

                let Some(proccode) = mutation_string(mutation, "proccode") else {
                    self.warn_once(format!(
                        "procedure prototype {} missing proccode",
                        prototype_id
                    ));
                    continue;
                };

                let argument_names =
                    mutation_string_array(mutation, "argumentnames").unwrap_or_default();
                let warp = mutation_bool(mutation, "warp").unwrap_or(false);
                let symbol = format!(
                    "proc_{}_{}_{}",
                    sanitize_symbol(&target.name),
                    sanitize_symbol(&proccode),
                    self.procedures.len()
                );
                let procedure_index = self.procedures.len();

                self.procedures.push(Procedure {
                    name: symbol,
                    target_name: target.name.clone(),
                    proccode: proccode.clone(),
                    arg_names: argument_names,
                    warp,
                    body: Vec::new(),
                });

                let key = procedure_key(target, &proccode);
                self.procedure_index_by_key.insert(key, procedure_index);
                let fuzzy_key = procedure_fuzzy_key(target, &proccode);
                self.procedure_index_by_fuzzy_key
                    .entry(fuzzy_key)
                    .or_insert(procedure_index);
                self.procedure_sites.push(ProcedureDefSite {
                    procedure_index,
                    target_index,
                    definition_id,
                });
            }
        }
    }

    fn lower_procedure_bodies(&mut self) {
        for site in self.procedure_sites.clone() {
            let Some(target) = self.project.targets.get(site.target_index) else {
                continue;
            };
            let Some(definition) = target.blocks.get(&site.definition_id) else {
                self.warn_once(format!(
                    "missing procedure definition block {}",
                    site.definition_id
                ));
                continue;
            };

            let arg_map = self.procedures[site.procedure_index]
                .arg_names
                .iter()
                .enumerate()
                .map(|(index, name)| (name.clone(), index))
                .collect::<HashMap<_, _>>();

            self.current_proc_args = Some(arg_map);
            let body = self.lower_statement_chain(target, definition.next.as_deref());
            self.current_proc_args = None;

            if let Some(procedure) = self.procedures.get_mut(site.procedure_index) {
                procedure.body = body;
            }
        }
    }

    fn lower_hat_script(
        &mut self,
        target: &Target,
        target_index: usize,
        hat_id: &str,
        script_index: usize,
    ) -> Option<Script> {
        let hat = target.blocks.get(hat_id)?;
        let trigger = match hat.opcode.as_str() {
            "event_whenflagclicked" => ScriptTrigger::GreenFlag,
            "event_whenbroadcastreceived" => {
                let message = self
                    .field_value_as_string(hat, "BROADCAST_OPTION")
                    .unwrap_or_else(|| "message1".to_string());
                ScriptTrigger::Broadcast(message)
            }
            "event_whenkeypressed" => {
                let key = self
                    .field_value_as_string(hat, "KEY_OPTION")
                    .unwrap_or_else(|| "space".to_string());
                ScriptTrigger::KeyPressed(normalize_key_name(&key))
            }
            "event_whencloned" => ScriptTrigger::CloneStart,
            _ => return None,
        };

        let body = self.lower_statement_chain(target, hat.next.as_deref());
        if body.is_empty() {
            return None;
        }

        let target_kind = if target.is_stage { "stage" } else { "sprite" };
        let trigger_name = match &trigger {
            ScriptTrigger::GreenFlag => "greenflag".to_string(),
            ScriptTrigger::Broadcast(message) => {
                format!("broadcast_{}", sanitize_symbol(message))
            }
            ScriptTrigger::KeyPressed(key) => {
                format!("keypress_{}", sanitize_symbol(key))
            }
            ScriptTrigger::CloneStart => "clonestart".to_string(),
        };
        let script_name = format!(
            "{}_{}_{}_{}_{}",
            target_kind,
            target_index,
            sanitize_symbol(&target.name),
            trigger_name,
            script_index
        );
        Some(Script {
            name: script_name,
            target_name: target.name.clone(),
            trigger,
            body,
        })
    }

    fn lower_statement_chain(&mut self, target: &Target, start_id: Option<&str>) -> Vec<Stmt> {
        let mut statements = Vec::new();
        let mut visited = HashSet::new();
        let mut cursor = start_id.map(str::to_string);

        while let Some(block_id) = cursor {
            if !visited.insert(block_id.clone()) {
                self.warn_once(format!("detected cyclic block chain at {}", block_id));
                break;
            }

            let Some(block) = target.blocks.get(&block_id) else {
                self.warn_once(format!("missing block reference {}", block_id));
                break;
            };

            if let Some(statement) = self.lower_statement(target, &block_id, block) {
                statements.push(statement);
            }

            cursor = block.next.clone();
        }

        statements
    }

    fn lower_statement(&mut self, target: &Target, block_id: &str, block: &Block) -> Option<Stmt> {
        match block.opcode.as_str() {
            "motion_movesteps" => Some(Stmt::MotionMoveSteps(
                self.lower_numeric_input(target, block, "STEPS"),
            )),
            "motion_pointindirection" => Some(Stmt::MotionSetDirection(self.lower_numeric_input(
                target,
                block,
                "DIRECTION",
            ))),
            "motion_changexby" => Some(Stmt::MotionChangeX(
                self.lower_numeric_input(target, block, "DX"),
            )),
            "motion_changeyby" => Some(Stmt::MotionChangeY(
                self.lower_numeric_input(target, block, "DY"),
            )),
            "motion_setx" => Some(Stmt::MotionSetX(
                self.lower_numeric_input(target, block, "X"),
            )),
            "motion_sety" => Some(Stmt::MotionSetY(
                self.lower_numeric_input(target, block, "Y"),
            )),
            "motion_gotoxy" => Some(Stmt::MotionGoToXY {
                x: self.lower_numeric_input(target, block, "X"),
                y: self.lower_numeric_input(target, block, "Y"),
            }),
            "data_setvariableto" => {
                let variable_id = self.variable_id_from_field(block, "VARIABLE")?;
                let Some(variable_index) = self.variable_index.get(&variable_id).copied() else {
                    self.warn_once(format!(
                        "unknown variable id {} in {}",
                        variable_id, block_id
                    ));
                    return None;
                };
                Some(Stmt::DataSetVariable {
                    variable_index,
                    value: self.lower_numeric_input(target, block, "VALUE"),
                })
            }
            "data_changevariableby" => {
                let variable_id = self.variable_id_from_field(block, "VARIABLE")?;
                let Some(variable_index) = self.variable_index.get(&variable_id).copied() else {
                    self.warn_once(format!(
                        "unknown variable id {} in {}",
                        variable_id, block_id
                    ));
                    return None;
                };
                Some(Stmt::DataChangeVariable {
                    variable_index,
                    delta: self.lower_numeric_input(target, block, "VALUE"),
                })
            }
            "data_replaceitemoflist" => {
                let list_id = self.list_id_from_field(block, "LIST")?;
                let Some(list_index) = self.list_index.get(&list_id).copied() else {
                    self.warn_once(format!("unknown list id {} in {}", list_id, block_id));
                    return None;
                };
                Some(Stmt::DataReplaceListItem {
                    list_index,
                    index: self.lower_numeric_input(target, block, "INDEX"),
                    item: self.lower_numeric_input(target, block, "ITEM"),
                })
            }
            "data_addtolist" => {
                let list_id = self.list_id_from_field(block, "LIST")?;
                let Some(list_index) = self.list_index.get(&list_id).copied() else {
                    self.warn_once(format!("unknown list id {} in {}", list_id, block_id));
                    return None;
                };
                Some(Stmt::DataAddToList {
                    list_index,
                    item: self.lower_numeric_input(target, block, "ITEM"),
                })
            }
            "data_deleteoflist" => {
                let list_id = self.list_id_from_field(block, "LIST")?;
                let Some(list_index) = self.list_index.get(&list_id).copied() else {
                    self.warn_once(format!("unknown list id {} in {}", list_id, block_id));
                    return None;
                };
                Some(Stmt::DataDeleteListItem {
                    list_index,
                    index: self.lower_numeric_input(target, block, "INDEX"),
                })
            }
            "data_deletealloflist" => {
                let list_id = self.list_id_from_field(block, "LIST")?;
                let Some(list_index) = self.list_index.get(&list_id).copied() else {
                    self.warn_once(format!("unknown list id {} in {}", list_id, block_id));
                    return None;
                };
                Some(Stmt::DataDeleteAllOfList { list_index })
            }
            "looks_switchcostumeto" => Some(Stmt::LooksSwitchCostumeTo(
                self.lower_numeric_input(target, block, "COSTUME"),
            )),
            "looks_setsizeto" => Some(Stmt::LooksSetSize(
                self.lower_numeric_input(target, block, "SIZE"),
            )),
            "looks_show" => Some(Stmt::LooksShow),
            "looks_say" => Some(Stmt::LooksSay(
                self.lower_say_input(target, block, "MESSAGE"),
            )),
            "sound_play" | "sound_playuntildone" => Some(Stmt::SoundPlay),
            "control_repeat" => Some(Stmt::ControlRepeat {
                times: self.lower_numeric_input(target, block, "TIMES"),
                body: self.lower_substack(target, block, "SUBSTACK"),
            }),
            "control_wait" => Some(Stmt::ControlWait {
                duration: self.lower_numeric_input(target, block, "DURATION"),
            }),
            "control_wait_until" => Some(Stmt::ControlWaitUntil {
                condition: self.lower_numeric_input(target, block, "CONDITION"),
            }),
            "control_for_each" => {
                let variable_id = self.variable_id_from_field(block, "VARIABLE")?;
                let Some(variable_index) = self.variable_index.get(&variable_id).copied() else {
                    self.warn_once(format!(
                        "unknown variable id {} in {}",
                        variable_id, block_id
                    ));
                    return None;
                };
                Some(Stmt::ControlForEach {
                    variable_index,
                    count: self.lower_numeric_input(target, block, "VALUE"),
                    body: self.lower_substack(target, block, "SUBSTACK"),
                })
            }
            "control_forever" => Some(Stmt::ControlForever {
                body: self.lower_substack(target, block, "SUBSTACK"),
            }),
            "control_repeat_until" => Some(Stmt::ControlRepeatUntil {
                condition: self.lower_numeric_input(target, block, "CONDITION"),
                body: self.lower_substack(target, block, "SUBSTACK"),
            }),
            "control_while" => Some(Stmt::ControlWhile {
                condition: self.lower_numeric_input(target, block, "CONDITION"),
                body: self.lower_substack(target, block, "SUBSTACK"),
            }),
            "control_if" => Some(Stmt::ControlIf {
                condition: self.lower_numeric_input(target, block, "CONDITION"),
                then_body: self.lower_substack(target, block, "SUBSTACK"),
                else_body: Vec::new(),
            }),
            "control_if_else" => Some(Stmt::ControlIf {
                condition: self.lower_numeric_input(target, block, "CONDITION"),
                then_body: self.lower_substack(target, block, "SUBSTACK"),
                else_body: self.lower_substack(target, block, "SUBSTACK2"),
            }),
            "event_broadcast" => Some(Stmt::EventBroadcast {
                message: self.lower_broadcast_expr(target, block),
                wait: false,
            }),
            "event_broadcastandwait" => Some(Stmt::EventBroadcast {
                message: self.lower_broadcast_expr(target, block),
                wait: true,
            }),
            "looks_hide" => Some(Stmt::LooksHide),
            "motion_setrotationstyle" => Some(Stmt::MotionSetRotationStyle),
            "data_showvariable" => Some(Stmt::DataShowVariable),
            "control_stop" => {
                let raw = self
                    .field_value_as_string(block, "STOP_OPTION")
                    .unwrap_or_else(|| "this script".to_string());
                Some(Stmt::ControlStop {
                    mode: parse_control_stop_mode(&raw),
                })
            }
            "control_create_clone_of" => {
                let Some(clone_target) = self.lower_clone_target(target, block, block_id) else {
                    return None;
                };
                Some(Stmt::ControlCreateCloneOf {
                    target: clone_target,
                })
            }
            "control_delete_this_clone" => Some(Stmt::ControlDeleteThisClone),
            "sensing_askandwait" => Some(Stmt::SensingAskAndWait {
                question: self.lower_numeric_input(target, block, "QUESTION"),
            }),
            "music_setTempo" => Some(Stmt::MusicSetTempo {
                tempo: self.lower_numeric_input(target, block, "TEMPO"),
            }),
            "sensing_resettimer" => Some(Stmt::SensingResetTimer),
            "sensing_setdragmode" => Some(Stmt::SensingSetDragMode),
            "text2speech_speakAndWait" => Some(Stmt::TextToSpeechSpeakAndWait(
                self.lower_numeric_input(target, block, "WORDS"),
            )),
            "pen_penDown" => Some(Stmt::PenDown),
            "pen_penUp" => Some(Stmt::PenUp),
            "pen_clear" => Some(Stmt::PenClear),
            "pen_setPenSizeTo" => Some(Stmt::PenSetSize(
                self.lower_numeric_input(target, block, "SIZE"),
            )),
            "pen_setPenColorToColor" => Some(Stmt::PenSetColor(
                self.lower_numeric_input(target, block, "COLOR"),
            )),
            "pen_stamp" => Some(Stmt::PenStamp),
            "pen_setPenColorParamTo" => Some(Stmt::PenSetColorParam {
                param: self.lower_pen_color_param(target, block),
                value: self.lower_numeric_input(target, block, "VALUE"),
            }),
            "procedures_return" => Some(Stmt::ProcedureReturn {
                value: self.lower_numeric_input(target, block, "VALUE"),
            }),
            "procedures_call" => self.lower_procedure_call(target, block, block_id),
            unsupported => {
                self.warn_once(format!("unsupported statement opcode: {}", unsupported));
                None
            }
        }
    }

    fn lower_procedure_call(
        &mut self,
        target: &Target,
        block: &Block,
        block_id: &str,
    ) -> Option<Stmt> {
        let (procedure_index, args) = self.lower_procedure_call_parts(target, block, block_id)?;
        Some(Stmt::ProcedureCall {
            procedure_index,
            args,
        })
    }

    fn lower_procedure_call_parts(
        &mut self,
        target: &Target,
        block: &Block,
        block_id: &str,
    ) -> Option<(usize, Vec<Expr>)> {
        let Some(mutation) = block.mutation.as_ref() else {
            self.warn_once(format!("procedure call {} missing mutation", block_id));
            return None;
        };
        let Some(proccode) = mutation_string(mutation, "proccode") else {
            self.warn_once(format!("procedure call {} missing proccode", block_id));
            return None;
        };

        let key = procedure_key(target, &proccode);
        let procedure_index = self.procedure_index_by_key.get(&key).copied().or_else(|| {
            let fuzzy_key = procedure_fuzzy_key(target, &proccode);
            self.procedure_index_by_fuzzy_key.get(&fuzzy_key).copied()
        });
        let Some(procedure_index) = procedure_index else {
            self.warn_once(format!(
                "procedure call {} references unknown procedure {}",
                block_id, proccode
            ));
            return None;
        };

        let arg_ids = mutation_string_array(mutation, "argumentids").unwrap_or_default();
        let args = arg_ids
            .iter()
            .map(|arg_id| self.lower_numeric_input(target, block, arg_id))
            .collect();

        Some((procedure_index, args))
    }

    fn lower_substack(&mut self, target: &Target, block: &Block, name: &str) -> Vec<Stmt> {
        let Some(payload) = self.input_payload(block, name) else {
            return Vec::new();
        };
        if let Some(block_id) = payload.as_str() {
            return self.lower_statement_chain(target, Some(block_id));
        }
        Vec::new()
    }

    fn lower_broadcast_expr(&mut self, target: &Target, block: &Block) -> Expr {
        if let Some(payload) = self.input_payload(block, "BROADCAST_INPUT") {
            return self.lower_numeric_payload(target, payload);
        }

        if let Some(message) = self.field_value_as_string(block, "BROADCAST_OPTION") {
            return self.string_literal(&message);
        }

        self.string_literal("")
    }

    fn lower_pen_color_param(&mut self, target: &Target, block: &Block) -> PenColorParam {
        if let Some(payload) = self.input_payload(block, "COLOR_PARAM") {
            if let Some(param) = self.parse_pen_color_param_payload(target, payload) {
                return param;
            }
        }

        self.field_value_as_string(block, "COLOR_PARAM")
            .or_else(|| self.field_value_as_string(block, "colorParam"))
            .and_then(|raw| parse_pen_color_param(&raw))
            .unwrap_or(PenColorParam::Color)
    }

    fn parse_pen_color_param_payload(
        &mut self,
        target: &Target,
        payload: &Value,
    ) -> Option<PenColorParam> {
        match payload {
            Value::String(value) => {
                if target.blocks.contains_key(value) {
                    self.pen_color_param_from_block(target, value)
                } else {
                    parse_pen_color_param(value)
                }
            }
            Value::Array(values) => values
                .get(1)
                .or_else(|| values.first())
                .map(value_as_string)
                .and_then(|raw| parse_pen_color_param(&raw)),
            _ => parse_pen_color_param(&value_as_string(payload)),
        }
    }

    fn pen_color_param_from_block(&self, target: &Target, block_id: &str) -> Option<PenColorParam> {
        let block = target.blocks.get(block_id)?;
        if block.opcode != "pen_menu_colorParam" {
            return None;
        }
        self.field_value_as_string(block, "colorParam")
            .or_else(|| self.field_value_as_string(block, "COLOR_PARAM"))
            .and_then(|raw| parse_pen_color_param(&raw))
    }

    fn lower_clone_target(
        &mut self,
        target: &Target,
        block: &Block,
        block_id: &str,
    ) -> Option<CloneTarget> {
        if let Some(payload) = self.input_payload(block, "CLONE_OPTION") {
            if let Some(clone_target) = self.lower_clone_payload(target, payload) {
                return Some(clone_target);
            }
        }

        if let Some(option) = self.field_value_as_string(block, "CLONE_OPTION") {
            return self.resolve_clone_target_name(&option, block_id);
        }

        self.warn_once(format!("failed to resolve clone target in {}", block_id));
        None
    }

    fn lower_clone_payload(&mut self, target: &Target, payload: &Value) -> Option<CloneTarget> {
        match payload {
            Value::String(value) => {
                if target.blocks.contains_key(value) {
                    self.lower_clone_from_block(target, value)
                } else {
                    self.resolve_clone_target_name(value, value)
                }
            }
            Value::Array(values) => values
                .get(1)
                .or_else(|| values.first())
                .map(value_as_string)
                .and_then(|text| self.resolve_clone_target_name(&text, &text)),
            _ => {
                let text = value_as_string(payload);
                self.resolve_clone_target_name(&text, &text)
            }
        }
    }

    fn lower_clone_from_block(&mut self, target: &Target, block_id: &str) -> Option<CloneTarget> {
        let block = target.blocks.get(block_id)?;
        if block.opcode == "control_create_clone_of_menu" {
            if let Some(option) = self.field_value_as_string(block, "CLONE_OPTION") {
                return self.resolve_clone_target_name(&option, block_id);
            }
        }
        None
    }

    fn resolve_clone_target_name(&mut self, raw: &str, context: &str) -> Option<CloneTarget> {
        let normalized = raw.trim();
        if normalized.eq_ignore_ascii_case("_myself_") || normalized.eq_ignore_ascii_case("myself")
        {
            return Some(CloneTarget::Myself);
        }
        if let Some(target_index) = self.target_index_by_name.get(normalized).copied() {
            return Some(CloneTarget::Target(target_index));
        }

        self.warn_once(format!(
            "unknown clone target '{}' while lowering {}",
            normalized, context
        ));
        None
    }

    fn lower_say_input(&mut self, target: &Target, block: &Block, name: &str) -> SayExpr {
        let Some(payload) = self.input_payload(block, name) else {
            return SayExpr::Text(String::new());
        };
        self.lower_say_payload(target, payload)
    }

    fn lower_say_payload(&mut self, target: &Target, payload: &Value) -> SayExpr {
        match payload {
            Value::String(block_or_text) => {
                if target.blocks.contains_key(block_or_text) {
                    self.lower_say_from_block(target, block_or_text)
                } else {
                    SayExpr::Text(block_or_text.clone())
                }
            }
            Value::Array(values) => {
                if let Some(text) = primitive_text(values) {
                    SayExpr::Text(text)
                } else {
                    SayExpr::Numeric(self.lower_numeric_payload(target, payload))
                }
            }
            _ => SayExpr::Numeric(self.lower_numeric_payload(target, payload)),
        }
    }

    fn lower_say_from_block(&mut self, target: &Target, block_id: &str) -> SayExpr {
        let Some(block) = target.blocks.get(block_id) else {
            return SayExpr::Numeric(Expr::Number(0.0));
        };
        if block.opcode == "text" {
            SayExpr::Text(field_as_string(block, "TEXT"))
        } else {
            SayExpr::Numeric(self.lower_numeric_from_block(target, block_id))
        }
    }

    fn lower_numeric_input(&mut self, target: &Target, block: &Block, name: &str) -> Expr {
        let Some(payload) = self.input_payload(block, name) else {
            return Expr::Number(0.0);
        };
        self.lower_numeric_payload(target, payload)
    }

    fn lower_operator_length(&mut self, target: &Target, block: &Block) -> Expr {
        let Some(payload) = self.input_payload(block, "STRING") else {
            return Expr::Number(0.0);
        };

        if let Some(length) = self.try_constant_string_length(target, payload) {
            return Expr::Number(length as f64);
        }

        Expr::StringLength(Box::new(self.lower_numeric_payload(target, payload)))
    }

    fn try_constant_string_length(&mut self, target: &Target, payload: &Value) -> Option<usize> {
        match payload {
            Value::String(value) => {
                if target.blocks.contains_key(value) {
                    let block = target.blocks.get(value)?;
                    if block.opcode == "text" {
                        return Some(field_as_string(block, "TEXT").chars().count());
                    }
                    if block.opcode == "colour_picker" {
                        return Some(field_as_string(block, "COLOUR").chars().count());
                    }
                    None
                } else {
                    Some(value.chars().count())
                }
            }
            Value::Array(values) => {
                let primitive = values.first().and_then(Value::as_i64).unwrap_or_default();
                if primitive == 10 || primitive == 9 {
                    let text = values.get(1).map(value_as_string).unwrap_or_default();
                    return Some(text.chars().count());
                }
                None
            }
            _ => None,
        }
    }

    fn lower_numeric_payload(&mut self, target: &Target, payload: &Value) -> Expr {
        match payload {
            Value::String(block_or_number) => {
                if target.blocks.contains_key(block_or_number) {
                    self.lower_numeric_from_block(target, block_or_number)
                } else {
                    Expr::Number(cast_to_number(payload))
                }
            }
            Value::Array(values) => {
                if let Some(primitive) = self.lower_numeric_primitive(values) {
                    primitive
                } else {
                    Expr::Number(0.0)
                }
            }
            _ => Expr::Number(cast_to_number(payload)),
        }
    }

    fn lower_numeric_primitive(&mut self, values: &[Value]) -> Option<Expr> {
        let primitive_id = values.first()?.as_i64()?;
        let payload = values.get(1).unwrap_or(&Value::Null);
        match primitive_id {
            4 | 5 | 6 | 7 | 8 => Some(Expr::Number(cast_to_number(payload))),
            9 | 10 => Some(self.lower_scalar_literal(payload)),
            11 => Some(self.string_literal(&value_as_string(payload))),
            12 => {
                let variable_id = values.get(2).and_then(Value::as_str)?;
                let Some(index) = self.variable_index.get(variable_id).copied() else {
                    self.warn_once(format!("unknown variable id in primitive: {}", variable_id));
                    return Some(Expr::Number(0.0));
                };
                Some(Expr::Variable(index))
            }
            _ => Some(Expr::Number(cast_to_number(payload))),
        }
    }

    fn lower_numeric_from_block(&mut self, target: &Target, block_id: &str) -> Expr {
        let Some(block) = target.blocks.get(block_id) else {
            self.warn_once(format!("missing reporter block {}", block_id));
            return Expr::Number(0.0);
        };

        match block.opcode.as_str() {
            "math_number"
            | "math_positive_number"
            | "math_whole_number"
            | "math_integer"
            | "math_angle" => Expr::Number(field_as_number(block, "NUM")),
            "text" => self.lower_scalar_literal(&Value::String(field_as_string(block, "TEXT"))),
            "event_broadcast_menu" => self.string_literal(
                &self
                    .field_value_as_string(block, "BROADCAST_OPTION")
                    .unwrap_or_default(),
            ),
            "motion_xposition" => Expr::MotionXPosition,
            "motion_yposition" => Expr::MotionYPosition,
            "sensing_mousex" => Expr::SensingMouseX,
            "sensing_mousey" => Expr::SensingMouseY,
            "sensing_mousedown" => Expr::SensingMouseDown,
            "sensing_answer" => Expr::SensingAnswer,
            "sensing_timer" => Expr::SensingTimer,
            "sensing_dayssince2000" => Expr::SensingDaysSince2000,
            "sensing_keyoptions" => self.lower_scalar_literal(&Value::String(
                self.field_value_as_string(block, "KEY_OPTION")
                    .unwrap_or_default(),
            )),
            "sensing_touchingobjectmenu" => self.lower_scalar_literal(&Value::String(
                self.field_value_as_string(block, "TOUCHINGOBJECTMENU")
                    .unwrap_or_default(),
            )),
            "pen_menu_colorParam" => self.lower_scalar_literal(&Value::String(
                self.field_value_as_string(block, "colorParam")
                    .or_else(|| self.field_value_as_string(block, "COLOR_PARAM"))
                    .unwrap_or_default(),
            )),
            "sensing_of_object_menu" => self.lower_scalar_literal(&Value::String(
                self.field_value_as_string(block, "OBJECT")
                    .unwrap_or_default(),
            )),
            "looks_costumenumbername" => {
                let raw = self
                    .field_value_as_string(block, "NUMBER_NAME")
                    .unwrap_or_else(|| "number".to_string());
                if raw.trim().eq_ignore_ascii_case("name") {
                    Expr::LooksCostumeName
                } else {
                    Expr::LooksCostumeNumber
                }
            }
            "looks_costume" => self.lower_scalar_literal(&Value::String(
                self.field_value_as_string(block, "COSTUME")
                    .unwrap_or_default(),
            )),
            "sensing_of" => {
                let property_text = self
                    .field_value_as_string(block, "PROPERTY")
                    .unwrap_or_default();
                let property = self.intern_string(&property_text);
                Expr::SensingOf {
                    object: Box::new(self.lower_numeric_input(target, block, "OBJECT")),
                    property,
                }
            }
            "sensing_current" => {
                let raw = self
                    .field_value_as_string(block, "CURRENTMENU")
                    .unwrap_or_default();
                match parse_sensing_current_menu(&raw) {
                    Some(menu) => Expr::SensingCurrent(menu),
                    None => Expr::Number(0.0),
                }
            }
            "sensing_keypressed" => Expr::KeyPressed(Box::new(self.lower_numeric_input(
                target,
                block,
                "KEY_OPTION",
            ))),
            "sensing_touchingobject" => Expr::SensingTouchingObject(Box::new(
                self.lower_numeric_input(target, block, "TOUCHINGOBJECTMENU"),
            )),
            "data_variable" => {
                let Some(variable_id) = self.variable_id_from_field(block, "VARIABLE") else {
                    return Expr::Number(0.0);
                };
                let Some(variable_index) = self.variable_index.get(&variable_id).copied() else {
                    self.warn_once(format!(
                        "unknown variable id {} in reporter block {}",
                        variable_id, block_id
                    ));
                    return Expr::Number(0.0);
                };
                Expr::Variable(variable_index)
            }
            "argument_reporter_string_number" | "argument_reporter_boolean" => {
                let arg_name = self
                    .field_value_as_string(block, "VALUE")
                    .unwrap_or_else(|| "".to_string());
                let Some(arg_map) = self.current_proc_args.as_ref() else {
                    if arg_name.trim().eq_ignore_ascii_case("is compiled?") {
                        return Expr::Number(1.0);
                    }
                    self.warn_once(format!(
                        "argument reporter {} used outside procedure",
                        block_id
                    ));
                    return Expr::Number(0.0);
                };
                let Some(index) = arg_map.get(&arg_name).copied() else {
                    self.warn_once(format!(
                        "unknown procedure argument '{}' in {}",
                        arg_name, block_id
                    ));
                    return Expr::Number(0.0);
                };
                Expr::ProcedureArg(index)
            }
            "operator_add" => Expr::Add(
                Box::new(self.lower_numeric_input(target, block, "NUM1")),
                Box::new(self.lower_numeric_input(target, block, "NUM2")),
            ),
            "operator_subtract" => Expr::Subtract(
                Box::new(self.lower_numeric_input(target, block, "NUM1")),
                Box::new(self.lower_numeric_input(target, block, "NUM2")),
            ),
            "operator_multiply" => Expr::Multiply(
                Box::new(self.lower_numeric_input(target, block, "NUM1")),
                Box::new(self.lower_numeric_input(target, block, "NUM2")),
            ),
            "operator_divide" => Expr::Divide(
                Box::new(self.lower_numeric_input(target, block, "NUM1")),
                Box::new(self.lower_numeric_input(target, block, "NUM2")),
            ),
            "operator_mod" => Expr::Mod(
                Box::new(self.lower_numeric_input(target, block, "NUM1")),
                Box::new(self.lower_numeric_input(target, block, "NUM2")),
            ),
            "operator_gt" => Expr::GreaterThan(
                Box::new(self.lower_numeric_input(target, block, "OPERAND1")),
                Box::new(self.lower_numeric_input(target, block, "OPERAND2")),
            ),
            "operator_lt" => Expr::LessThan(
                Box::new(self.lower_numeric_input(target, block, "OPERAND1")),
                Box::new(self.lower_numeric_input(target, block, "OPERAND2")),
            ),
            "operator_equals" => Expr::Equals(
                Box::new(self.lower_numeric_input(target, block, "OPERAND1")),
                Box::new(self.lower_numeric_input(target, block, "OPERAND2")),
            ),
            "operator_length" => self.lower_operator_length(target, block),
            "operator_join" => Expr::StringJoin(
                Box::new(self.lower_numeric_input(target, block, "STRING1")),
                Box::new(self.lower_numeric_input(target, block, "STRING2")),
            ),
            "operator_contains" => Expr::StringContains(
                Box::new(self.lower_numeric_input(target, block, "STRING1")),
                Box::new(self.lower_numeric_input(target, block, "STRING2")),
            ),
            "operator_letter_of" => Expr::LetterOf {
                letter: Box::new(self.lower_numeric_input(target, block, "LETTER")),
                string: Box::new(self.lower_numeric_input(target, block, "STRING")),
            },
            "operator_round" => {
                Expr::Round(Box::new(self.lower_numeric_input(target, block, "NUM")))
            }
            "operator_mathop" => {
                let Some(op_text) = self.field_value_as_string(block, "OPERATOR") else {
                    return Expr::Number(0.0);
                };
                let Some(op) = parse_mathop(&op_text) else {
                    self.warn_once(format!(
                        "unsupported mathop variant '{}' in {}",
                        op_text, block_id
                    ));
                    return Expr::Number(0.0);
                };
                Expr::MathOp {
                    op,
                    value: Box::new(self.lower_numeric_input(target, block, "NUM")),
                }
            }
            "operator_random" => Expr::Random(
                Box::new(self.lower_numeric_input(target, block, "FROM")),
                Box::new(self.lower_numeric_input(target, block, "TO")),
            ),
            "operator_and" => Expr::And(
                Box::new(self.lower_numeric_input(target, block, "OPERAND1")),
                Box::new(self.lower_numeric_input(target, block, "OPERAND2")),
            ),
            "operator_or" => Expr::Or(
                Box::new(self.lower_numeric_input(target, block, "OPERAND1")),
                Box::new(self.lower_numeric_input(target, block, "OPERAND2")),
            ),
            "operator_not" => {
                Expr::Not(Box::new(self.lower_numeric_input(target, block, "OPERAND")))
            }
            "procedures_call" => {
                let Some((procedure_index, args)) =
                    self.lower_procedure_call_parts(target, block, block_id)
                else {
                    return Expr::Number(0.0);
                };
                Expr::ProcedureCall {
                    procedure_index,
                    args,
                }
            }
            "data_itemoflist" => {
                let Some(list_id) = self.list_id_from_field(block, "LIST") else {
                    return Expr::Number(0.0);
                };
                let Some(list_index) = self.list_index.get(&list_id).copied() else {
                    self.warn_once(format!(
                        "unknown list id {} in reporter block {}",
                        list_id, block_id
                    ));
                    return Expr::Number(0.0);
                };
                Expr::ListItem {
                    list_index,
                    index: Box::new(self.lower_numeric_input(target, block, "INDEX")),
                }
            }
            "data_itemnumoflist" => {
                let Some(list_id) = self.list_id_from_field(block, "LIST") else {
                    return Expr::Number(0.0);
                };
                let Some(list_index) = self.list_index.get(&list_id).copied() else {
                    self.warn_once(format!(
                        "unknown list id {} in reporter block {}",
                        list_id, block_id
                    ));
                    return Expr::Number(0.0);
                };
                Expr::ListItemNum {
                    list_index,
                    item: Box::new(self.lower_numeric_input(target, block, "ITEM")),
                }
            }
            "data_lengthoflist" => {
                let Some(list_id) = self.list_id_from_field(block, "LIST") else {
                    return Expr::Number(0.0);
                };
                let Some(list_index) = self.list_index.get(&list_id).copied() else {
                    self.warn_once(format!(
                        "unknown list id {} in reporter block {}",
                        list_id, block_id
                    ));
                    return Expr::Number(0.0);
                };
                Expr::ListLength { list_index }
            }
            "data_listcontainsitem" => {
                let Some(list_id) = self.list_id_from_field(block, "LIST") else {
                    return Expr::Number(0.0);
                };
                let Some(list_index) = self.list_index.get(&list_id).copied() else {
                    self.warn_once(format!(
                        "unknown list id {} in reporter block {}",
                        list_id, block_id
                    ));
                    return Expr::Number(0.0);
                };
                Expr::ListContainsItem {
                    list_index,
                    item: Box::new(self.lower_numeric_input(target, block, "ITEM")),
                }
            }
            unsupported => {
                self.warn_once(format!("unsupported reporter opcode: {}", unsupported));
                Expr::Number(0.0)
            }
        }
    }

    fn lower_scalar_literal(&mut self, value: &Value) -> Expr {
        match value {
            Value::String(text) => {
                if text.trim().parse::<f64>().is_ok() {
                    Expr::Number(cast_to_number(value))
                } else {
                    self.string_literal(text)
                }
            }
            Value::Number(_) | Value::Bool(_) | Value::Null => Expr::Number(cast_to_number(value)),
            Value::Array(array) => self.lower_scalar_literal(array.first().unwrap_or(&Value::Null)),
            Value::Object(object) => {
                if let Some(inner) = object.get("value") {
                    self.lower_scalar_literal(inner)
                } else {
                    Expr::Number(0.0)
                }
            }
        }
    }

    fn string_literal(&mut self, text: &str) -> Expr {
        Expr::StringLiteral(self.intern_string(text))
    }

    fn variable_id_from_field(&mut self, block: &Block, field_name: &str) -> Option<String> {
        let field = block.fields.get(field_name)?;
        let array = field.as_array()?;
        let id = array.get(1).and_then(Value::as_str)?;
        Some(id.to_string())
    }

    fn list_id_from_field(&mut self, block: &Block, field_name: &str) -> Option<String> {
        let field = block.fields.get(field_name)?;
        let array = field.as_array()?;
        let id = array.get(1).and_then(Value::as_str)?;
        Some(id.to_string())
    }

    fn field_value_as_string(&self, block: &Block, field_name: &str) -> Option<String> {
        let field = block.fields.get(field_name)?;
        if let Some(array) = field.as_array() {
            return array.first().map(value_as_string);
        }
        if let Some(object) = field.as_object() {
            return object.get("value").map(value_as_string);
        }
        Some(value_as_string(field))
    }

    fn input_payload<'b>(&self, block: &'b Block, name: &str) -> Option<&'b Value> {
        let raw = block.inputs.get(name)?;
        if let Some(array) = raw.as_array() {
            return array.get(1).or_else(|| array.first());
        }
        Some(raw)
    }

    fn warn_once(&mut self, warning: String) {
        if self.seen_warnings.insert(warning.clone()) {
            self.warnings.push(warning);
        }
    }

    fn parse_list(&mut self, raw: &Value) -> (String, Vec<ScalarValue>) {
        let Some(array) = raw.as_array() else {
            return ("list".to_string(), Vec::new());
        };
        let name = array
            .first()
            .and_then(Value::as_str)
            .unwrap_or("list")
            .to_string();
        let initial_values = array
            .get(1)
            .and_then(Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .map(|item| match item {
                        Value::String(text) => {
                            if text.trim().parse::<f64>().is_ok() {
                                ScalarValue::Number(cast_to_number(item))
                            } else {
                                ScalarValue::String(self.intern_string(text))
                            }
                        }
                        _ => ScalarValue::Number(cast_to_number(item)),
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        (name, initial_values)
    }

    fn intern_string(&mut self, text: &str) -> usize {
        if let Some(index) = self.string_index.get(text).copied() {
            return index;
        }
        let index = self.strings.len();
        self.strings.push(text.to_string());
        self.string_index.insert(text.to_string(), index);
        index
    }
}

fn procedure_key(target: &Target, proccode: &str) -> String {
    format!("{}::{}", target.name, proccode)
}

fn procedure_fuzzy_key(target: &Target, proccode: &str) -> String {
    format!("{}::{}", target.name, normalize_proccode(proccode))
}

fn normalize_proccode(raw: &str) -> String {
    let mut out = String::new();
    let mut prev_space = false;
    let mut prev_slash = false;

    for ch in raw.chars() {
        if matches!(
            ch,
            '\u{200b}' | '\u{200c}' | '\u{200d}' | '\u{2060}' | '\u{feff}'
        ) {
            continue;
        }

        if ch.is_whitespace() {
            if !prev_space {
                out.push(' ');
            }
            prev_space = true;
            prev_slash = false;
            continue;
        }

        if ch == '/' {
            if !prev_slash {
                out.push('/');
            }
            prev_slash = true;
            prev_space = false;
            continue;
        }

        out.push(ch);
        prev_space = false;
        prev_slash = false;
    }

    out.trim().to_string()
}

fn mutation_string(mutation: &Value, key: &str) -> Option<String> {
    if let Some(object) = mutation.as_object() {
        return object.get(key).map(value_as_string);
    }
    None
}

fn mutation_bool(mutation: &Value, key: &str) -> Option<bool> {
    let object = mutation.as_object()?;
    let raw = object.get(key)?;
    match raw {
        Value::Bool(value) => Some(*value),
        Value::String(text) => match text.trim().to_ascii_lowercase().as_str() {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        },
        Value::Number(value) => value.as_f64().map(|number| number != 0.0),
        _ => None,
    }
}

fn mutation_string_array(mutation: &Value, key: &str) -> Option<Vec<String>> {
    let text = mutation_string(mutation, key)?;
    parse_json_string_array(&text)
}

fn parse_json_string_array(text: &str) -> Option<Vec<String>> {
    let parsed = serde_json::from_str::<Value>(text).ok()?;
    let array = parsed.as_array()?;
    Some(array.iter().map(value_as_string).collect::<Vec<_>>())
}

fn parse_mathop(raw: &str) -> Option<MathOp> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "abs" => Some(MathOp::Abs),
        "floor" => Some(MathOp::Floor),
        "ceiling" => Some(MathOp::Ceil),
        "sqrt" => Some(MathOp::Sqrt),
        "sin" => Some(MathOp::Sin),
        "cos" => Some(MathOp::Cos),
        "tan" => Some(MathOp::Tan),
        "asin" => Some(MathOp::Asin),
        "acos" => Some(MathOp::Acos),
        "atan" => Some(MathOp::Atan),
        "ln" => Some(MathOp::Ln),
        "log" => Some(MathOp::Log),
        "e ^" => Some(MathOp::Exp),
        "10 ^" => Some(MathOp::Exp10),
        _ => None,
    }
}

fn parse_control_stop_mode(raw: &str) -> ControlStopMode {
    match raw.trim().to_ascii_lowercase().as_str() {
        "all" => ControlStopMode::All,
        "other scripts in sprite" | "other scripts in stage" => {
            ControlStopMode::OtherScriptsInTarget
        }
        _ => ControlStopMode::ThisScript,
    }
}

fn parse_pen_color_param(raw: &str) -> Option<PenColorParam> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "color" => Some(PenColorParam::Color),
        "saturation" => Some(PenColorParam::Saturation),
        "brightness" => Some(PenColorParam::Brightness),
        "transparency" => Some(PenColorParam::Transparency),
        _ => None,
    }
}

fn parse_sensing_current_menu(raw: &str) -> Option<SensingCurrentMenu> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "year" => Some(SensingCurrentMenu::Year),
        "month" => Some(SensingCurrentMenu::Month),
        "date" => Some(SensingCurrentMenu::Date),
        "dayofweek" => Some(SensingCurrentMenu::DayOfWeek),
        "hour" => Some(SensingCurrentMenu::Hour),
        "minute" => Some(SensingCurrentMenu::Minute),
        "second" => Some(SensingCurrentMenu::Second),
        _ => None,
    }
}

fn normalize_key_name(raw: &str) -> String {
    let lowered = raw.trim().to_ascii_lowercase();
    match lowered.as_str() {
        "left" | "left arrow" => "left arrow".to_string(),
        "right" | "right arrow" => "right arrow".to_string(),
        "up" | "up arrow" => "up arrow".to_string(),
        "down" | "down arrow" => "down arrow".to_string(),
        "space" | "spacebar" => "space".to_string(),
        _ => lowered,
    }
}

fn sanitize_symbol(input: &str) -> String {
    let sanitized: String = input
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    if sanitized.is_empty() {
        "target".to_string()
    } else {
        sanitized
    }
}

fn parse_variable(raw: &Value) -> (String, f64) {
    if let Some(array) = raw.as_array() {
        let name = array
            .first()
            .and_then(Value::as_str)
            .unwrap_or("variable")
            .to_string();
        let initial = array.get(1).map(cast_to_number).unwrap_or(0.0);
        return (name, initial);
    }
    ("variable".to_string(), 0.0)
}

fn field_as_string(block: &Block, field_name: &str) -> String {
    block
        .fields
        .get(field_name)
        .map_or_else(String::new, |field| {
            if let Some(array) = field.as_array() {
                return array
                    .first()
                    .map(value_as_string)
                    .unwrap_or_else(String::new);
            }
            if let Some(object) = field.as_object() {
                if let Some(value) = object.get("value") {
                    return value_as_string(value);
                }
            }
            value_as_string(field)
        })
}

fn field_as_number(block: &Block, field_name: &str) -> f64 {
    block
        .fields
        .get(field_name)
        .map_or(0.0, |field| cast_to_number(field_value(field)))
}

fn field_value(field: &Value) -> &Value {
    if let Some(array) = field.as_array() {
        return array.first().unwrap_or(&Value::Null);
    }
    if let Some(object) = field.as_object() {
        if let Some(value) = object.get("value") {
            return value;
        }
    }
    field
}

fn primitive_text(values: &[Value]) -> Option<String> {
    let primitive_id = values.first()?.as_i64()?;
    if primitive_id != 10 {
        return None;
    }
    Some(
        values
            .get(1)
            .map(value_as_string)
            .unwrap_or_else(String::new),
    )
}

fn value_as_string(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Number(number) => number.to_string(),
        Value::Bool(flag) => {
            if *flag {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
        _ => String::new(),
    }
}

fn cast_to_number(value: &Value) -> f64 {
    match value {
        Value::Number(number) => number.as_f64().unwrap_or(0.0),
        Value::Bool(flag) => {
            if *flag {
                1.0
            } else {
                0.0
            }
        }
        Value::String(text) => text.trim().parse::<f64>().unwrap_or(0.0),
        Value::Array(array) => cast_to_number(array.first().unwrap_or(&Value::Null)),
        Value::Object(object) => {
            if let Some(inner) = object.get("value") {
                cast_to_number(inner)
            } else {
                0.0
            }
        }
        Value::Null => 0.0,
    }
}
