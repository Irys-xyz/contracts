import express, { Request, Response } from "express";

import Arweave from "arweave";

import { connect, SlashProposal, Vote } from "../../validators/ts/contract";
import { ArWallet, Warp } from "warp-contracts";

async function create(
  arweave: Arweave,
  warp: Warp,
  contract: string,
  wallet: ArWallet
) {
  let contractConnection = await connect(warp, contract, wallet);

  if (process.env.GW_STATE_ENDPOINT) await contractConnection.syncState(process.env.GW_STATE_ENDPOINT);

  const router = express.Router();

  router.get("/state", async (req: Request, res: Response) => {
    if (req.query.height) {
      return contractConnection
        .currentState(Number(req.query.height))
        .then((state) => {
          res.status(200).send(state);
        });
    } else {
      return contractConnection.currentState().then((state) => {
        res.status(200).send(state);
      });
    }
  });

  router.post("/update-epoch", (_: Request, res: Response) => {
    try {
      contractConnection.updateEpoch().then(
        (result) => {
          res.send({ status: "ok", tx: result });
        },
        (err) => {
          console.error("Failed to update epoch:", err);
          res.status(500).send("Operation failed, check logs");
        }
      );
    } catch (err: any) {
      res.status(400).send({ status: "error", msg: err.toString() });
    }
  });

  router.post("/propose", (req: Request, res: Response) => {
    try {
      // TODO: instead of validating data, pass it to the contract
      // and validate the interaction using smartweave
      const proposal = new SlashProposal(req.body);
      contractConnection.proposeSlash(proposal).then(
        (result) => {
          console.log("Result: ", result);
          res.send({ status: "ok" });
        },
        (err) => {
          console.error("Failed to propose slashing:", err);
          res.status(500).send("Operation failed, check logs");
        }
      );
    } catch (err: any) {
      res.status(400).send({ status: "error", msg: err.toString() });
    }
  });

  router.post("/vote", (req: Request, res: Response) => {
    try {
      // TODO: instead of validating data, pass it to the contraict
      // and validate the interaction using smartweave
      if (!req.body.tx) {
        throw Error("Invalid request data, missing `tx`");
      }
      if (!req.body.vote) {
        throw Error("Invalid request data, missing `vote`");
      }
      contractConnection.voteSlash(req.body.tx, req.body.vote).then(
        () => {
          res.send({ status: "ok" });
        },
        (err) => {
          console.error("Failed to vote for slashing:", err);
          res.status(500).send("Operation failed, check logs");
        }
      );
    } catch (err: any) {
      res.status(400).send({ status: "error", msg: err.toString() });
    }
  });

  return router;
}

export default {
  create,
};
