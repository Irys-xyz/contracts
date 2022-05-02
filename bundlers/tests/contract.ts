import fs from "fs";

import Arweave from "arweave";
import { JWKInterface } from "arweave/node/lib/wallet";
import {
  Contract,
  HandlerBasedContract,
  SmartWeave,
} from "redstone-smartweave";
import path from "path";

export class State {
  token: string;
  bundlers: Set<string>;
}

export interface BundlersContract extends Contract<State> {
  currentState(): Promise<State>;
  bundlers(): Promise<{ [key: string]: bigint }>;
  withdrawDelay(): Promise<number>;
  stake(): Promise<bigint>;
  token(): Promise<string>;
  join(): Promise<string | null>;
  leave(): Promise<string | null>;
  withdraw(): Promise<string | null>;
  syncSlash(): Promise<string | null>;
}

class BundlersContractImpl
  extends HandlerBasedContract<State>
  implements BundlersContract
{
  async currentState() {
    return (await super.readState()).state as State;
  }
  async token() {
    const interactionResult = await this.viewState({
      function: "token",
    });
    if (interactionResult.type !== "ok") {
      throw Error(interactionResult.errorMessage);
    }
    return interactionResult.result as string;
  }
  async stake() {
    const interactionResult = await this.viewState({
      function: "stake",
    });
    if (interactionResult.type !== "ok") {
      throw Error(interactionResult.errorMessage);
    }
    return interactionResult.result as bigint;
  }
  async bundlers() {
    const interactionResult = await this.viewState({
      function: "bundlers",
    });
    if (interactionResult.type !== "ok") {
      throw Error(interactionResult.errorMessage);
    }
    return interactionResult.result as { [key: string]: bigint };
  }
  async withdrawDelay() {
    const interactionResult = await this.viewState({
      function: "withdrawDelay",
    });
    if (interactionResult.type !== "ok") {
      throw Error(interactionResult.errorMessage);
    }
    return interactionResult.result as number;
  }
  async join() {
    return this.writeInteraction({
      function: "join",
    });
  }
  async leave() {
    return this.writeInteraction({
      function: "leave",
    });
  }
  async withdraw() {
    return this.writeInteraction({
      function: "withdraw",
    });
  }
  async syncSlash() {
    return this.writeInteraction({
      function: "syncSlash",
    });
  }
}

export async function deploy(
  smartweave: SmartWeave,
  token: string,
  stake: bigint,
  owner: { wallet: JWKInterface }
): Promise<[State, string]> {
  let contractSrc = fs.readFileSync(
    path.join(__dirname, "../pkg/rust-contract_bg.wasm")
  );
  const stateFromFile: State = JSON.parse(
    fs.readFileSync(path.join(__dirname, "./data/bundlers.json"), "utf8")
  );

  let initialState = {
    ...stateFromFile,
    token,
    stake: stake.toString(),
  };

  // deploying contract using the new SDK.
  return smartweave.createContract
    .deploy({
      wallet: owner.wallet,
      initState: JSON.stringify(initialState),
      src: contractSrc,
      wasmSrcCodeDir: path.join(__dirname, "../src"),
      wasmGlueCode: path.join(__dirname, "../pkg/rust-contract.js"),
    })
    .then((txId) => [initialState, txId]);
}

export async function connect(
  smartweave: SmartWeave,
  contractTxId: string,
  wallet: JWKInterface
): Promise<BundlersContract> {
  let contract = new BundlersContractImpl(
    contractTxId,
    smartweave
  ).setEvaluationOptions({
    internalWrites: true,
  }) as BundlersContract;

  return contract.connect(wallet) as BundlersContract;
}
