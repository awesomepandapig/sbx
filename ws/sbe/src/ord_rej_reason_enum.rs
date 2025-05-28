#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OrdRejReasonEnum {
    UnknownOrder = 0x5_u8,
    DuplicateOrder = 0x6_u8,
    StaleOrder = 0x8_u8,
    Other = 0x63_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for OrdRejReasonEnum {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x5_u8 => Self::UnknownOrder,
            0x6_u8 => Self::DuplicateOrder,
            0x8_u8 => Self::StaleOrder,
            0x63_u8 => Self::Other,
            _ => Self::NullVal,
        }
    }
}
impl From<OrdRejReasonEnum> for u8 {
    #[inline]
    fn from(v: OrdRejReasonEnum) -> Self {
        match v {
            OrdRejReasonEnum::UnknownOrder => 0x5_u8,
            OrdRejReasonEnum::DuplicateOrder => 0x6_u8,
            OrdRejReasonEnum::StaleOrder => 0x8_u8,
            OrdRejReasonEnum::Other => 0x63_u8,
            OrdRejReasonEnum::NullVal => 0xff_u8,
        }
    }
}
impl core::str::FromStr for OrdRejReasonEnum {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "UnknownOrder" => Ok(Self::UnknownOrder),
            "DuplicateOrder" => Ok(Self::DuplicateOrder),
            "StaleOrder" => Ok(Self::StaleOrder),
            "Other" => Ok(Self::Other),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for OrdRejReasonEnum {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnknownOrder => write!(f, "UnknownOrder"),
            Self::DuplicateOrder => write!(f, "DuplicateOrder"),
            Self::StaleOrder => write!(f, "StaleOrder"),
            Self::Other => write!(f, "Other"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
