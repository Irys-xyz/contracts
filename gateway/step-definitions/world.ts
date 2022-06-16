import { setWorldConstructor, After, Before, World } from "@cucumber/cucumber";

import arweave from "./arweave";
import Arweave = require("arweave");
import { ArWallet, SmartWeave } from "redstone-smartweave";

import {
  connect as connectTokenContract,
  deploy as deployTokenContract,
  TokenState,
} from "../../token/tests/contract";

import {
  connect as connectBundlersContract,
  deploy as deployBundlersContract,
} from "../../bundlers/tests/contract";

import { connect, deploy, State } from "../../validators/tests/contract";

import { addFunds, mineBlock } from "./utils";
import { JWKInterface } from "arweave/node/lib/wallet";

import { Express } from "express";
import app from "../src/app";

class ArweaveWorld extends World {
  arweaveConnection: Arweave;
  smartWeaveConnection: SmartWeave;
  gateway: Express | null;
  contractTxId: string | null;
  validators: { address: string; wallet: ArWallet; url: URL }[];
  bundler: { address: string; wallet: ArWallet } | null;
  response: any;
  tx: any;

  constructor(options: any) {
    super(options);
    [this.arweaveConnection, this.smartWeaveConnection] =
      arweave.getConnection();
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

    const smartweave: SmartWeave = this.smartWeaveConnection;

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

    [initialTokenContractState, tokenContractTxId] = await deployTokenContract(
      smartweave,
      accounts[0]
    );

    [, bundlersContractTxId] = await deployBundlersContract(
      smartweave,
      tokenContractTxId,
      BigInt(10) ** BigInt(initialTokenContractState.decimals),
      accounts[0]
    );

    [, contractTxId] = await deploy(
      smartweave,
      tokenContractTxId,
      bundlersContractTxId,
      BigInt(10) ** BigInt(initialTokenContractState.decimals),
      accounts[1]
    );

    await mineBlock(this.arweaveConnection);

    let [tokenOwner, bundler, validator1, validator2] = await Promise.all(
      accounts.map(async ({ address, wallet }) => {
        return Promise.all([
          connectTokenContract(smartweave, tokenContractTxId, wallet),
          connectBundlersContract(smartweave, bundlersContractTxId, wallet),
          connect(smartweave, contractTxId, wallet),
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
      this.smartWeaveConnection,
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
