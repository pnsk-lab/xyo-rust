use serde_repr::Deserialize_repr;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize_repr)]
pub enum Shadow {
    SameBlockShadow = 1,
    NoShadow = 2,
    DiffBlockShadow = 3,
}

impl Shadow {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for Shadow {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            1 => Self::SameBlockShadow,
            2 => Self::NoShadow,
            3 => Self::DiffBlockShadow,
            _ => return Err(()),
        })
    }
}
