#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CxlRejResponseToEnum {
    OrderCancelRequest = 49_u8, 
    OrderCancelReplaceRequest = 50_u8, 
    #[default]
    NullVal = 0_u8, 
}
impl From<u8> for CxlRejResponseToEnum {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            49_u8 => Self::OrderCancelRequest, 
            50_u8 => Self::OrderCancelReplaceRequest, 
            _ => Self::NullVal,
        }
    }
}
impl From<CxlRejResponseToEnum> for u8 {
    #[inline]
    fn from(v: CxlRejResponseToEnum) -> Self {
        match v {
            CxlRejResponseToEnum::OrderCancelRequest => 49_u8, 
            CxlRejResponseToEnum::OrderCancelReplaceRequest => 50_u8, 
            CxlRejResponseToEnum::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for CxlRejResponseToEnum {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "OrderCancelRequest" => Ok(Self::OrderCancelRequest), 
            "OrderCancelReplaceRequest" => Ok(Self::OrderCancelReplaceRequest), 
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for CxlRejResponseToEnum {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OrderCancelRequest => write!(f, "OrderCancelRequest"), 
            Self::OrderCancelReplaceRequest => write!(f, "OrderCancelReplaceRequest"), 
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
