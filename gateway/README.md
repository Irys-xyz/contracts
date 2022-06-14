# Contract Gateway

## Build

This project uses `validators` contract that depends also from `token` and `bundlers` sub-projects. These sub-projects needs to be build before running the gateway.

Build dependencies:

```sh
cd ../token
yarn build

cd ../bundlers
yarn build

cd ../validators
yarn build
```

Build the gateway:

```sh
yarn build
```

## Run

When starting the gateway, it needs a path to the wallet file and address of the `validators` contract. Defaults for the other CLI args should be OK:

```sh
$ yarn start -h
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
$ yarn start -c "<VALIDATORS_CONTRACT_ADDRESS>" -w /path/to/my/wallet.json
```

## Run Tests

```sh
$ yarn test
```
