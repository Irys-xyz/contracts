# Staking contract

## Methods
---
### ***Mutators***

### `stake`

>**Stake users token in contract**

Make sure users have called `transferFrom` to allow staking contract to have custody

### `unstake`

> **Return stake to user's address**

### `syncSlashed`

> **View slashed bundlers on the validator contract and reallocate tokens appropriately**

Return all staking balances to users

-----
### ***Views***

### `getStake`

Returns the staking balance of a specific address


