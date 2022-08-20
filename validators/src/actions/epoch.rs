use std::str::FromStr;

use bundlr_contracts_shared::TransactionId;
use rand_xoshiro::rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

use bundlr_contracts_shared::Address;

#[cfg(feature = "js-runtime")]
use bundlr_contracts_shared::contract_utils::js_imports::{Block, Transaction};

#[cfg(feature = "js-runtime")]
use crate::{
    action::ActionResult, contract_utils::handler_result::HandlerResult, error::ContractError,
    state::State,
};

use super::slashing;

struct TransactionBasedRngSeed(pub [u8; 32]);

impl TryFrom<&str> for TransactionBasedRngSeed {
    type Error = ();
    fn try_from(tx_hash: &str) -> Result<Self, Self::Error> {
        let bytes = data_encoding::BASE64URL_NOPAD
            .decode(tx_hash.as_bytes())
            .unwrap();

        if bytes.len() < 32 {
            Err(())
        } else {
            let bytes = bytes
                .iter()
                .take(32)
                .fold((0usize, [0u8; 32]), |mut acc, byte| {
                    acc.1[acc.0] = *byte;
                    acc.0 += 1;
                    acc
                });

            Ok(Self(bytes.1))
        }
    }
}

fn pick_random_nominees<Rng>(rng: &mut Rng, validators: &[&Address], count: u8) -> Vec<Address>
where
    Rng: RngCore,
{
    let mut addresses = Vec::with_capacity(count as usize);
    let mut range = (0..validators.len()).collect::<Vec<usize>>();
    for i in 0..count as usize {
        // get random index, but when selecting the index, skip first i indices as those
        // are already randomized.
        let random_val = rng.next_u32();
        let random_index: usize = (random_val as usize % (validators.len() - i)) + i;
        range.swap(i, random_index);
        addresses.push(validators[range[i]].clone());
    }
    addresses
}

#[cfg(feature = "js-runtime")]
pub async fn update_epoch(mut state: State) -> ActionResult {
    if (Block::height() as u128) <= state.epoch.height {
        return Err(ContractError::UpdateEpochBlocked);
    }

    // NOTE: this is a bit hacky, but way to initialize this to block data
    // from the block where the contract deployment happened and similarly
    // there is currently no way to access the information for the block
    // height for the tx where the contract got deployed. Epoch sequence
    // number zero inidicates that we aren't yet in an epoch.
    // TODO: should we have some kind of grace period before the epoch becomes active?
    let next_epoch_height = if state.epoch.seq == 0 {
        Block::height() as u128
    } else {
        state.epoch.height + state.epoch_duration as u128
    };

    state.epoch = state.epoch.next(
        TransactionId::from_str(&Transaction::id()).map_err(|err| {
            ContractError::ParseError(format!("Failed to parse transaction ID: {}", err))
        })?,
        next_epoch_height,
    );

    let seed = TransactionBasedRngSeed::try_from(Transaction::id().as_str()).map_err(|()| {
        ContractError::RuntimeError("could not extract 32 bytes from Transaction::id()".to_string())
    })?;

    let mut rng = Xoshiro256PlusPlus::from_seed(seed.0);

    // Pick 10 random nominees or pick all if number of validatros is 10 or less
    if state.validators.len() <= state.max_num_nominated_validators as usize {
        state.nominated_validators = state.validators.keys().cloned().collect::<Vec<Address>>();
    } else {
        state.nominated_validators = pick_random_nominees(
            &mut rng,
            &state.validators.keys().collect::<Vec<&Address>>(),
            state.max_num_nominated_validators,
        );
    };

    // on each epoch update, check if there are any expired slash proposals
    slashing::on_update_epoch(&mut state, Block::height() as u128);

    Ok(HandlerResult::NewState(state))
}
