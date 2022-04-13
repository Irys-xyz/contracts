use std::num::ParseIntError;
use std::ops::Deref;
use std::str::FromStr;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::contract_utils::handler_result::HandlerResult;
use crate::error::ContractError;
use crate::state::State;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub struct Address(String);

impl Deref for Address {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for Address {
    type Error = ();

    fn try_from(val: &str) -> Result<Self, ()> {
        // TODO: Check that the address is a valid Arweave address
        Ok(Address(val.to_string()))
    }
}

impl TryFrom<String> for Address {
    type Error = ();

    fn try_from(val: String) -> Result<Self, ()> {
        // TODO: Check that the address is a valid Arweave address
        Ok(Address(val))
    }
}

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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "function")]
pub enum Action {
    Name,
    Symbol,
    Decimals,
    TotalSupply,
    BalanceOf {
        target: String,
    },
    Allowance {
        target: String,
        spender: String,
    },
    Transfer {
        to: String,
        amount: String,
    },
    TransferFrom {
        from: String,
        to: String,
        amount: String,
    },
    Approve {
        spender: String,
        amount: String,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum QueryResponseMsg {
    Balance {
        balance: Amount,
        ticker: String,
        target: Address,
    },
    Decimals(u8),
    Name(Option<String>),
    Symbol(String),
    TotalSupply(Amount),
}

pub type ActionResult = Result<HandlerResult<State, QueryResponseMsg>, ContractError>;
