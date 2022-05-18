use serde::{Deserialize, Serialize};

use bundlr_contracts_shared::{u128_utils, TransactionId};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, PartialOrd, Serialize)]
pub struct Epoch {
    #[serde(with = "u128_utils")]
    pub seq: u128,
    pub tx: TransactionId,
    #[serde(with = "u128_utils")]
    pub height: u128,
}

impl Epoch {
    pub fn next(&self, tx: TransactionId, height: u128) -> Self {
        Epoch {
            seq: self.seq + 1,
            tx,
            height,
        }
    }
}
