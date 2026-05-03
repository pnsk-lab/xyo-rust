use serde_repr::Deserialize_repr;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize_repr)]
pub enum InputType {
    Number = 4,
    PositiveNumber = 5,
    PositiveInteger = 6,
    Integer = 7,
    Angle = 8,
    Color = 9,
    String = 10,
    Broadcast = 11,
    Variable = 12,
    List = 13,
}

impl TryFrom<u8> for InputType {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            4 => Self::Number,
            5 => Self::PositiveNumber,
            6 => Self::PositiveInteger,
            7 => Self::Integer,
            8 => Self::Angle,
            9 => Self::Color,
            10 => Self::String,
            11 => Self::Broadcast,
            12 => Self::Variable,
            13 => Self::List,
            _ => return Err(()),
        })
    }
}
