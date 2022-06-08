import express, { Request, Response } from "express";

import Arweave from "arweave";
import { ArWallet, SmartWeave } from "redstone-smartweave";

import { connect, SlashProposal, Vote } from "../../validators/tests/contract";

async function create(
  arweave: Arweave,
  smartweave: SmartWeave,
  contract: string,
  wallet: ArWallet
) {
  let contractConnection = await connect(smartweave, contract, wallet);

  const router = express.Router();

  router.get("/state", async (_: Request, res: Response) => {
    contractConnection.currentState().then((state) => {
      res.status(200).send(state);
    });
  });

  router.post("/propose", (req: Request, res: Response) => {
    try {
      // TODO: instead of validating data, pass it to the contract
      // and validate the interaction using smartweave
      const proposal = new SlashProposal(req.body);
      contractConnection.proposeSlash(proposal).then(
        (result) => {
          console.log("Result: ", result);
          res.send(`{"status": "OK"}`);
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
      // TODO: instead of validating data, pass it to the contract
      // and validate the interaction using smartweave
      if (!req.body.tx) {
        throw Error("Invalid request data, missing `tx`");
      }
      if (!req.body.vote) {
        throw Error("Invalid request data, missing `vote`");
      }
      contractConnection.voteSlash(req.body.tx, req.body.vote).then(
        () => {
          res.send(`{"status": "OK"}`);
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
