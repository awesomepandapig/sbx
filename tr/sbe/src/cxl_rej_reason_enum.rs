#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CxlRejReasonEnum {
    TooLateToCancel = 48_u8, 
    UnknownOrder = 49_u8, 
    DuplicateClOrdID = 54_u8, 
    #[default]
    NullVal = 0_u8, 
}
impl From<u8> for CxlRejReasonEnum {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            48_u8 => Self::TooLateToCancel, 
            49_u8 => Self::UnknownOrder, 
            54_u8 => Self::DuplicateClOrdID, 
            _ => Self::NullVal,
        }
    }
}
impl From<CxlRejReasonEnum> for u8 {
    #[inline]
    fn from(v: CxlRejReasonEnum) -> Self {
        match v {
            CxlRejReasonEnum::TooLateToCancel => 48_u8, 
            CxlRejReasonEnum::UnknownOrder => 49_u8, 
            CxlRejReasonEnum::DuplicateClOrdID => 54_u8, 
            CxlRejReasonEnum::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for CxlRejReasonEnum {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "TooLateToCancel" => Ok(Self::TooLateToCancel), 
            "UnknownOrder" => Ok(Self::UnknownOrder), 
            "DuplicateClOrdID" => Ok(Self::DuplicateClOrdID), 
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for CxlRejReasonEnum {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TooLateToCancel => write!(f, "TooLateToCancel"), 
            Self::UnknownOrder => write!(f, "UnknownOrder"), 
            Self::DuplicateClOrdID => write!(f, "DuplicateClOrdID"), 
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
