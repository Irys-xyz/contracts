import express, { Express, Request, Response } from "express";

import Arweave from "arweave";
import { ArWallet, SmartWeave } from "redstone-smartweave";

import router from "./validators";

async function create(
  arweave: Arweave,
  smartweave: SmartWeave,
  contract: string,
  wallet: ArWallet
) {
  const app: Express = express();

  app.use(express.json());
  app.use("/", await router.create(arweave, smartweave, contract, wallet));

  return app;
}

export default {
  create,
};
