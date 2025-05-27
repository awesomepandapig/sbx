#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ExecTypeEnum {
    New = 48_u8, 
    Canceled = 52_u8, 
    Rejected = 56_u8, 
    Trade = 70_u8, 
    #[default]
    NullVal = 0_u8, 
}
impl From<u8> for ExecTypeEnum {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            48_u8 => Self::New, 
            52_u8 => Self::Canceled, 
            56_u8 => Self::Rejected, 
            70_u8 => Self::Trade, 
            _ => Self::NullVal,
        }
    }
}
impl From<ExecTypeEnum> for u8 {
    #[inline]
    fn from(v: ExecTypeEnum) -> Self {
        match v {
            ExecTypeEnum::New => 48_u8, 
            ExecTypeEnum::Canceled => 52_u8, 
            ExecTypeEnum::Rejected => 56_u8, 
            ExecTypeEnum::Trade => 70_u8, 
            ExecTypeEnum::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for ExecTypeEnum {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "New" => Ok(Self::New), 
            "Canceled" => Ok(Self::Canceled), 
            "Rejected" => Ok(Self::Rejected), 
            "Trade" => Ok(Self::Trade), 
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for ExecTypeEnum {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::New => write!(f, "New"), 
            Self::Canceled => write!(f, "Canceled"), 
            Self::Rejected => write!(f, "Rejected"), 
            Self::Trade => write!(f, "Trade"), 
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
