use serde::Deserialize;
use serde_repr::Deserialize_repr;

use crate::types::inputtype::InputType;

#[repr(u8)]
#[derive(Debug, Deserialize_repr)]
pub enum NumPrimitiveInputTypes {
    Number = InputType::Number as u8,
    PositiveNumber = InputType::PositiveNumber as u8,
    PositiveInteger = InputType::PositiveInteger as u8,
    Integer = InputType::Integer as u8,
    Angle = InputType::Angle as u8,
}
impl NumPrimitiveInputTypes {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}
impl TryFrom<u8> for NumPrimitiveInputTypes {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            4 => Self::Number,
            5 => Self::PositiveNumber,
            6 => Self::PositiveInteger,
            7 => Self::Integer,
            8 => Self::Angle,
            _ => return Err(()),
        })
    }
}
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum StringOrNumber {
    String(String),
    Number(f64),
}

pub type NumPrimitive = (NumPrimitiveInputTypes, StringOrNumber);

#[repr(u8)]
#[derive(Debug, Deserialize_repr)]
pub enum ColorPrimitiveInputTypes {
    Color = InputType::Color as u8,
}
impl ColorPrimitiveInputTypes {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}
impl TryFrom<u8> for ColorPrimitiveInputTypes {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            9 => ColorPrimitiveInputTypes::Color,
            _ => return Err(()),
        })
    }
}

pub type ColorPrimitive = (ColorPrimitiveInputTypes, String);

#[repr(u8)]
#[derive(Debug, Deserialize_repr)]
pub enum TextPrimitiveInputTypes {
    String = InputType::String as u8,
}
impl TextPrimitiveInputTypes {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}
impl TryFrom<u8> for TextPrimitiveInputTypes {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            10 => TextPrimitiveInputTypes::String,
            _ => return Err(()),
        })
    }
}

pub type TextPrimitive = (TextPrimitiveInputTypes, StringOrNumber);

#[repr(u8)]
#[derive(Debug, Deserialize_repr)]
pub enum BroadcastPrimitiveInputTypes {
    Broadcast = InputType::Broadcast as u8,
}
impl BroadcastPrimitiveInputTypes {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}
impl TryFrom<u8> for BroadcastPrimitiveInputTypes {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            11 => BroadcastPrimitiveInputTypes::Broadcast,
            _ => return Err(()),
        })
    }
}

pub type BroadcastPrimitive = (BroadcastPrimitiveInputTypes, String, String);

#[repr(u8)]
#[derive(Debug, Deserialize_repr)]
pub enum VariablePrimitiveInputTypes {
    Variable = InputType::Variable as u8,
}
impl VariablePrimitiveInputTypes {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}
impl TryFrom<u8> for VariablePrimitiveInputTypes {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            12 => VariablePrimitiveInputTypes::Variable,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum VariablePrimitive {
    V3((VariablePrimitiveInputTypes, String, String)),
    V5(
        (
            VariablePrimitiveInputTypes,
            String,
            String,
            Option<f64>,
            Option<f64>,
        ),
    ),
}

#[repr(u8)]
#[derive(Debug, Deserialize_repr)]
pub enum ListPrimitiveInputTypes {
    List = InputType::List as u8,
}
impl ListPrimitiveInputTypes {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}
impl TryFrom<u8> for ListPrimitiveInputTypes {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            13 => ListPrimitiveInputTypes::List,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ListPrimitive {
    V3((ListPrimitiveInputTypes, String, String)),
    V5(
        (
            ListPrimitiveInputTypes,
            String,
            String,
            Option<f64>,
            Option<f64>,
        ),
    ),
}

#[repr(u8)]
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum InputPrimitive {
    NumPrimitive(NumPrimitive),
    ColorPrimitive(ColorPrimitive),
    TextPrimitive(TextPrimitive),
    BroadcastPrimitive(BroadcastPrimitive),
    VariablePrimitive(VariablePrimitive),
    ListPrimitive(ListPrimitive),
}

#[repr(u8)]
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TopLevelPrimitive {
    VariablePrimitive(VariablePrimitive),
    ListPrimitive(ListPrimitive),
}
