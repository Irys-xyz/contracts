# Validator contract

## Methods
---
### ***Mutators***

### `join`

> **Add your validator to the network linked to a specific bundler**

This will stake your tokens in this contract. Make sure the operator has called `approve` on the token contract with sufficient tokens to allow this contract to custody the tokens

Tax stake

### `leave`

> **Remove your validator from the network**

This is only allows if you're not currently a leader

### `syncSlashed`

> **View slashed bundlers on the validator contract and reallocate tokens appropiately**

Delete bundler and move all tokens to a treasury address

### `updateEpoch`

> **Update epoch number and nominate new leaders**

### `nominate`

> **Nominate a new set of 10 leaders**

If number of validators < 10 then choose all

### `propose`

> **Propose a vote** (usually to slash a bundler)

### `vote`

> **Vote on a proposal**

One vote per validator

---
### ***Views***

### `leaders`

> **Get all current epoch leaders**