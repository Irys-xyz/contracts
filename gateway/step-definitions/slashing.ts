import { Given, Then, When } from "@cucumber/cucumber";

import supertest from "supertest";
import { expect } from "chai";

import { mineBlock } from "./utils";

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
    this.response = await supertest(this.gateway)
      .post("/validators/propose")
      .send({
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

  console.debug(await this.validators[i].validators.validators());

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
    this.response = await supertest(this.gateway)
      .post("/validators/vote")
      .send({
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
  this.response = await supertest(this.gateway)
    .post("/validators/propose")
    .send({
      foo: "bar",
    });
  await mineBlock(this.arweaveConnection);
});

When("posting invalid voting data", async function () {
  this.response = await supertest(this.gateway).post("/validators/vote").send({
    foo: "bar",
  });
  await mineBlock(this.arweaveConnection);
});

Given("the validator has proposed slashing", async function () {
  this.tx = await this.validators[0].validators.proposeSlash({
    id: "tx1",
    size: 1,
    fee: "1",
    currency: "BTC",
    block: "100",
    validator: this.validators[0].address,
    signature: "this is not verified",
  });

  expect(this.tx).to.be.string;
  expect(this.tx).not.to.be.null;

  await mineBlock(this.arweaveConnection);

  let state = await this.validators[0].validators.currentState();
  expect(state.slashProposals["tx1"]).not.to.be.undefined;
});

When("requesting status of the tx", async function () {
  this.response = await supertest(this.gateway).get(`/tx/${this.tx}/status`);
});

Then("the response body defines {string} field", function (fieldName: string) {
  let obj = this.response.body;
  fieldName.split(".").forEach((f) => {
    try {
      expect(obj).to.have.property(f);
      obj = obj[f];
    } catch (err) {
      console.error(`No "${f}" in ${JSON.stringify(obj)}`);
      throw Error(`No "${fieldName}" in ${JSON.stringify(this.response.body)}`);
    }
  });
});
