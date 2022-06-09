# Contract Gateway

## Build

This project uses `validators` contract that depends also from `token` and `bundlers` sub-projects. These sub-projects needs to be build before running the gateway.

Build dependencies:

```sh
cd ../token
npm build

cd ../bundlers
npm build

cd ../validators
npm build
```

Build the gateway:

```sh
npm build
```

## Run

When starting the gateway, it needs a path to the wallet file and address of the `validators` contract. Defaults for the other CLI args should be OK:

```sh
$ npm start -- -h

> bundlr-contract-gateway@1.0.0 start
> node dist/index.js "-h"

Usage: index [options]

Options:
  -V, --version   output the version number
  -a, --arweave   Arweave connection
  -l, --listen    Listen to address
  -c, --contract  Contract address
  -w, --wallet    Path to Arweave wallet file
  -h, --help      display help for command
```

To start the gateway:

```sh
$ npm start -- -c "<VALIDATORS_CONTRACT_ADDRESS>" -w /path/to/my/wallet.json
```

## Run Tests

```sh
$ npm test
```
