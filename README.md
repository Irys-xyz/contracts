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
await arweave.api.get("/mint/<WALLET_ADDRESS>/10000000000000000");
```

Now keep this terminal open, because after each deployment, we need to instruct `ArLocal`
to mine a new block.

### Deploy Token Contract

In another terminal, run `yarn deploy` to deploy the contract:

```sh
cd tokens
yarn deploy -g http://localhost:1984 -w path/to/my/wallet.json -s deploy/state/example-state.json
```

Remember to mine a block in the node.js terminal

```js
await arweave.api.get("mine");
```

### Deploy Bundlers Contract

In another terminal, run `yarn deploy` to deploy the contract:

```js
cd bundlers
yarn deploy -g http://localhost:1984 -w path/to/my/wallet.json -s deploy/state/example-state.json -t <TOKEN_CONTRACT_ADDRESS>
```

Remember to mine a block in the node.js terminal

```js
await arweave.api.get("mine");
```

### Deploy Validators Contract

In another terminal, run `yarn deploy` to deploy the contract:

```js
cd validators
yarn deploy -g http://localhost:1984 -w path/to/my/wallet.json -s deploy/state/example-state.json -t <TOKEN_CONTRACT_ADDRESS> -c <BUNDLERS_CONTRACT_ADDRESS> -a <BUNDLER_ADDRESS>
```

Remember to mine a block in the node.js terminal

```js
await arweave.api.get("mine");
```
