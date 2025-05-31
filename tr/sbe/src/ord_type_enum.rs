#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OrdTypeEnum {
    Market = 49_u8, 
    Limit = 50_u8, 
    #[default]
    NullVal = 0_u8, 
}
impl From<u8> for OrdTypeEnum {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            49_u8 => Self::Market, 
            50_u8 => Self::Limit, 
            _ => Self::NullVal,
        }
    }
}
impl From<OrdTypeEnum> for u8 {
    #[inline]
    fn from(v: OrdTypeEnum) -> Self {
        match v {
            OrdTypeEnum::Market => 49_u8, 
            OrdTypeEnum::Limit => 50_u8, 
            OrdTypeEnum::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for OrdTypeEnum {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "Market" => Ok(Self::Market), 
            "Limit" => Ok(Self::Limit), 
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for OrdTypeEnum {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Market => write!(f, "Market"), 
            Self::Limit => write!(f, "Limit"), 
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
