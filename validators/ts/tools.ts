import fs from "node:fs/promises";
import path from "node:path";
import { URL } from "node:url";

import dotenv from "dotenv";
import { Command } from "commander";

import Arweave from "arweave";
import { JWKInterface } from "arweave/node/lib/wallet";
import { WarpNodeFactory } from "warp-contracts";
import { connect } from "./contract";

function readJwk(filepath: string): Promise<JWKInterface> {
  let f = path.resolve(process.cwd(), filepath);
  return fs.readFile(f).then((walletData) => {
    let json = JSON.parse(walletData.toString());
    return json as JWKInterface;
  });
}

type JoinArgs = {
  gateway: string;
  contract: string;
  wallet: string;
  stake: string;
  url: string;
};

type GetValidatorsArgs = {
  gateway: string;
  contract: string;
  wallet: string;
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

async function join(args: JoinArgs) {
  let arweaveUrl = new URL(args.gateway);
  let validatorUrl = new URL(args.url);

  let wallet = await readJwk(args.wallet);

  let arweave: Arweave = Arweave.init({
    host: arweaveUrl.hostname,
    port: arweaveUrl.port ? arweaveUrl.port : defaultPort(arweaveUrl.protocol),
    protocol: arweaveUrl.protocol.split(":")[0], // URL holds colon at the end of the protocol
  });

  let smartweave = WarpNodeFactory.memCached(arweave);

  let connection = await connect(smartweave, args.contract, wallet);
  let res = await connection.join(BigInt(args.stake), validatorUrl);

  console.log(res);
}

async function getValidators(args: GetValidatorsArgs) {
  let arweaveUrl = new URL(args.gateway);

  let wallet = await readJwk(args.wallet);

  let arweave: Arweave = Arweave.init({
    host: arweaveUrl.hostname,
    port: arweaveUrl.port ? arweaveUrl.port : defaultPort(arweaveUrl.protocol),
    protocol: arweaveUrl.protocol.split(":")[0], // URL holds colon at the end of the protocol
  });

  let smartweave = WarpNodeFactory.memCached(arweave);

  let connection = await connect(smartweave, args.contract, wallet);
  let res = await connection.validators();

  console.log(res);
}

let appVersion: string;

if (process.env.npm_package_version) {
  appVersion = process.env.npm_package_version;
} else {
  appVersion = require("../package.json").version;
}

dotenv.config();

let program = new Command();

program.version(appVersion);

program
  .command("join")
  .option(
    "-g, --gateway <url>",
    "Arweave gateway URL",
    process.env.BUNDLR_ARWEAVE
      ? process.env.BUNDLR_ARWEAVE
      : "https://arweave.net"
  )
  .requiredOption(
    "-c, --contract <address>",
    "Token contract address",
    process.env.BUNDLR_TOKEN_CONTRACT
  )
  .requiredOption("-w, --wallet <path>", "Path to wallet file")
  .requiredOption("-s, --stake <amount>", "Amount to stake when joining")
  .requiredOption("-u, --url <url>", "Validator URL")
  .action((opts) => {
    join(opts)
      .then(() => {
        process.exit(0);
      })
      .catch((err) => {
        console.error("Application failed: ", err);
        process.exit(1);
      });
  });

program
  .command("get-validators")
  .option(
    "-g, --gateway <url>",
    "Arweave gateway URL",
    process.env.BUNDLR_ARWEAVE
      ? process.env.BUNDLR_ARWEAVE
      : "https://arweave.net"
  )
  .requiredOption(
    "-c, --contract <address>",
    "Token contract address",
    process.env.BUNDLR_TOKEN_CONTRACT
  )
  .requiredOption("-w, --wallet <path>", "Path to wallet file")
  .action((opts) => {
    getValidators(opts)
      .then(() => {
        process.exit(0);
      })
      .catch((err) => {
        console.error("Application failed: ", err);
        process.exit(1);
      });
  });

program.parse();
