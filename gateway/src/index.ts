import fs from "node:fs/promises";
import http from "http";
import { URL } from "url";

import dotenv from "dotenv";

import { Command } from "commander";

import Arweave from "arweave";
import { JWKInterface } from "arweave/node/lib/wallet";
import { WarpNodeFactory } from "warp-contracts";

import app from "./app";

function readJwk(path: string): Promise<JWKInterface> {
  return fs.readFile(path).then((walletData) => {
    let json = JSON.parse(walletData.toString());
    return json as JWKInterface;
  });
}

type CliArgs = {
  arweave: string;
  listen: string;
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

async function run(args: CliArgs) {
  let arweaveUrl = new URL(args.arweave);
  let listenUrl = new URL(args.listen);

  if (listenUrl.protocol != "http:") {
    throw Error(`Unsupported listen protocol: ${listenUrl.protocol}`);
  }

  let bindPort = listenUrl.port
    ? parseInt(listenUrl.port)
    : defaultPort(listenUrl.protocol);
  let bindHostname = listenUrl.hostname;

  console.log(
    `Create contract gateway: arweave=${args.arweave}, contract=${args.contract}`
  );

  let wallet = await readJwk(args.wallet);

  let arweave: Arweave = Arweave.init({
    host: arweaveUrl.hostname,
    port: arweaveUrl.port ? arweaveUrl.port : defaultPort(arweaveUrl.protocol),
    protocol: arweaveUrl.protocol.split(":")[0], // URL holds colon at the end of the protocol
  });

  let smartweave = WarpNodeFactory.memCached(arweave);

  let appInstance = await app.create(
    arweave,
    smartweave,
    args.contract,
    wallet
  );

  console.log(
    `Validator wallet: path=${
      args.wallet
    }, address=${await arweave.wallets.getAddress(wallet)}`
  );

  return new Promise((resolve, reject) => {
    const server = http.createServer(appInstance);

    server.on("listening", () => {
      console.log(
        `Contract gateway is up and running at ${listenUrl.protocol}//${bindHostname}:${bindPort}`
      );
    });

    server.on("error", (err) => {
      reject(err);
    });

    server.on("close", () => {
      resolve(null);
    });

    server.listen(bindPort, bindHostname);
  });
}

let appVersion;

if (process.env.npm_package_version) {
  appVersion = process.env.npm_package_version;
} else {
  appVersion = require("../package.json").version;
}

dotenv.config();

const appArgs = new Command();
appArgs
  .version(appVersion)
  .option(
    "-a, --arweave",
    "Arweave connection",
    process.env.GW_ARWEAVE ? process.env.GW_ARWEAVE : "https://arweave.net"
  )
  .option(
    "-l, --listen",
    "Listen to address",
    process.env.GW_LISTEN ? process.env.GW_LISTEN : "http://127.0.0.1:3000"
  )
  .requiredOption("-c, --contract", "Contract address", process.env.GW_CONTRACT)
  .requiredOption(
    "-w, --wallet",
    "Path to Arweave wallet file",
    process.env.GW_WALLET
  );

run(appArgs.parse(process.argv).opts())
  .then(() => {
    console.error("Contract gateway done");
    process.exit(0);
  })
  .catch((err) => {
    console.error("Contract gateway failed: ", err);
    process.exit(1);
  });
