import express, { Express, Request, Response } from "express";

import Arweave from "arweave";
import { ArWallet, SmartWeave } from "redstone-smartweave";

import validators from "./validators";
import transactions from "./transactions";

async function create(
  arweave: Arweave,
  smartweave: SmartWeave,
  contract: string,
  wallet: ArWallet
) {
  const app: Express = express();

  app.use(express.json());
  app.use(
    "/validators",
    await validators.create(arweave, smartweave, contract, wallet)
  );
  app.use(
    "/tx",
    await transactions.create(arweave, smartweave, contract, wallet)
  );

  return app;
}

export default {
  create,
};
