const fs = require("node:fs/promises");
const http = require("node:http");
const path = require("node:path");
const process = require("node:process");
const { URL } = require("node:url");

const { Command } = require("commander");

const Arweave = require("arweave");
const { LoggerFactory, SmartWeaveNodeFactory } = require("redstone-smartweave");

async function readJwk(filepath) {
  let f = path.resolve(process.cwd(), filepath);
  return fs.readFile(f).then((walletData) => {
    let json = JSON.parse(walletData.toString());
    return json;
  });
}

function defaultPort(protocol) {
  switch (protocol) {
    case "http:":
      return 80;
    case "https:":
      return 443;
    default:
      throw Error(`Unsupported protocol: ${protocol}`);
  }
}

async function run(args) {
  let arweaveUrl = new URL(args.gateway);
  let wallet = await readJwk(args.wallet);

  let arweave = Arweave.init({
    host: arweaveUrl.hostname,
    port: arweaveUrl.port ? arweaveUrl.port : defaultPort(arweaveUrl.protocol),
    protocol: arweaveUrl.protocol.split(":")[0], // URL holds colon at the end of the protocol
  });

  let smartweave = SmartWeaveNodeFactory.memCached(arweave);

  let walletAddress = await arweave.wallets.jwkToAddress(wallet);

  let contractSrc = await fs.readFile(
    path.join(__dirname, "../../pkg/rust-contract_bg.wasm")
  );

  let initialState = JSON.parse(
    await fs.readFile(path.resolve(process.cwd(), args.state), "utf8")
  );

  if (args.token) {
    console.warn(
      `Override token contract address from initial state file, using: ${args.token}`
    );
    initialState.token = args.token;
  }

  initialState.allowedInteractors.push(walletAddress);
  // Just in case deduplicate
  initialState.allowedInteractors = [
    ...new Set(initialState.allowedInteractors),
  ];

  let deploymentData = {
    wallet,
    initState: JSON.stringify(initialState),
    src: contractSrc,
    wasmSrcCodeDir: path.join(__dirname, "../../src"),
    wasmGlueCode: path.join(__dirname, "../../pkg/rust-contract.js"),
  };

  // deploying contract using the new SDK.
  return await smartweave.createContract.deploy(deploymentData);
}

let appVersion;

if (process.env.npm_package_version) {
  appVersion = process.env.npm_package_version;
} else {
  appVersion = require("../../package.json").version;
}

const appArgs = new Command();
appArgs
  .version(appVersion)
  .requiredOption("-g, --gateway <url>", "Arweave gateway URL")
  .requiredOption("-w, --wallet <path>", "Path to Arweave wallet file")
  .requiredOption(
    "-s, --state <path>",
    "Path to JSON file defining initial state"
  )
  .option("-t, --token <address>", "Override token address");

run(appArgs.parse(process.argv).opts())
  .then((txId) => {
    console.error(`Deployment done, tx=${txId}`);
    process.exit(0);
  })
  .catch((err) => {
    console.error("Deployment failed: ", err);
    process.exit(1);
  });
