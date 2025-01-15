use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum EventType {
    Mint,
    Burn,
    Swap,
}

impl EventType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            EventType::Mint => "Mint",
            EventType::Burn => "Burn",
            EventType::Swap => "Swap",
        }
    }
}

impl TryFrom<i32> for EventType {
    type Error = crate::Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(EventType::Mint),
            1 => Ok(EventType::Burn),
            2 => Ok(EventType::Swap),
            _ => Err(crate::Error::UnknownEventType(v)),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

impl Side {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Side::Buy => "Buy",
            Side::Sell => "Sell",
        }
    }
}

impl TryFrom<i32> for Side {
    type Error = crate::Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Side::Buy),
            1 => Ok(Side::Sell),
            _ => Err(crate::Error::UnknownSide(v)),
        }
    }
}
