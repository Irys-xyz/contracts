use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{
    action::ActionResult, contract_utils::handler_result::HandlerResult, error::ContractError,
    state::State,
};
use bundlr_contracts_shared::{u128_utils, Address, Amount, TransactionId};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Proposal {
    id: String,
    size: usize,
    #[serde(with = "u128_utils")]
    fee: u128,
    currency: String,
    #[serde(with = "u128_utils")]
    block: u128,
    validator: String,
    signature: String,
}

pub type Validator = Address;
pub type Stake = Amount;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Vote {
    For,
    Against,
}

impl From<i128> for Vote {
    fn from(vote: i128) -> Self {
        // In case of tie, vote against
        if vote > 0 {
            Self::For
        } else {
            Self::Against
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Voting {
    Open(HashMap<Validator, Vote>),
    Closed {
        votes: HashMap<Validator, (Vote, Stake)>,
        final_vote: Vote,
    },
}

impl Voting {
    pub fn is_open(&self) -> bool {
        match self {
            Voting::Open(_) => true,
            Voting::Closed {
                votes: _,
                final_vote: _,
            } => false,
        }
    }
}

impl Default for Voting {
    fn default() -> Self {
        Voting::Open(HashMap::new())
    }
}

// each validator can propose single slashing per epoch
pub async fn propose(
    mut state: State,
    caller: Address,
    current_tx_id: TransactionId,
    current_block_height: u128,
    proposal: Proposal,
) -> ActionResult {
    if !state.validators.contains_key(&caller) {
        return Err(ContractError::InvalidValidator(caller));
    }

    let tx_id = TransactionId::from_str(&proposal.id).map_err(|_| {
        ContractError::ParseError(format!("Failed to parse transaction ID: {}", proposal.id))
    })?;

    // if tx is already proposed, return Err(AlreadyProposed)
    if state.slash_proposals.contains_key(&tx_id) {
        return Err(ContractError::AlreadyProposed(tx_id));
    }

    let current_epoch_start = if state.epoch.height > current_block_height {
        // Epoch is already updated and reflects the next one
        state.epoch.height - state.epoch_duration as u128
    } else {
        state.epoch.height
    };

    if let Some(_) = state
        .slash_proposals
        .iter()
        .find(|(_, (_, reporter, height, _, _))| {
            *reporter == caller && current_epoch_start < *height
        })
    {
        // if validator has already made proposal in this epoch, return Err(TooManyProposals)
        return Err(ContractError::TooManyProposals);
    }

    let voting_data = {
        let mut votes = HashMap::new();
        votes.insert(caller.clone(), Vote::For);
        Voting::Open(votes)
    };

    state.slash_proposals.insert(
        tx_id,
        (
            proposal,
            caller,
            current_block_height,
            current_tx_id,
            voting_data,
        ),
    );

    Ok(HandlerResult::NewState(state))
}

pub async fn vote(
    mut state: State,
    caller: Address,
    current_block_height: u128,
    tx: TransactionId,
    vote: Vote,
) -> ActionResult {
    if !state.validators.contains_key(&caller) {
        return Err(ContractError::InvalidValidator(caller));
    }

    let (_, _, height, _, ref mut voting_data) =
        if let Some(data) = state.slash_proposals.get_mut(&tx) {
            data
        } else {
            // No proposal found matching the transaction ID
            return Err(ContractError::InvalidTransactionId);
        };

    let votes = if let Voting::Open(votes) = voting_data {
        votes
    } else {
        // if voting is already concluded
        return Err(ContractError::VotingClosed);
    };

    // if proposal is expired
    if current_block_height as u128 > (*height + (state.slash_proposal_lifetime as u128)) {
        return Err(ContractError::ProposalExpired);
    }

    if let Some(_) = votes.insert(caller, vote) {
        // if caller has already voted
        return Err(ContractError::AlreadyVoted);
    }

    let (total_stake, voted_stake, result) = evaluate_votes(&state.validators, &votes);

    // close voting, if all votes are casted or the remaining stake cannot flip the vote
    if total_stake - voted_stake < result.abs() as u128 || total_stake == voted_stake {
        let result = result.into();
        let votes = votes
            .iter()
            .map(|(validator, vote)| {
                (
                    validator.clone(),
                    (*vote, state.validators.get(validator).unwrap().clone()),
                )
            })
            .collect();
        *voting_data = Voting::Closed {
            votes,
            final_vote: result,
        };

        match result {
            Vote::For => on_positive_voting_result(&state, &tx),
            Vote::Against => on_negative_voting_result(&state, &tx),
        }
    }

    Ok(HandlerResult::NewState(state))
}

fn evaluate_votes(
    validators: &HashMap<Validator, Stake>,
    votes: &HashMap<Validator, Vote>,
) -> (u128, u128, i128) {
    // dereference Amount to u128 so that the computaion later is easier
    let total_stake = *validators
        .iter()
        .fold(Amount::default(), |total_stake, (_, stake)| {
            total_stake + *stake
        });

    let (voted_stake, result) = votes.iter().fold(
        (0u128, 0i128),
        |(voted_stake, result), (validator, vote)| {
            let voter_stake = validators
                .get(validator)
                .expect("Could not find the validator who has voted earlier");
            let stake_weighted_vote =
                match vote {
                    Vote::For => i128::try_from(**voter_stake)
                        .expect("Could not fit validator stake in i128"),
                    Vote::Against => -i128::try_from(**voter_stake)
                        .expect("Could not fit validator stake in i128"),
                };
            (voted_stake + **voter_stake, result + stake_weighted_vote)
        },
    );
    (total_stake, voted_stake, result)
}

fn on_positive_voting_result(_state: &State, _tx: &TransactionId) {
    // validators voted for bundler to be slashed

    // no-op, at this point we do nothing
    // TODO: implement in the next version of the contract
}

fn on_negative_voting_result(_state: &State, _tx: &TransactionId) {
    // invalid proposal causes validator to be thrown out and stake being returned

    // no-op, at this point we do nothing
    // TODO: implement in the next version of the contract
}

pub(super) fn on_update_epoch(state: &mut State, current_block_height: u128) {
    // proposal is valid for state.slash_proposal_lifetime blocks
    // update_epoch will check all proposals that have expired without conclusive result
    // if cast votes cover 75% of all stake, then calculate result for the vote
    // else define resulta as negative -> validator who proposed to be kicked out and stake
    // returned

    state
        .slash_proposals
        .iter_mut()
        .filter(|(_, (_, _, height, _, voting_data))| {
            current_block_height > height + state.slash_proposal_lifetime as u128
                && voting_data.is_open()
        })
        .map(|(_, (_, _, _, _, voting_data))| voting_data)
        .for_each(|voting_data| {
            let votes = match voting_data {
                Voting::Open(votes) => votes,
                Voting::Closed {
                    votes: _,
                    final_vote: _,
                } => unreachable!(), // We already checked above that this is open
            };

            let (total_stake, voted_stake, result) = evaluate_votes(&state.validators, &votes);

            let final_vote = if voted_stake as f64 > 0.75 * total_stake as f64 {
                Vote::from(result)
            } else {
                Vote::Against
            };
            let votes = votes
                .iter()
                .map(|(validator, vote)| {
                    (
                        validator.clone(),
                        (*vote, state.validators.get(validator).unwrap().clone()),
                    )
                })
                .collect();
            *voting_data = Voting::Closed { votes, final_vote }
        });
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, str::FromStr};

    use bundlr_contracts_shared::{Address, Amount};
    use futures::executor::LocalPool;

    use crate::{
        actions::slashing::{propose, Proposal, Stake, Validator, Voting},
        contract_utils::handler_result::HandlerResult,
        epoch::Epoch,
        error::ContractError,
        state::State,
    };

    use super::{evaluate_votes, on_update_epoch, vote, TransactionId, Vote};

    fn state() -> State {
        static VALIDATORS_AND_STAKES: [(&str, u128); 13] = [
            ("a1", 10000),
            ("a2", 20000),
            ("a3", 10000),
            ("a4", 15000),
            ("a5", 10000),
            ("a6", 10000),
            ("a7", 10010),
            ("a8", 10000),
            ("a9", 10010),
            ("a10", 10000),
            ("a11", 10000),
            ("a12", 30000),
            ("a13", 15000),
        ];

        let validators = VALIDATORS_AND_STAKES
            .iter()
            .map(|(address, stake)| (Validator::from_str(address).unwrap(), stake.into()))
            .collect::<HashMap<Validator, Stake>>();

        let nominated_validators = VALIDATORS_AND_STAKES[2..12]
            .iter()
            .map(|(address, _)| Validator::from_str(address).unwrap())
            .collect::<Vec<Validator>>();

        let slash_proposals = [
            (
                TransactionId::from_str("tx1").unwrap(),
                (
                    Proposal {
                        id: "tx1".to_string(),
                        size: 100,
                        fee: 100,
                        currency: "BTC".to_string(),
                        block: 1900,
                        validator: "a1".to_string(),
                        signature: "foo".to_string(),
                    },
                    Validator::from_str("a1").unwrap(),
                    2350,
                    TransactionId::from_str("proposal_tx_id_1").unwrap(),
                    Voting::Closed {
                        votes: [
                            ("a1", Vote::For),
                            ("a2", Vote::For),
                            ("a3", Vote::For),
                            ("a4", Vote::For),
                            ("a5", Vote::For),
                            ("a6", Vote::For),
                            ("a12", Vote::For),
                        ]
                        .into_iter()
                        .map(|(validator, vote)| {
                            let validator = Validator::from_str(validator).unwrap();
                            let stake = validators.get(&validator).unwrap();
                            (validator, (vote, *stake))
                        })
                        .collect(),
                        final_vote: Vote::For,
                    },
                ),
            ),
            (
                TransactionId::from_str("tx2").unwrap(),
                (
                    Proposal {
                        id: "tx2".to_string(),
                        size: 100,
                        fee: 100,
                        currency: "BTC".to_string(),
                        block: 1900,
                        validator: "a3".to_string(),
                        signature: "foo".to_string(),
                    },
                    Validator::from_str("a3").unwrap(),
                    2350,
                    TransactionId::from_str("proposal_tx_id_2").unwrap(),
                    Voting::Open(
                        [
                            ("a3", Vote::For),
                            ("a1", Vote::For),
                            ("a2", Vote::Against),
                            ("a4", Vote::For),
                            ("a5", Vote::For),
                            ("a6", Vote::For),
                            ("a10", Vote::For),
                            ("a11", Vote::For),
                            ("a7", Vote::For),
                        ]
                        .into_iter()
                        .map(|(validator, vote)| (validator.try_into().unwrap(), vote))
                        .collect(),
                    ),
                ),
            ),
            (
                TransactionId::from_str("tx3").unwrap(),
                (
                    Proposal {
                        id: "tx3".to_string(),
                        size: 100,
                        fee: 100,
                        currency: "BTC".to_string(),
                        block: 2300,
                        validator: "a4".to_string(),
                        signature: "foo".to_string(),
                    },
                    Validator::from_str("a3").unwrap(),
                    2350,
                    TransactionId::from_str("proposal_tx_id_2").unwrap(),
                    Voting::Open(
                        [
                            ("a4", Vote::For),
                            ("a1", Vote::For),
                            ("a2", Vote::Against),
                            ("a3", Vote::For),
                            ("a6", Vote::For),
                            ("a5", Vote::Against),
                            ("a9", Vote::For),
                            ("a7", Vote::Against),
                            ("a8", Vote::Against),
                            ("a10", Vote::Against),
                            ("a12", Vote::For),
                            ("a11", Vote::Against),
                        ]
                        .into_iter()
                        .map(|(validator, vote)| (validator.try_into().unwrap(), vote))
                        .collect(),
                    ),
                ),
            ),
        ]
        .into_iter()
        .collect::<HashMap<TransactionId, (Proposal, Validator, u128, TransactionId, Voting)>>();

        State {
            bundler: Address::from_str("bundler").unwrap(),
            bundlers_contract: Address::from_str("bundler_contract").unwrap(),
            epoch: Epoch {
                seq: 5,
                tx: TransactionId::from_str("epoch_update_tx_id").unwrap(),
                height: 2522,
            },
            epoch_duration: 500,
            minimum_stake: Amount::from(100),
            token: Address::from_str("token_address").unwrap(),
            max_num_nominated_validators: 10,
            validators,
            nominated_validators,
            slash_proposal_lifetime: 300,
            slash_proposals,
        }
    }

    #[test]
    fn evaluate_votes_returns_total_stake_value() {
        let state = state();

        let votes = match state
            .slash_proposals
            .get("tx2".try_into().as_ref().unwrap())
            .unwrap()
            .4
        {
            Voting::Open(ref votes) => votes,
            _ => unreachable!(),
        };

        let (total_stake, _, _) = evaluate_votes(&state.validators, votes);

        assert_eq!(total_stake, 170020);
    }

    #[test]
    fn evaluate_votes_returns_voted_stake_value() {
        let state = state();

        let votes = match state
            .slash_proposals
            .get("tx2".try_into().as_ref().unwrap())
            .unwrap()
            .4
        {
            Voting::Open(ref votes) => votes,
            _ => unreachable!(),
        };

        let (_, voted_stake, _) = evaluate_votes(&state.validators, votes);

        assert_eq!(voted_stake, 105010);
    }

    #[test]
    fn evaluate_votes_returns_stake_weighted_result() {
        let state = state();

        let votes = match state
            .slash_proposals
            .get("tx2".try_into().as_ref().unwrap())
            .unwrap()
            .4
        {
            Voting::Open(ref votes) => votes,
            _ => unreachable!(),
        };

        let (_, _, result) = evaluate_votes(&state.validators, votes);

        assert_eq!(result, 65010);
    }

    #[test]
    fn on_update_epoch_closes_expired_proposals() {
        let mut state = state();

        on_update_epoch(&mut state, 2700);

        match state
            .slash_proposals
            .get("tx2".try_into().as_ref().unwrap())
            .unwrap()
            .4
        {
            Voting::Open(_) => unreachable!(),
            Voting::Closed {
                votes: _,
                final_vote: _,
            } => (),
        };
    }

    #[test]
    fn voting_is_prevented_if_proposal_is_expired() {
        let mut pool = LocalPool::new();

        let state = state();

        let result = pool.run_until(vote(
            state,
            "a12".try_into().unwrap(),
            2700,
            "tx2".try_into().unwrap(),
            Vote::For,
        ));

        assert_eq!(result, Err(ContractError::ProposalExpired));
    }

    #[test]
    fn voting_is_prevented_if_vote_is_already_concluded() {
        let mut pool = LocalPool::new();

        let state = state();

        let result = pool.run_until(vote(
            state,
            "a12".try_into().unwrap(),
            2400,
            "tx1".try_into().unwrap(),
            Vote::For,
        ));

        assert_eq!(result, Err(ContractError::VotingClosed));
    }

    #[test]
    fn when_vote_adds_stake_enough_to_conculude_the_vote_then_voting_is_closed_and_result_evaluated(
    ) {
        let mut pool = LocalPool::new();

        let state = state();

        if let Ok(HandlerResult::NewState(state)) = pool.run_until(vote(
            state,
            "a12".try_into().unwrap(),
            2400,
            "tx2".try_into().unwrap(),
            Vote::For,
        )) {
            match state
                .slash_proposals
                .get("tx2".try_into().as_ref().unwrap())
                .unwrap()
                .4
            {
                Voting::Open(_) => {
                    unreachable!("Voting should have been closed")
                }
                _ => (),
            }
        } else {
            unreachable!("Vote should have succeeded")
        }
    }

    #[test]
    fn when_last_vote_causes_a_tie_voting_is_closed_with_negative_result() {
        let mut pool = LocalPool::new();

        let state = state();

        if let Ok(HandlerResult::NewState(state)) = pool.run_until(vote(
            state,
            "a13".try_into().unwrap(),
            2400,
            "tx3".try_into().unwrap(),
            Vote::Against,
        )) {
            match state
                .slash_proposals
                .get("tx3".try_into().as_ref().unwrap())
                .unwrap()
                .4
            {
                Voting::Open(_) => unreachable!("Voting should have been closed"),
                Voting::Closed {
                    votes: _,
                    final_vote: Vote::For,
                } => {
                    unreachable!("Voting should have resulted against the vote")
                }
                Voting::Closed {
                    ref votes,
                    final_vote: Vote::Against,
                } => {
                    let (votes_for, votes_against) =
                        votes
                            .iter()
                            .fold((0, 0), |acc, (_, (vote, stake))| match vote {
                                Vote::For => (acc.0 + **stake, acc.1),
                                Vote::Against => (acc.0, acc.1 + **stake),
                            });

                    assert_eq!(votes_for, votes_against);
                    assert_eq!(votes_for + votes_against, 170020);
                }
            }
        } else {
            unreachable!("Vote should have succeeded")
        }
    }

    #[test]
    fn only_a_joined_validator_can_propose() {
        let mut pool = LocalPool::new();

        let state = state();

        let caller: Address = "a14".try_into().unwrap();

        let result = pool.run_until(propose(
            state,
            caller.clone(),
            "tx100".try_into().unwrap(),
            2200,
            Proposal {
                id: "tx5".to_string(),
                size: 100,
                fee: 100,
                currency: "BTC".to_string(),
                block: 1900,
                validator: "a14".to_string(),
                signature: "foo".to_string(),
            },
        ));

        assert_eq!(result, Err(ContractError::InvalidValidator(caller)));
    }

    #[test]
    fn only_a_joined_validator_can_vote() {
        let mut pool = LocalPool::new();

        let state = state();

        let caller: Address = "a14".try_into().unwrap();

        let result = pool.run_until(vote(
            state,
            caller.clone(),
            2200,
            "tx2".try_into().unwrap(),
            Vote::For,
        ));

        assert_eq!(result, Err(ContractError::InvalidValidator(caller)));
    }
}
