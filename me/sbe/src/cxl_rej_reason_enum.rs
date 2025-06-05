#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CxlRejReasonEnum {
    UnknownOrder = 49_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for CxlRejReasonEnum {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            49_u8 => Self::UnknownOrder,
            _ => Self::NullVal,
        }
    }
}
impl From<CxlRejReasonEnum> for u8 {
    #[inline]
    fn from(v: CxlRejReasonEnum) -> Self {
        match v {
            CxlRejReasonEnum::UnknownOrder => 49_u8,
            CxlRejReasonEnum::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for CxlRejReasonEnum {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "UnknownOrder" => Ok(Self::UnknownOrder),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for CxlRejReasonEnum {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnknownOrder => write!(f, "UnknownOrder"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
