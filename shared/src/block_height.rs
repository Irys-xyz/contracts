use std::{
    num::ParseIntError,
    ops::{Add, Deref, Sub},
    str::FromStr,
};

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd)]
pub struct BlockHeight(pub u128);

impl Serialize for BlockHeight {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for BlockHeight {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = de::Deserialize::deserialize(deserializer)?;
        s.parse().map_err(de::Error::custom)
    }
}

impl FromStr for BlockHeight {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BlockHeight(s.parse()?))
    }
}

impl From<u128> for BlockHeight {
    fn from(val: u128) -> Self {
        BlockHeight(val)
    }
}

impl Deref for BlockHeight {
    type Target = u128;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Sub for BlockHeight {
    type Output = BlockHeight;

    fn sub(self, rhs: Self) -> Self::Output {
        BlockHeight(self.0 - rhs.0)
    }
}

impl Add for BlockHeight {
    type Output = BlockHeight;

    fn add(self, rhs: Self) -> Self::Output {
        BlockHeight(self.0 + rhs.0)
    }
}
