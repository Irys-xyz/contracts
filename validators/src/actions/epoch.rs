use rand_xoshiro::rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

use bundlr_contracts_shared::{
    contract_utils::js_imports::{Block, Transaction},
    Address,
};

use crate::{
    action::ActionResult, contract_utils::handler_result::HandlerResult, error::ContractError,
    state::State,
};

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

fn pick_random_nominees<Rng>(rng: &mut Rng, validators: &[Address], count: u8) -> Vec<Address>
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

pub async fn update_epoch(mut state: State) -> ActionResult {
    if (Block::height() as u128) < state.epoch.height + (state.epoch_duration as u128) {
        return Err(ContractError::UpdateEpochBlocked);
    }

    state.epoch = state.epoch.next(Transaction::id(), Block::height() as u128);

    let seed = TransactionBasedRngSeed::try_from(Transaction::id().as_str()).map_err(|()| {
        ContractError::RuntimeError("could not extract 32 bytes from Transaction::id()".to_string())
    })?;

    let mut rng = Xoshiro256PlusPlus::from_seed(seed.0);

    // Pick 10 random nominees or pick all if number of validatros is 10 or less
    if state.validators.len() <= state.max_num_nominated_validators as usize {
        state.nominated_validators = state.validators.iter().cloned().collect::<Vec<Address>>();
    } else {
        state.nominated_validators = pick_random_nominees(
            &mut rng,
            &state.validators,
            state.max_num_nominated_validators,
        );
    };

    Ok(HandlerResult::NewState(state))
}
