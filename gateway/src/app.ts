import express, { Express, Request, Response } from "express";

import Arweave from "arweave";

import validators from "./validators";
import transactions from "./transactions";

import { ArWallet, Warp } from "warp-contracts";

async function create(
  arweave: Arweave,
  warp: Warp,
  contract: string,
  wallet: ArWallet
) {
  const app: Express = express();

  app.use(express.json());
  app.use(
    "/validators",
    await validators.create(arweave, warp, contract, wallet)
  );
  app.use(
    "/tx",
    await transactions.create(arweave, warp, contract, wallet)
  );

  return app;
}

export default {
  create,
};
