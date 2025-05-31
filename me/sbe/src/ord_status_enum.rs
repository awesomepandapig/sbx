#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OrdStatusEnum {
    New = 48_u8,
    PartiallyFilled = 49_u8,
    Filled = 50_u8,
    Canceled = 52_u8,
    Rejected = 56_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for OrdStatusEnum {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            48_u8 => Self::New,
            49_u8 => Self::PartiallyFilled,
            50_u8 => Self::Filled,
            52_u8 => Self::Canceled,
            56_u8 => Self::Rejected,
            _ => Self::NullVal,
        }
    }
}
impl From<OrdStatusEnum> for u8 {
    #[inline]
    fn from(v: OrdStatusEnum) -> Self {
        match v {
            OrdStatusEnum::New => 48_u8,
            OrdStatusEnum::PartiallyFilled => 49_u8,
            OrdStatusEnum::Filled => 50_u8,
            OrdStatusEnum::Canceled => 52_u8,
            OrdStatusEnum::Rejected => 56_u8,
            OrdStatusEnum::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for OrdStatusEnum {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "New" => Ok(Self::New),
            "PartiallyFilled" => Ok(Self::PartiallyFilled),
            "Filled" => Ok(Self::Filled),
            "Canceled" => Ok(Self::Canceled),
            "Rejected" => Ok(Self::Rejected),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for OrdStatusEnum {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::New => write!(f, "New"),
            Self::PartiallyFilled => write!(f, "PartiallyFilled"),
            Self::Filled => write!(f, "Filled"),
            Self::Canceled => write!(f, "Canceled"),
            Self::Rejected => write!(f, "Rejected"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
