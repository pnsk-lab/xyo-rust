#![allow(unused)]

pub mod inputtype;
pub mod primitive;
pub mod shadow;
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

// Str => enum, enum => Strを実現できます
#[macro_export]
macro_rules! str_enum_with_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $Name:ident : $Kind:ty {
            $(
                $Group:ident {
                    $(
                        $Var:ident => $lit:literal
                    ),* $(,)?
                }
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $vis enum $Name {
            $(
                $($Var),*,
            )*
        }

        impl $Name {
            $vis fn as_str(self) -> &'static str {
                match self {
                    $($(
                        Self::$Var => $lit,
                    )*)*
                }
            }

            $vis const fn kind(self) -> $Kind {
                match self {
                    $($(
                        Self::$Var => <$Kind>::$Group,
                    )*)*
                }
            }
        }

        impl ::core::fmt::Display for $Name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str((*self).as_str())
            }
        }

        impl ::core::str::FromStr for $Name {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($(
                        $lit => Ok(Self::$Var),
                    )*)*
                    _ => Err(format!("unknown {}: {}", stringify!($Name), s)),
                }
            }
        }

        impl ::serde::Serialize for $Name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $Name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                struct V;

                impl<'de> ::serde::de::Visitor<'de> for V {
                    type Value = $Name;

                    fn expecting(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        write!(f, "a string for {}", stringify!($Name))
                    }

                    fn visit_str<E>(self, v: &str) -> Result<$Name, E>
                    where
                        E: ::serde::de::Error,
                    {
                        ::core::str::FromStr::from_str(v)
                            .map_err(|e| E::custom(e))
                    }

                    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<$Name, E>
                    where
                        E: ::serde::de::Error,
                    {
                        self.visit_str(v)
                    }

                    fn visit_string<E>(self, v: String) -> Result<$Name, E>
                    where
                        E: ::serde::de::Error,
                    {
                        self.visit_str(&v)
                    }
                }

                deserializer.deserialize_str(V)
            }
        }
    };
}
// Str => enum, enum => Strを実現できます
#[macro_export]
macro_rules! str_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $Name:ident {
            $(
                $Var:ident => $lit:literal
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $vis enum $Name { $($Var),* }

        impl $Name {
           #[allow(dead_code)]
            $vis fn as_str(self) -> &'static str {
                match self {
                    $(Self::$Var => $lit),*
                }
            }
        }

        impl ::core::fmt::Display for $Name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str((*self).as_str())
            }
        }

        impl ::core::str::FromStr for $Name {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($lit => Ok(Self::$Var),)*
                    _ => Err(format!("unknown {}: {}", stringify!($Name), s)),
                }
            }
        }

        impl ::serde::Serialize for $Name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $Name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                struct V;

                impl<'de> ::serde::de::Visitor<'de> for V {
                    type Value = $Name;

                    fn expecting(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        write!(f, "a string for {}", stringify!($Name))
                    }

                    fn visit_str<E>(self, v: &str) -> Result<$Name, E>
                    where
                        E: ::serde::de::Error,
                    {
                        match v {
                            $($lit => Ok($Name::$Var),)*
                            _ => Err(E::custom(format!("unknown {}: {}", stringify!($Name), v))),
                        }
                    }

                    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<$Name, E>
                    where
                        E: ::serde::de::Error,
                    {
                        self.visit_str(v)
                    }

                    fn visit_string<E>(self, v: String) -> Result<$Name, E>
                    where
                        E: ::serde::de::Error,
                    {
                        self.visit_str(&v)
                    }
                }

                deserializer.deserialize_str(V)
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockKind {
    Motion,
    Looks,
    Sound,
    Event,
    Control,
    Sensing,
    Operator,
    Data,
    Procedures,
    Pen,
}
str_enum_with_enum! {
    pub enum BlockOpCodes: BlockKind {
        Motion {
            MotionMoveSteps => "motion_movesteps",
            MotionGoToXY => "motion_gotoxy",
            MotionGoTo => "motion_goto",
            MotionGoToMenu => "motion_goto_menu",
            MotionTurnRight => "motion_turnright",
            MotionTurnLeft => "motion_turnleft",
            MotionPointInDirection => "motion_pointindirection",
            MotionPointTowards => "motion_pointtowards",
            MotionPointTowardsMenu => "motion_pointtowards_menu",
            MotionGlideSecsToXY => "motion_glidesecstoxy",
            MotionGlideTo => "motion_glideto",
            MotionGlideToMenu => "motion_glideto_menu",
            MotionIfOnEdgeBounce => "motion_ifonedgebounce",
            MotionSetRotationStyle => "motion_setrotationstyle",
            MotionChangeXBy => "motion_changexby",
            MotionSetX => "motion_setx",
            MotionChangeYBy => "motion_changeyby",
            MotionSetY => "motion_sety",
            MotionXPosition => "motion_xposition",
            MotionYPosition => "motion_yposition",
            MotionDirection => "motion_direction",
            MotionScrollRight => "motion_scroll_right",
            MotionScrollUp => "motion_scroll_up",
            MotionAlignScene => "motion_align_scene",
            MotionXScroll => "motion_xscroll",
            MotionYScroll => "motion_yscroll"
        },
        Looks {
            LooksSay => "looks_say",
            LooksSayForSecs => "looks_sayforsecs",
            LooksThink => "looks_think",
            LooksThinkForSecs => "looks_thinkforsecs",
            LooksShow => "looks_show",
            LooksHide => "looks_hide",
            LooksHideAllSprites => "looks_hideallsprites",
            LooksSwitchCostumeTo => "looks_switchcostumeto",
            LooksSwitchBackdropTo => "looks_switchbackdropto",
            LooksSwitchBackdropToAndWait => "looks_switchbackdroptoandwait",
            LooksNextCostume => "looks_nextcostume",
            LooksNextBackdrop => "looks_nextbackdrop",
            LooksChangeEffectBy => "looks_changeeffectby",
            LooksSetEffectTo => "looks_seteffectto",
            LooksClearGraphicEffects => "looks_cleargraphiceffects",
            LooksSetSizeTo => "looks_setsizeto",
            LooksCostume => "looks_costume",
            LooksCostumeNumberName => "looks_costumenumbername",
            LooksGoForwardBackwardLayers => "looks_goforwardbackwardlayers",
            LooksGotoFrontBack => "looks_gotofrontback",
            LooksChangeSizeBy => "looks_changesizeby",
            LooksSize => "looks_size",
            LooksBackdrops => "looks_backdrops",
            LooksBackdropNumberName => "looks_backdropnumbername",
            LooksChangeStretchBy => "looks_changestretchby",
            LooksSetStretchTo => "looks_setstretchto"
        },
        Sound {
            SoundPlay => "sound_play",
            SoundPlayUntilDone => "sound_playuntildone",
            SoundStopAllSounds => "sound_stopallsounds",
            SoundChangeEffectBy => "sound_changeeffectby",
            SoundSetEffectTo => "sound_seteffectto",
            SoundClearEffects => "sound_cleareffects",
            SoundChangeVolumeBy => "sound_changevolumeby",
            SoundSetVolumeTo => "sound_setvolumeto",
            SoundVolume => "sound_volume",
            SoundSoundsMenu => "sound_sounds_menu",
            SoundBeatsMenu => "sound_beats_menu",
            SoundEffectsMenu => "sound_effects_menu",
        },
        Event {
            EventWhenTouchingObject => "event_whentouchingobject",
            EventBroadcast => "event_broadcast",
            EventBroadcastAndWait => "event_broadcastandwait",
            EventWhenGreaterThan => "event_whengreaterthan",
            EventWhenFlagClicked => "event_whenflagclicked",
            EventWhenKeyPressed => "event_whenkeypressed",
            EventWhenThisSpriteClicked => "event_whenthisspriteclicked",
            EventWhenStageClicked => "event_whenstageclicked",
            EventWhenBackdropSwitchesTo => "event_whenbackdropswitchesto",
            EventWhenBroadcastReceived => "event_whenbroadcastreceived"
        },
        Control {
            ControlRepeat => "control_repeat",
            ControlRepeatUntil => "control_repeat_until",
            ControlWhile => "control_while",
            ControlForEach => "control_for_each",
            ControlForever => "control_forever",
            ControlWait => "control_wait",
            ControlWaitUntil => "control_wait_until",
            ControlIf => "control_if",
            ControlIfElse => "control_if_else",
            ControlStop => "control_stop",
            ControlCreateCloneOf => "control_create_clone_of",
            ControlDeleteThisClone => "control_delete_this_clone",
            ControlGetCounter => "control_get_counter",
            ControlIncrCounter => "control_incr_counter",
            ControlClearCounter => "control_clear_counter",
            ControlAllAtOnce => "control_all_at_once",
            ControlStartAsClone => "control_start_as_clone",
            ControlCreateCloneOfMenu => "control_create_clone_of_menu"
        },
        Sensing {
            SensingTouchingObject => "sensing_touchingobject",
            SensingTouchingColor => "sensing_touchingcolor",
            SensingColorIsTouchingColor => "sensing_coloristouchingcolor",
            SensingDistanceTo => "sensing_distanceto",
            SensingDistanceToMenu=> "sensing_distancetomenu",
            SensingTimer => "sensing_timer",
            SensingResetTimer => "sensing_resettimer",
            SensingOf => "sensing_of",
            SensingMouseX => "sensing_mousex",
            SensingMouseY => "sensing_mousey",
            SensingSetDragMode => "sensing_setdragmode",
            SensingMouseDown => "sensing_mousedown",
            SensingKeyPressed => "sensing_keypressed",
            SensingCurrent => "sensing_current",
            SensingDaysSince2000 => "sensing_dayssince2000",
            SensingLoudness => "sensing_loudness",
            SensingLoud => "sensing_loud",
            SensingAskAndWait => "sensing_askandwait",
            SensingAnswer => "sensing_answer",
            SensingUsername => "sensing_username",
            SensingOnline => "sensing_online",
            SensingKeyOptions => "sensing_keyoptions",
            SensingTouchingObjectMenu => "sensing_touchingobjectmenu",
            SensingOfObjectMenu => "sensing_of_object_menu",
            SensingUserid => "sensing_userid"
        },
        Operator {
            OperatorAdd => "operator_add",
            OperatorSubtract => "operator_subtract",
            OperatorMultiply => "operator_multiply",
            OperatorDivide => "operator_divide",
            OperatorLt => "operator_lt",
            OperatorEquals => "operator_equals",
            OperatorGt => "operator_gt",
            OperatorAnd => "operator_and",
            OperatorOr => "operator_or",
            OperatorNot => "operator_not",
            OperatorRandom => "operator_random",
            OperatorJoin => "operator_join",
            OperatorLetterOf => "operator_letter_of",
            OperatorLength => "operator_length",
            OperatorContains => "operator_contains",
            OperatorMod => "operator_mod",
            OperatorRound => "operator_round",
            OperatorMathOp => "operator_mathop"
        },
        Data {
            DataVariable => "data_variable",
            DataSetVariableTo => "data_setvariableto",
            DataChangeVariableBy => "data_changevariableby",
            DataHideVariable => "data_hidevariable",
            DataShowVariable => "data_showvariable",
            DataListContents => "data_listcontents",
            DataAddToList => "data_addtolist",
            DataDeleteOfList => "data_deleteoflist",
            DataDeleteAllOfList => "data_deletealloflist",
            DataInsertAtList => "data_insertatlist",
            DataReplaceItemOfList => "data_replaceitemoflist",
            DataItemOfList => "data_itemoflist",
            DataItemNumOfList => "data_itemnumoflist",
            DataLengthOfList => "data_lengthoflist",
            DataListContainsItem => "data_listcontainsitem",
            DataHideList => "data_hidelist",
            DataShowList => "data_showlist"
        },
        Procedures {
            ProceduresDefinition => "procedures_definition",
            ProceduresCall => "procedures_call",
            ProceduresPrototype => "procedures_prototype",
            ArgumentReporterStringNumber => "argument_reporter_string_number",
            ArgumentReporterBoolean => "argument_reporter_boolean"
        },
        Pen {
            PenClear => "pen_clear",
            PenStamp => "pen_stamp",
            PenDown => "pen_penDown",
            PenUp => "pen_penUp",
            PenSetPenColorToColor => "pen_setPenColorToColor",
            PenChangePenColorParamBy => "pen_changePenColorParamBy",
            PenSetPenColorParamTo => "pen_setPenColorParamTo",
            PenChangePenSizeBy => "pen_changePenSizeBy",
            PenSetPenSizeTo => "pen_setPenSizeTo",
            PenSetPenShadeToNumber => "pen_setPenShadeToNumber",
            PenChangePenShadeBy => "pen_changePenShadeBy",
            PenSetPenHueToNumber => "pen_setPenHueToNumber",
            PenChangePenHueBy => "pen_changePenHueBy",
            PenMenuColorParam => "pen_menu_colorParam"
        }
    }
}

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
            let stage: Stage = serde_json::from_value(v).map_err(|e| D::Error::custom(e.to_string()))?;
            Ok(StageOrSprite::Stage(stage))
        } else {
            let sprite: Sprite = serde_json::from_value(v).map_err(|e| D::Error::custom(e.to_string()))?;
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
                            op_codes_set.insert(t.opcode.to_string());
                        }
                    }
                }
                StageOrSprite::Sprite(v) => {
                    for j in &v.blocks {
                        if let BlockAndTopLevelPrimitive::Block(t) = j.1 {
                            op_codes_set.insert(t.opcode.to_string());
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
#[derive(Debug, Deserialize, Clone)]
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
    pub base: Target,
    pub name: String,
    pub isStage: bool,
    pub tempo: Option<f64>,
    pub videoTransparency: Option<f64>,
    pub videoState: Option<VideoState>,
    pub layerOrder: Option<u32>,
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
    pub name: String,
    pub isStage: bool,
    pub visible: Option<bool>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub size: Option<f64>,
    pub direction: Option<f64>,
    pub draggable: Option<bool>,
    pub rotationStyle: Option<RotationStyle>,
    pub layerOrder: Option<u32>,
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
    pub opcode: BlockOpCodes,
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
    MutationProceduresPrototype(MutationProceduresPrototype),
    MutationProceduresCall(MutationProceduresCall),
    MutationControlStop(MutationControlStop),
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct MutationProceduresCall {
    pub tagName: Option<String>,
    pub proccode: String,
    pub argumentids: StringOrStringArray,
    pub warp: Option<WarpValue>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct MutationProceduresPrototype {
    pub tagName: Option<String>,
    pub proccode: String,
    pub argumentids: String,
    pub argumentnames: String,
    pub argumentdefaults: String,
    pub warp: Option<WarpValue>,
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
    pub blockId: Option<String>,
    pub text: String,
    pub minimized: Option<bool>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct Costume {
    pub assetId: String,
    pub bitmapResolution: Option<f64>,
    pub dataFormat: ImageFormat,
    pub md5ext: Option<String>,
    pub name: String,
    pub rotationCenterX: Option<f64>,
    pub rotationCenterY: Option<f64>,
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

#[derive(Debug, Deserialize, Clone)]
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
    pub assetId: String,
    pub dataFormat: AudioFormat,
    pub md5ext: Option<String>,
    pub name: String,
    pub rate: Option<f64>,
    pub sampleCount: Option<f64>,
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
    Reference(String),
}

#[repr(u8)]
#[derive(Debug, Deserialize_repr, PartialEq)]
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
    V2((SameBlockShadowOrNoShadow, Option<InputPrimitiveOrReference>)),
    V3(
        (
            DiffBlockShadow,
            Option<InputPrimitiveOrReference>,
            Option<InputPrimitiveOrReference>,
        ),
    ),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Fields {
    V1((String,)),
    V2((String, Option<String>)),
}
