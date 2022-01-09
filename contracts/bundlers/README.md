# Bundler contract
---
### ***Mutators***

## Methods

### `join`

> **Add your bundler to the network**

This will stake your tokens in this contract. Make sure the operator has called `approve` on the token contract with sufficient tokens to allow this contract to custody the tokens

### `leave`

> **Remove your bundler from the network**

This will start a countdown of 1 month. This allows people using the bundler to migrate to another. After 1 month the stake will be moved back to the tokens address

### `syncSlashed`

> **View slashed bundlers on the validator contract and reallocate tokens appropiately**

Delete bundler and move all tokens to a treasury address

---
### ***Views***