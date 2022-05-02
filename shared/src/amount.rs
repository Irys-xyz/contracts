use std::{
    num::ParseIntError,
    ops::{Add, Deref, Sub},
    str::FromStr,
};

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd)]
pub struct Amount(u128);

impl Serialize for Amount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Amount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = de::Deserialize::deserialize(deserializer)?;
        s.parse().map_err(de::Error::custom)
    }
}

impl Amount {
    pub const ZERO: Amount = Amount(0);
}

impl FromStr for Amount {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Amount(s.parse()?))
    }
}

impl From<u128> for Amount {
    fn from(val: u128) -> Self {
        Amount(val)
    }
}

impl Deref for Amount {
    type Target = u128;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Sub for Amount {
    type Output = Amount;

    fn sub(self, rhs: Self) -> Self::Output {
        Amount(self.0 - rhs.0)
    }
}

impl Add for Amount {
    type Output = Amount;

    fn add(self, rhs: Self) -> Self::Output {
        Amount(self.0 + rhs.0)
    }
}
