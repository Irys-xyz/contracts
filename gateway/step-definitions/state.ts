import { Then, When } from "@cucumber/cucumber";

import { State } from "../../validators/tests/contract";

import supertest = require("supertest");
import { expect } from "chai";

When("requesting contract state", async function () {
  this.response = await supertest(this.gateway).get("/validators/state");
});

When("requesting contract state one block earlier", async function () {
  let networkInfo = await this.arweaveConnection.network.getInfo();
  let height = networkInfo.height - 1;
  this.response = await supertest(this.gateway).get(
    `/validators/state?height=${height}`
  );
});

Then(
  "response body is valid JSON for validators contract state",
  async function () {
    let currentState = await this.validators[0].validators.currentState();
    let receivedState = new State(this.response.body);

    expect(receivedState).to.deep.equal(currentState);
  }
);

Then(
  "validator {int} is listed as a validator",
  async function (validator: number) {
    let receivedState = new State(this.response.body);

    expect(receivedState.validators).to.have.property(
      this.validators[validator - 1].address
    );
  }
);

Then(
  "validator {int} is not listed as a validator",
  async function (validator: number) {
    let receivedState = new State(this.response.body);
    console.log(receivedState);
    expect(receivedState.validators).to.not.have.property(
      this.validators[validator - 1].address
    );
  }
);
