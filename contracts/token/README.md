# Token contract

## Methods
---
### ***Mutators***

### `transfer`

>**Transfer tokens from A -> B**

Throws if A and B aren't valid addresses
Throws if amount == 0
Throws if A doesn't have enough

### `transferFrom`

> **Transfer from A -> B if B is approved to move funds**

### `approve`

> **Approve another contract to custody tokens for you** (similar to ERC20)

### `propose`

> **Create a proposal for other token holders to vote on** 

TBC

### `vote`

> **Vote on a pre-existing proposal**

TBC

-----
### ***Views***

### `balance`

Returns the balance of a specific address



