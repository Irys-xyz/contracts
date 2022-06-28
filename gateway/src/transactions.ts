import express, { Request, Response } from "express";

import Arweave from "arweave";
import { ArweaveErrorType } from "arweave/node/lib/error";
import { ArWallet, Warp } from "warp-contracts";

async function create(
  arweave: Arweave,
  warp: Warp,
  contract: string,
  wallet: ArWallet
) {
  const router = express.Router();

  router.get("/:id", async (req: Request, res: Response) => {
    arweave.transactions
      .get(req.params.id)
      .then((tx) => {
        res.status(200).send(tx);
      })
      .catch((err) => {
        switch (err.getType()) {
          case ArweaveErrorType.TX_NOT_FOUND:
            break;
          case ArweaveErrorType.TX_FAILED:
            break;
          case ArweaveErrorType.TX_INVALID:
            break;
          default:
            console.error("Request failed: ", err);
            res.status(500).send("Unexpected failure, check server logs");
        }
      });
  });

  router.get("/:id/status", async (req: Request, res: Response) => {
    return arweave.transactions.getStatus(req.params.id).then((status) => {
      res.status(200).send(status);
    });
  });

  return router;
}

export default {
  create,
};
