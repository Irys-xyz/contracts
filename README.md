# Bundlr Contracts

## Testing

### Start ArLocal

Start with defaults from command line:

```sh
npx arlocal
```

Or start in a node.js terminal:

```js
const ArLocal = require("arlocal").default;
const arlocal = new ArLocal(1984, false);
await arlocal.start();
```

Then open a node.js terminal or continue in the previous one:

```js
const Arweave = require("arweave");
let arweave = Arweave.init({ host: "localhost", port: 1984, protocol: "http" });
```

Add some AR to the address matching your wallet:

```js
await arweave.api.get("/mint/<WALLET_ADDRESS_1>/10000000000000000");
await arweave.api.get("/mint/<WALLET_ADDRESS_2>/10000000000000000");
```

Now keep this terminal open, because after each deployment, we need to instruct `ArLocal`
to mine a new block.

### Deploy Token Contract

In another terminal, run `yarn deploy` to deploy the contract:

```sh
cd tokens
yarn deploy \
    -g http://localhost:1984 \
    -w path/to/wallet1.json \
    -s example-data/example-state.json
```

Remember to mine a block in the node.js terminal

```js
await arweave.api.get("mine");
```

Move some tokens to second wallet address:

```sh
cd tokens
npx ts-node ts/tools.ts \
    -g http://localhost:1984 \
    -w path/to/wallet1.json \
    -c <TOKEN_CONTRACT_ADDRESS> \
    -r <WALLET_ADDRESS_2> \
    -a 200
```

And mine a block in the node.js terminal

```js
await arweave.api.get("mine");
```

### Deploy Bundlers Contract

In another terminal, run `yarn deploy` to deploy the contract:

```js
cd bundlers
yarn deploy \
    -g http://localhost:1984 \
    -w path/to/wallet1.json \
    -s deploy/state/example-state.json \
    -t <TOKEN_CONTRACT_ADDRESS>
```

Remember to mine a block in the node.js terminal

```js
await arweave.api.get("mine");
```

### Bundler Join

Allow second wallet address to interact (join) using bundlers contract:

```sh
cd bundlers
npx ts-node ts/tools.ts allow \
    -g http://localhost:1984 \
    -w path/to/wallet1.json \
    -a <WALLET_ADDRESS_2>
```

```js
await arweave.api.get("mine");
```

Approve token transfer from second wallet address to the bundlers contract address:

```sh
cd tokens
npx ts-node ts/tools.ts approve \
    -g http://localhost:1984 \
    -w path/to/wallet2.json \
    -c <TOKEN_CONTRACT_ADDRESS> \
    -s <BUNDLER_CONTRACT_ADDRESS> \
    -a 100
```

```js
await arweave.api.get("mine");
```

Join as a new bundler

```sh
cd bundlers
npx ts-node ts/tools.ts join \
    -g http://localhost:1984 \
    -w path/to/wallet2.json \
    -c <BUNDLERS_CONTRACT_ADDRESS>
```

```js
await arweave.api.get("mine");
```

### Deploy Validators Contract

In another terminal, run `yarn deploy` to deploy the contract:

```js
cd validators
yarn deploy \
    -g http://localhost:1984 \
    -w path/to/wallet2.json \
    -s ./example-data/example-state.json \
    -t <TOKEN_CONTRACT_ADDRESS> \
    -c <BUNDLERS_CONTRACT_ADDRESS> \
    -a <WALLET_ADDRESS_2>
```

Remember to mine a block in the node.js terminal

```js
await arweave.api.get("mine");
```

### Validator Join

Approve token transfer from first wallet address to the validators contract address:

```sh
cd tokens
npx ts-node ts/tools.ts approve \
    -g http://localhost:1984 \
    -w path/to/wallet1.json \
    -c <TOKEN_CONTRACT_ADDRESS> \
    -s <VALIDATORS_CONTRACT_ADDRESS> \
    -a 10
```

```js
await arweave.api.get("mine");
```

Join as a new validator using first wallet address:

```sh
npx ts-node ts/tools.ts join \
    -g http://localhost:1984 \
    -w path/to/wallet1.json \
    -c <VALIDATORS_CONTRACT_ADDRESS> \
    -s 10 \
    -u https://example.com
```
