use std::{convert::Infallible, ops::Deref, str::FromStr};

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd)]
pub struct Address(String);

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = de::Deserialize::deserialize(deserializer)?;
        Address::from_str(s).map_err(de::Error::custom)
    }
}

impl Deref for Address {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Address {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Address(s.to_owned()))
    }
}

impl AsRef<str> for Address {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
