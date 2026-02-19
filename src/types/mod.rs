mod inputtype;
mod primitive;
mod shadow;
use std::collections::{HashMap, HashSet};

use serde::de::Error as _;
use serde_repr::Deserialize_repr;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::types::{
    primitive::{InputPrimitive, TopLevelPrimitive},
    shadow::Shadow,
};

#[derive(Debug)]
pub enum StageOrSprite {
    Stage(Stage),
    Sprite(Sprite),
}
impl<'de> Deserialize<'de> for StageOrSprite {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // いったん Value で受けて isStage を覗く
        let v = Value::deserialize(deserializer)?;

        let is_stage = v
            .get("isStage")
            .and_then(|x| x.as_bool())
            .ok_or_else(|| D::Error::custom("missing or non-bool field: isStage"))?;

        if is_stage {
            let stage: Stage =
                serde_json::from_value(v).map_err(|e| D::Error::custom(e.to_string()))?;
            Ok(StageOrSprite::Stage(stage))
        } else {
            let sprite: Sprite =
                serde_json::from_value(v).map_err(|e| D::Error::custom(e.to_string()))?;
            Ok(StageOrSprite::Sprite(sprite))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ScratchProject {
    pub meta: Meta,
    pub targets: Vec<StageOrSprite>,
}
impl ScratchProject {
    pub fn count_blocks(&self) -> usize {
        let mut sum = 0;
        for i in &self.targets {
            sum += match i {
                StageOrSprite::Stage(v) => v.blocks.len(),
                StageOrSprite::Sprite(v) => v.blocks.len(),
            };
        }
        sum
    }
    pub fn check_op_codes(&self) -> Vec<String> {
        let mut op_codes_set: HashSet<String> = HashSet::new();
        for i in &self.targets {
            match i {
                StageOrSprite::Stage(v) => {
                    for j in &v.blocks {
                        if let BlockAndTopLevelPrimitive::Block(t) = j.1 {
                            op_codes_set.insert(t.opcode.clone());
                        }
                    }
                }
                StageOrSprite::Sprite(v) => {
                    for j in &v.blocks {
                        if let BlockAndTopLevelPrimitive::Block(t) = j.1 {
                            op_codes_set.insert(t.opcode.clone());
                        }
                    }
                }
            };
        }
        op_codes_set.into_iter().collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub semver: String,
    pub vm: Option<String>,
    pub agent: Option<String>,
    pub origin: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum VideoState {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "on-flipped")]
    OnFlipped,
}
#[derive(Debug, Deserialize)]
pub enum RotationStyle {
    #[serde(rename = "all around")]
    AllAround,
    #[serde(rename = "don't rotate")]
    DontRotate,
    #[serde(rename = "left-right")]
    LeftRight,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct Stage {
    #[serde(flatten)]
    base: Target,
    name: String,
    isStage: bool,
    tempo: Option<f64>,
    videoTransparency: Option<f64>,
    videoState: Option<VideoState>,
    layerOrder: Option<u32>,
}
impl Deref for Stage {
    type Target = Target;
    fn deref(&self) -> &Target {
        &self.base
    }
}
impl DerefMut for Stage {
    fn deref_mut(&mut self) -> &mut Target {
        &mut self.base
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct Sprite {
    #[serde(flatten)]
    base: Target,
    name: String,
    isStage: bool,
    visible: Option<bool>,
    x: Option<f64>,
    y: Option<f64>,
    size: Option<f64>,
    direction: Option<f64>,
    draggable: Option<bool>,
    rotationStyle: Option<RotationStyle>,
    layerOrder: Option<u32>,
}
impl Deref for Sprite {
    type Target = Target;
    fn deref(&self) -> &Target {
        &self.base
    }
}
impl DerefMut for Sprite {
    fn deref_mut(&mut self) -> &mut Target {
        &mut self.base
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct Target {
    pub currentCostume: u32,
    pub blocks: HashMap<String, BlockAndTopLevelPrimitive>,
    pub variables: HashMap<String, ScalarVariable>,
    pub lists: HashMap<String, List>,
    pub broadcasts: HashMap<String, String>,
    pub comments: Option<HashMap<String, Comment>>,
    pub costumes: Vec<Costume>,
    pub sounds: Vec<Sound>,
    pub volume: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum BlockAndTopLevelPrimitive {
    Block(Block),
    TopLevelPrimitive(TopLevelPrimitive),
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct Block {
    pub opcode: String,
    pub comment: Option<String>,
    pub inputs: Option<HashMap<String, Input>>,
    pub fields: Option<HashMap<String, Fields>>,
    pub next: Option<String>,
    pub parent: Option<String>,
    pub topLevel: Option<bool>,
    pub shadow: Option<bool>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub mutation: Option<Mutation>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Mutation {
    MutationProceduresCall(MutationProceduresCall),
    MutationProceduresPrototype(MutationProceduresPrototype),
    MutationControlStop(MutationControlStop),
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct MutationProceduresCall {
    pub tagName: Option<String>,
    pub proccode: Option<String>,
    pub argumentids: StringOrStringArray,
    pub warp: Option<WarpValue>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct MutationProceduresPrototype {
    pub tagName: Option<String>,
    pub argumentdefaults: Vec<StringOrBool>,
}
#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct MutationControlStop {
    pub tagName: Option<String>,
    pub hasnext: Option<HasNext>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct Comment {
    blockId: Option<String>,
    text: String,
    minimized: Option<bool>,
    x: Option<f64>,
    y: Option<f64>,
    width: Option<f64>,
    height: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct Costume {
    assetId: String,
    bitmapResolution: Option<f64>,
    dataFormat: ImageFormat,
    md5ext: Option<String>,
    name: String,
    rotationCenterX: Option<f64>,
    rotationCenterY: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum ImageFormat {
    #[serde(rename = "png")]
    png,
    #[serde(rename = "svg")]
    svg,
    #[serde(rename = "jpeg")]
    jpeg,
    #[serde(rename = "jpg")]
    jpg,
    #[serde(rename = "bmp")]
    bmp,
    #[serde(rename = "gif")]
    gif,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum WarpValue {
    String(String),
    Bool(bool),
    Null,
}
pub type HasNext = WarpValue;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum StringOrStringArray {
    String(String),
    StringArray(Vec<String>),
}
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum StringOrBool {
    String(String),
    Bool(bool),
}
#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum AudioFormat {
    #[serde(rename = "wav")]
    wav,
    #[serde(rename = "wave")]
    wave,
    #[serde(rename = "mp3")]
    mp3,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct Sound {
    assetId: String,
    dataFormat: AudioFormat,
    md5ext: Option<String>,
    name: String,
    rate: Option<f64>,
    sampleCount: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ScalarVal {
    String(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ScalarVariable {
    V2((String, ScalarVal)),
    V3((String, ScalarVal, bool)),
}
impl ScalarVariable {
    pub fn display_name(&self) -> String {
        match self {
            ScalarVariable::V2(t) => t.0.clone(),
            ScalarVariable::V3(t) => t.0.clone(),
        }
    }
    pub fn default_value(&self) -> ScalarVal {
        match self {
            ScalarVariable::V2(t) => t.1.clone(),
            ScalarVariable::V3(t) => t.1.clone(),
        }
    }
    pub fn is_cloud_variable(&self) -> Option<bool> {
        match self {
            ScalarVariable::V2(_) => None,
            ScalarVariable::V3(t) => Some(t.2),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct List(String, Vec<ScalarVal>);
impl List {
    pub fn display_name(&self) -> String {
        self.0.clone()
    }
    pub fn default_value(&self) -> &Vec<ScalarVal> {
        return &self.1;
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum InputPrimitiveOrReference {
    InputPrimitive(InputPrimitive),
    String(String),
    Null,
}

#[repr(u8)]
#[derive(Debug, Deserialize_repr)]
pub enum SameBlockShadowOrNoShadow {
    SameBlockShadow = Shadow::SameBlockShadow as u8,
    NoShadow = Shadow::NoShadow as u8,
}
impl SameBlockShadowOrNoShadow {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}
impl TryFrom<u8> for SameBlockShadowOrNoShadow {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            1 => Self::SameBlockShadow,
            2 => Self::NoShadow,
            _ => return Err(()),
        })
    }
}

#[repr(u8)]
#[derive(Debug, Deserialize_repr)]
pub enum DiffBlockShadow {
    DiffBlockShadow = Shadow::DiffBlockShadow as u8,
}
impl DiffBlockShadow {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}
impl TryFrom<u8> for DiffBlockShadow {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            3 => DiffBlockShadow::DiffBlockShadow,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Input {
    V2((SameBlockShadowOrNoShadow, InputPrimitiveOrReference)),
    V3(
        (
            DiffBlockShadow,
            InputPrimitiveOrReference,
            InputPrimitiveOrReference,
        ),
    ),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Fields {
    V1((String,)),
    V2((String, Option<String>)),
}
