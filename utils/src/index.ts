import fs from "node:fs/promises";
import path from "node:path";
import { URL } from "node:url";

import { Command } from "commander";

import Arweave from "arweave";
import { JWKInterface } from "arweave/node/lib/wallet";
import { WarpNodeFactory } from "warp-contracts";

import {
  deploy as deployTokenContract,
  TokenContract,
  TokenState,
} from "../../token/ts/contract";
import {
  deploy as deployBundlersContract,
  BundlersContract,
} from "../../bundlers/ts/contract";
import {
  deploy as deployValidatorsContract,
  ValidatorsContract,
} from "../../validators/ts/contract";

type Secrets = {
  wallets: {
    ["token-contract-owner"]: JWKInterface;
    ["bundlers-contract-owner"]: JWKInterface;
    ["bundler-1"]: JWKInterface;
    ["bundler-2"]: JWKInterface;
    ["validator-1"]: JWKInterface;
    ["validator-2"]: JWKInterface;
    ["validator-3"]: JWKInterface;
    ["validator-4"]: JWKInterface;
    ["validator-5"]: JWKInterface;
    ["validator-6"]: JWKInterface;
    ["validator-7"]: JWKInterface;
    ["test1"]: JWKInterface;
  };
};

function readSecrets(filepath: string): Promise<Secrets> {
  let f = path.resolve(process.cwd(), filepath);
  return fs.readFile(f).then((walletData) => {
    let json = JSON.parse(walletData.toString());
    return json;
  });
}

type CliArgs = {
  gateway: string;
  secrets: string;
  arlocal: boolean;
};

function defaultPort(protocol: string) {
  switch (protocol) {
    case "http:":
      return 80;
    case "https:":
      return 443;
    default:
      throw Error(`Unsupported protocol: ${protocol}`);
  }
}

class ArLocal {
  arweave: Arweave;
  constructor(arweave: Arweave) {
    this.arweave = arweave;
  }
  async mine() {
    return this.arweave.api.get("mine");
  }
}

interface MaybeCallback<T> {
  (value: T): void;
}

class Maybe<T> {
  value?: T;
  constructor(value?: T) {
    this.value = value;
  }
  then(cb: MaybeCallback<T>): void {
    if (this.value) {
      cb(this.value);
    }
  }
}

async function run(args: CliArgs) {
  const arweaveUrl = new URL(args.gateway);
  const secrets = await readSecrets(args.secrets);

  const gwPort = arweaveUrl.port
    ? Number.parseInt(arweaveUrl.port)
    : defaultPort(arweaveUrl.protocol);

  const arweave = Arweave.init({
    host: arweaveUrl.hostname,
    port: gwPort,
    protocol: arweaveUrl.protocol.split(":")[0], // URL holds colon at the end of the protocol
  });

  const arlocal = new Maybe(args.arlocal ? new ArLocal(arweave) : undefined);

  const warp = WarpNodeFactory.memCached(arweave);

  const tokenContractOwnerAddress = await arweave.wallets.jwkToAddress(
    secrets.wallets["test1"]
  );
  const bundlersContractOwnerAddress = await arweave.wallets.jwkToAddress(
    secrets.wallets["bundlers-contract-owner"]
  );
  const bundlerAddresses = await Promise.all([
    arweave.wallets.jwkToAddress(secrets.wallets["bundler-1"]),
    arweave.wallets.jwkToAddress(secrets.wallets["bundler-2"]),
  ]);
  const validatorAddresses = await Promise.all([
    arweave.wallets.jwkToAddress(secrets.wallets["validator-1"]),
    arweave.wallets.jwkToAddress(secrets.wallets["validator-2"]),
    arweave.wallets.jwkToAddress(secrets.wallets["validator-3"]),
    arweave.wallets.jwkToAddress(secrets.wallets["validator-4"]),
    arweave.wallets.jwkToAddress(secrets.wallets["validator-5"]),
    arweave.wallets.jwkToAddress(secrets.wallets["validator-6"]),
    arweave.wallets.jwkToAddress(secrets.wallets["validator-7"]),
  ]);

  const tokenContractSrc = await fs.readFile(
    path.join(__dirname, "../../token/pkg/rust-contract_bg.wasm")
  );

  const tokenContractStateFromFile: TokenState = await fs
    .readFile(
      path.resolve(`${__dirname}/../data/initial-token-state.json`),
      "utf8"
    )
    .then((data) => JSON.parse(data.toString()));

  const initialTokenContractState: TokenState = {
    ...tokenContractStateFromFile,
    ...{
      owner: tokenContractOwnerAddress,
      balances: {
        [tokenContractOwnerAddress]:
          tokenContractStateFromFile.totalSupply.toString(),
      },
    },
  };

  const { contractTxId: tokenContractTxId } = await deployTokenContract(
    warp,
    secrets.wallets["test1"],
    initialTokenContractState,
    !args.arlocal
  );

  arlocal.then(async (arlocal) => await arlocal.mine());

  const initialBundlersContractState = await fs
    .readFile(
      path.resolve(`${__dirname}/../data/initial-bundlers-state.json`),
      "utf8"
    )
    .then((data) => JSON.parse(data.toString()));

  initialBundlersContractState.token = tokenContractTxId;
  initialBundlersContractState.allowedInteractors.push(
    bundlersContractOwnerAddress
  );
  // just to be sure, deduplicate
  initialBundlersContractState.allowedInteractors = [
    ...new Set(initialBundlersContractState.allowedInteractors),
  ];

  await deployBundlersContract(
    warp,
    secrets.wallets["bundlers-contract-owner"],
    initialBundlersContractState,
    !args.arlocal
  );

  arlocal.then(async (arlocal) => await arlocal.mine());
}

let appVersion: string;

if (process.env.npm_package_version) {
  appVersion = process.env.npm_package_version;
} else {
  appVersion = require("../package.json").version;
}

const appArgs = new Command();
appArgs
  .version(appVersion)
  .requiredOption("-g, --gateway <url>", "Arweave gateway URL")
  .requiredOption("-s, --secrets <path>", "Path to file containing all secrets")
  .option("-a, --arlocal", "Deploy to ArLocal");

run(appArgs.parse(process.argv).opts())
  .then((txId) => {
    console.error("Done");
    process.exit(0);
  })
  .catch((err) => {
    console.error("Deployment failed: ", err);
    process.exit(1);
  });
