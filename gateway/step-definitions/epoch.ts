import { Then, When } from "@cucumber/cucumber";

import supertest = require("supertest");
import { expect } from "chai";

import { mineBlock } from "./utils";

When("requesting to update epoch", async function () {
  this.response = await supertest(this.gateway).post(
    "/validators/update-epoch"
  );
  await mineBlock(this.arweaveConnection);
});

Then("the response data contains valid tx", function () {
  expect(this.response.body.tx).not.to.be.undefined;
  expect(this.response.body.tx).not.to.be.null;
  expect(this.response.body.tx).not.to.be.string;
});
