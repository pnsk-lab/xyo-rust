//! Scratch project (`.sb3` file) loading and parsing.
//!
//! This module provides functionality to load and deserialize Scratch 3.0
//! project files, which are ZIP archives containing:
//! - `project.json` - The project structure and blocks
//! - Asset files (images, sounds) referenced by MD5 hashes
//!
//! # Example
//!
//! ```rust,no_run
//! use scratch_native_runtime::project::sb3;
//!
//! let project = sb3::load_project_from_sb3("my_project.sb3")?;
//! println!("Project has {} targets", project.targets.len());
//! # Ok::<(), anyhow::Error>(())
//! ```

use anyhow::{Context, Result};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Cursor, Read, Seek};
use std::path::Path;
use zip::ZipArchive;

/// A complete Scratch project with all targets and assets.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    /// All targets (stage + sprites).
    #[serde(default)]
    pub targets: Vec<Target>,
    /// Asset files keyed by MD5 hash (e.g., "abc123.png").
    #[serde(skip)]
    pub assets: HashMap<String, Vec<u8>>,
}

/// A target (sprite or stage) in a Scratch project.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    /// Target name (e.g., "Stage", "Sprite1").
    pub name: String,
    /// Whether this target is the stage (backdrop).
    #[serde(default)]
    pub is_stage: bool,
    /// Blocks keyed by block ID.
    #[serde(default)]
    pub blocks: HashMap<String, Block>,
    /// Variables keyed by variable ID.
    #[serde(default)]
    pub variables: HashMap<String, Value>,
    /// Lists keyed by list ID.
    #[serde(default)]
    pub lists: HashMap<String, Value>,
    /// Currently selected costume index.
    #[serde(default)]
    pub current_costume: usize,
    /// All costumes for this target.
    #[serde(default)]
    pub costumes: Vec<Costume>,
    /// X position (-240 to 240).
    #[serde(default)]
    pub x: f64,
    /// Y position (-180 to 180).
    #[serde(default)]
    pub y: f64,
    /// Direction in degrees (0 = up, 90 = right).
    #[serde(default = "default_direction")]
    pub direction: f64,
    /// Whether the sprite is visible.
    #[serde(default = "default_visible")]
    pub visible: bool,
    /// Size as a percentage (100 = normal).
    #[serde(default = "default_size")]
    pub size: f64,
    /// Rendering layer order (higher = in front).
    #[serde(default)]
    pub layer_order: i64,
}

/// A costume (image) for a sprite or backdrop for the stage.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Costume {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub data_format: String,
    #[serde(default)]
    pub md5ext: String,
    #[serde(default)]
    pub rotation_center_x: f64,
    #[serde(default)]
    pub rotation_center_y: f64,
}

#[derive(Debug)]
pub struct Block {
    pub opcode: String,
    pub next: Option<String>,
    pub top_level: bool,
    pub inputs: HashMap<String, Value>,
    pub fields: HashMap<String, Value>,
    pub mutation: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BlockObject {
    opcode: String,
    #[serde(default)]
    next: Option<Value>,
    #[serde(default)]
    top_level: bool,
    #[serde(default)]
    inputs: HashMap<String, Value>,
    #[serde(default)]
    fields: HashMap<String, Value>,
    #[serde(default)]
    mutation: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum BlockWire {
    Object(BlockObject),
    Primitive(Vec<Value>),
}

impl<'de> Deserialize<'de> for Block {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = BlockWire::deserialize(deserializer)?;
        Ok(match wire {
            BlockWire::Object(object) => Block {
                opcode: object.opcode,
                next: coerce_optional_string(object.next),
                top_level: object.top_level,
                inputs: object.inputs,
                fields: object.fields,
                mutation: object.mutation,
            },
            BlockWire::Primitive(values) => block_from_primitive(values),
        })
    }
}

fn coerce_optional_string(value: Option<Value>) -> Option<String> {
    value.and_then(|value| match value {
        Value::String(text) => Some(text),
        Value::Number(number) => Some(number.to_string()),
        Value::Bool(flag) => Some(if flag {
            "true".to_string()
        } else {
            "false".to_string()
        }),
        _ => None,
    })
}

fn block_from_primitive(values: Vec<Value>) -> Block {
    let primitive_id = values.first().and_then(Value::as_i64).unwrap_or_default();
    let payload = values.get(1).cloned().unwrap_or(Value::Null);

    match primitive_id {
        4 | 5 | 6 | 7 | 8 => Block {
            opcode: match primitive_id {
                4 => "math_number",
                5 => "math_positive_number",
                6 => "math_whole_number",
                7 => "math_integer",
                _ => "math_angle",
            }
            .to_string(),
            next: None,
            top_level: false,
            inputs: HashMap::new(),
            fields: HashMap::from([("NUM".to_string(), payload)]),
            mutation: None,
        },
        9 | 10 => Block {
            opcode: "text".to_string(),
            next: None,
            top_level: false,
            inputs: HashMap::new(),
            fields: HashMap::from([("TEXT".to_string(), payload)]),
            mutation: None,
        },
        11 => Block {
            opcode: "event_broadcast_menu".to_string(),
            next: None,
            top_level: false,
            inputs: HashMap::new(),
            fields: HashMap::from([("BROADCAST_OPTION".to_string(), payload)]),
            mutation: None,
        },
        12 => {
            let variable_name = payload;
            let variable_id = values.get(2).cloned().unwrap_or(Value::Null);
            Block {
                opcode: "data_variable".to_string(),
                next: None,
                top_level: false,
                inputs: HashMap::new(),
                fields: HashMap::from([(
                    "VARIABLE".to_string(),
                    Value::Array(vec![variable_name, variable_id]),
                )]),
                mutation: None,
            }
        }
        _ => Block {
            opcode: "math_number".to_string(),
            next: None,
            top_level: false,
            inputs: HashMap::new(),
            fields: HashMap::from([("NUM".to_string(), payload)]),
            mutation: None,
        },
    }
}

fn default_direction() -> f64 {
    90.0
}

fn default_visible() -> bool {
    true
}

fn default_size() -> f64 {
    100.0
}

fn find_archive_index_by_name<R: Read + Seek>(
    archive: &mut ZipArchive<R>,
    name: &str,
) -> Option<usize> {
    for index in 0..archive.len() {
        let entry_name = match archive.by_index(index) {
            Ok(entry) => entry.name().to_owned(),
            Err(_) => continue,
        };
        if entry_name == name || entry_name.ends_with(name) {
            return Some(index);
        }
    }
    None
}

fn load_project_from_archive<R: Read + Seek>(
    archive: &mut ZipArchive<R>,
    source_name: &str,
) -> Result<Project> {
    let mut project_index = None;
    for index in 0..archive.len() as usize {
        let name = {
            let entry = archive
                .by_index(index)
                .context("failed to inspect sb3 zip entry")?;
            entry.name().to_owned()
        };
        if name.ends_with("project.json") {
            project_index = Some(index);
            break;
        }
    }

    let index = project_index.context("project.json not found in sb3 archive")?;
    let mut project_file = archive
        .by_index(index)
        .context("failed to open project.json from sb3")?;
    let mut project_json = String::new();
    project_file
        .read_to_string(&mut project_json)
        .context("failed to read project.json as UTF-8 text")?;
    drop(project_file);

    let mut project: Project =
        serde_json::from_str(&project_json).context("failed to parse project.json")?;
    project.assets = HashMap::new();

    let mut required_assets = HashSet::new();
    for target in &project.targets {
        for costume in &target.costumes {
            if !costume.md5ext.is_empty() {
                required_assets.insert(costume.md5ext.clone());
            }
        }
    }

    for asset_name in required_assets {
        let entry_index = find_archive_index_by_name(archive, &asset_name);
        let Some(entry_index) = entry_index else {
            continue;
        };

        let mut entry = archive
            .by_index(entry_index)
            .with_context(|| format!("failed to open asset {}", asset_name))?;
        let mut bytes = Vec::new();
        entry.read_to_end(&mut bytes).with_context(|| {
            format!(
                "failed to read asset bytes for {} from {}",
                asset_name, source_name
            )
        })?;
        project.assets.insert(asset_name, bytes);
    }

    Ok(project)
}

pub fn load_project_from_sb3(path: &Path) -> Result<Project> {
    let file = File::open(path).with_context(|| format!("failed to open {}", path.display()))?;
    let mut archive = ZipArchive::new(file).context("failed to read sb3 zip archive")?;
    load_project_from_archive(&mut archive, &path.display().to_string())
}

pub fn load_project_from_sb3_bytes(bytes: &[u8], source_name: &str) -> Result<Project> {
    let cursor = Cursor::new(bytes);
    let mut archive = ZipArchive::new(cursor).context("failed to read sb3 zip archive")?;
    load_project_from_archive(&mut archive, source_name)
}
