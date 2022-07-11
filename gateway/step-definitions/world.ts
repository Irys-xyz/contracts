import fs from "node:fs";
import path from "node:path";

import { setWorldConstructor, After, Before, World } from "@cucumber/cucumber";

import arweave from "./arweave";
import Arweave from "arweave";

import {
  connect as connectTokenContract,
  deploy as deployTokenContract,
  TokenState,
} from "../../token/ts/contract";

import {
  connect as connectBundlersContract,
  deploy as deployBundlersContract,
} from "../../bundlers/ts/contract";

import { connect, deploy, State } from "../../validators/ts/contract";

import { addFunds, mineBlock } from "./utils";
import { JWKInterface } from "arweave/node/lib/wallet";

import { Express } from "express";
import app from "../src/app";
import { ArWallet, Warp } from "warp-contracts";

class ArweaveWorld extends World {
  arweaveConnection: Arweave;
  warpConnection: Warp;
  gateway: Express | null;
  contractTxId: string | null;
  validators: { address: string; wallet: ArWallet; url: URL }[];
  bundler: { address: string; wallet: ArWallet } | null;
  response: any;
  tx: any;

  constructor(options: any) {
    super(options);
    [this.arweaveConnection, this.warpConnection] = arweave.getConnection();
    this.gateway = null;
    this.contractTxId = null;
    this.validators = [];
    this.bundler = null;
    this.response = null;
    this.tx = null;
  }

  async init() {
    let initialTokenContractState: TokenState;

    let contractTxId: string;
    let bundlersContractTxId: string;
    let tokenContractTxId: string;

    const warp: Warp = this.warpConnection;

    let accounts = await Promise.all(
      [1, 2, 3, 4].map(async (_) => {
        return this.arweaveConnection.wallets
          .generate()
          .then(async (wallet: JWKInterface) => {
            await addFunds(this.arweaveConnection, wallet);
            let address = await this.arweaveConnection.wallets.jwkToAddress(
              wallet
            );
            return {
              wallet,
              address,
            };
          });
      })
    );

    const tokenContractStateFromFile = JSON.parse(
      fs.readFileSync(
        path.join(__dirname, "../../token/tests/data/token.json"),
        "utf8"
      )
    );

    initialTokenContractState = {
      ...tokenContractStateFromFile,
      ...{
        owner: accounts[0].address,
        balances: {
          [accounts[0].address]:
            tokenContractStateFromFile.totalSupply.toString(),
        },
      },
    };

    tokenContractTxId = await deployTokenContract(
      warp,
      accounts[0].wallet,
      initialTokenContractState
    ).then((deployment) => deployment.contractTxId);

    const initialBundlersContractStateFromFile = JSON.parse(
      fs.readFileSync(
        path.join(__dirname, "../../bundlers/tests/data/bundlers.json"),
        "utf8"
      )
    );

    const initialBundlersContractState = {
      ...initialBundlersContractStateFromFile,
      withdrawDelay: 3, // NOTE: For tests, we allow withdraw after 3 blocks
      token: tokenContractTxId,
      stake: (
        BigInt(10) ** BigInt(initialTokenContractState.decimals)
      ).toString(),
    };

    bundlersContractTxId = await deployBundlersContract(
      warp,
      accounts[0].wallet,
      initialBundlersContractState
    ).then((deployment) => deployment.contractTxId);

    const stateFromFile: State = JSON.parse(
      fs.readFileSync(
        path.join(__dirname, "../../validators/tests/data/validators.json"),
        "utf8"
      )
    );

    let networkInfo = await warp.arweave.network.getInfo();

    const initialState = {
      ...stateFromFile,
      token: tokenContractTxId,
      bundlersContract: bundlersContractTxId,
      minimumStake: (
        BigInt(10) ** BigInt(initialTokenContractState.decimals)
      ).toString(),
      bundler: accounts[1].address,
      epoch: {
        seq: "0",
        tx: networkInfo.current,
        height: networkInfo.height.toString(),
      },
      epochDuration: 3,
    };

    contractTxId = await deploy(warp, accounts[1].wallet, initialState).then(
      (deployment) => deployment.contractTxId
    );

    await mineBlock(this.arweaveConnection);

    let [tokenOwner, bundler, validator1, validator2] = await Promise.all(
      accounts.map(async ({ address, wallet }) => {
        return Promise.all([
          connectTokenContract(warp, tokenContractTxId, wallet),
          connectBundlersContract(warp, bundlersContractTxId, wallet),
          connect(warp, contractTxId, wallet),
        ]).then(([token, bundlers, validators]) => {
          return { address, wallet, token, bundlers, validators };
        });
      })
    );

    this.validators.push({
      url: new URL("https://validator1.example.com"),
      ...validator1,
    });
    this.validators.push({
      url: new URL("https://validator2.example.com"),
      ...validator2,
    });
    this.bundler = bundler;
    this.contractTxId = contractTxId;

    let decimals = await tokenOwner.token
      .decimals()
      .then((decimals) => BigInt(decimals));

    this.validators.forEach(async ({ address }) => {
      await tokenOwner.token.transfer(
        address,
        BigInt(200) * BigInt(10) ** decimals
      );
      await mineBlock(this.arweaveConnection);
    });

    this.gateway = await app.create(
      this.arweaveConnection,
      this.warpConnection,
      contractTxId,
      validator1.wallet
    );
  }

  async tearDown() {}
}

setWorldConstructor(ArweaveWorld);

Before({ timeout: 10 * 1000 }, async function (_) {
  await this.init();
});

After(async function (_) {
  await this.tearDown();
});
