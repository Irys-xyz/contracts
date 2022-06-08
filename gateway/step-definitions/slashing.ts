import {
  setWorldConstructor,
  After,
  Before,
  Given,
  Then,
  When,
  World,
} from "@cucumber/cucumber";

import arweave from "./arweave";
import Arweave = require("arweave");
import { ArWallet, SmartWeave } from "redstone-smartweave";

import supertest = require("supertest");
import { expect } from "chai";

import {
  connect as connectTokenContract,
  deploy as deployTokenContract,
  TokenContract,
  TokenState,
} from "../../token/tests/contract";

import {
  connect as connectBundlersContract,
  deploy as deployBundlersContract,
  BundlersContract,
  State as BundlersState,
} from "../../bundlers/tests/contract";

import {
  connect,
  deploy,
  SlashProposal,
  State,
  ValidatorsContract,
} from "../../validators/tests/contract";

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

  constructor(options: any) {
    super(options);
    [this.arweaveConnection, this.smartWeaveConnection] =
      arweave.getConnection();
    this.gateway = null;
    this.contractTxId = null;
    this.validators = [];
    this.bundler = null;
    this.response = null;
  }

  async init() {
    let initialState: State;
    let initialBundlersContractState: BundlersState;
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

    [initialBundlersContractState, bundlersContractTxId] =
      await deployBundlersContract(
        smartweave,
        tokenContractTxId,
        BigInt(10) ** BigInt(initialTokenContractState.decimals),
        accounts[0]
      );

    [initialState, contractTxId] = await deploy(
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

Given("the validator is joined", async function () {
  let stake = await this.validators[0].validators
    .minimumStake()
    .then((stake: string) => BigInt(stake));

  await this.validators[0].token.approve(this.contractTxId, stake);
  await mineBlock(this.arweaveConnection);

  await this.validators[0].validators.join(stake, this.validators[0].url);
  await mineBlock(this.arweaveConnection);

  expect(await this.validators[0].validators.validators()).to.contain(
    this.validators[0].address
  );
});

When(
  "the validator proposes slashing because of a missing transaction",
  async function () {
    this.response = await supertest(this.gateway).post("/propose").send({
      id: "tx1",
      size: 1,
      fee: "1",
      currency: "BTC",
      block: "100",
      validator: this.validators[0].address,
      signature: "this is not verified",
    });
    await mineBlock(this.arweaveConnection);
  }
);

Then("the response code is {int}", async function (responseCode) {
  expect(this.response.statusCode).to.equal(responseCode);
});

Then("the proposal is recorded in the contract state", async function () {
  let state = await this.validators[1].validators.currentState();
  expect(state.slashProposals["tx1"]).not.to.be.undefined;
});

Given("validator {int} is joined", async function (validator: number) {
  let i = validator - 1;

  let stake = await this.validators[i].validators
    .minimumStake()
    .then((stake: string) => BigInt(stake));

  await this.validators[i].token.approve(this.contractTxId, stake);
  await mineBlock(this.arweaveConnection);

  await this.validators[i].validators.join(stake, this.validators[i].url);
  await mineBlock(this.arweaveConnection);

  expect(await this.validators[i].validators.validators()).to.contain(
    this.validators[i].address
  );
});

Given(
  "validator {int} has proposed for slashing because of a missing transaction",
  async function (validator: number) {
    let i = validator - 1;

    await this.validators[i].validators.proposeSlash({
      id: "tx1",
      size: 1,
      fee: "1",
      currency: "BTC",
      block: "100",
      validator: this.validators[i].address,
      signature: "this is not verified",
    });
    await mineBlock(this.arweaveConnection);

    let state = await this.validators[i].validators.currentState();
    expect(state.slashProposals["tx1"]).not.to.be.undefined;
  }
);

When(
  "the validator votes {string} slashing",
  async function (vote: "for" | "against") {
    this.response = await supertest(this.gateway).post("/vote").send({
      tx: "tx1",
      vote: vote,
    });
    await mineBlock(this.arweaveConnection);
  }
);

Then(
  "the vote {string} is recorded in the contract state",
  async function (vote: "for" | "against") {
    let state = await this.validators[0].validators.currentState();
    try {
      expect(
        state.slashProposals["tx1"][4].Closed.votes[
          this.validators[0].address
        ][0]
      ).to.equal(vote);
    } catch (err) {
      console.error(`Validator who voted: ${this.validators[0].address}`);
      console.error(`State: ${JSON.stringify(state, null, "\t")}`);
      throw err;
    }
  }
);

When("posting invalid proposal data", async function () {
  this.response = await supertest(this.gateway).post("/propose").send({
    foo: "bar",
  });
});

When("posting invalid voting data", async function () {
  this.response = await supertest(this.gateway).post("/vote").send({
    foo: "bar",
  });
});

When("requesting contract state", async function () {
  this.response = await supertest(this.gateway).get("/state");
});

Then(
  "response body is valid JSON for validators contract state",
  async function () {
    let currentState = await this.validators[0].validators.currentState();
    let receivedState = new State(this.response.body);

    expect(receivedState).to.deep.equal(currentState);
  }
);
