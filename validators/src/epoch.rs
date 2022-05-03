use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

/// Deserializer from string to u128
fn de_u128<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u128, D::Error> {
    let s: &str = de::Deserialize::deserialize(deserializer)?;
    s.parse().map_err(de::Error::custom)
}

/// Serializer u128 as string
fn ser_u128<S>(val: &u128, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&val.to_string())
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, PartialOrd, Serialize)]
pub struct Epoch {
    #[serde(serialize_with = "ser_u128", deserialize_with = "de_u128")]
    pub seq: u128,
    pub tx: String,
    #[serde(serialize_with = "ser_u128", deserialize_with = "de_u128")]
    pub height: u128,
}

impl Epoch {
    pub fn next(&self, tx: String, height: u128) -> Self {
        Epoch {
            seq: self.seq + 1,
            tx,
            height,
        }
    }
}
