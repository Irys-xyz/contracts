import fs from "fs";

import {
  ArWallet,
  Contract,
  HandlerBasedContract,
  SmartWeave,
} from "redstone-smartweave";
import path from "path";

export class State {
  bundler: string;
  bundlersContract: string;
  epoch: { seq: string; tx: string; height: string };
  epochDuration: number;
  minimumStake: string;
  token: string;
  validators: { [key: string]: string };
  nominatedValidators: string[];
  slashProposals: {
    [key: string]: [SlashProposal, string, string, string, any]; // TODO: model voting data correctly
  };

  constructor(data: {
    bundler: string;
    bundlersContract: string;
    epoch: { seq: string; tx: string; height: string };
    epochDuration: number;
    minimumStake: string;
    token: string;
    validators: { [key: string]: string };
    nominatedValidators: string[];
    slashProposals: {
      [key: string]: [SlashProposal, string, string, string, any]; // TODO: model voting data correctly
    };
  }) {
    if (!data.bundler) {
      throw Error("Invalid data, 'bundler' not defined");
    }
    this.bundler = data.bundler;

    if (!data.bundlersContract) {
      throw Error("Invalid data, 'bundlersContract' not defined");
    }
    this.bundlersContract = data.bundlersContract;

    if (!data.epoch) {
      throw Error("Invalid data, 'epoch' not defined");
    }
    this.epoch = data.epoch;

    if (!data.epochDuration) {
      throw Error("Invalid data, 'epochDuration' not defined");
    }
    this.epochDuration = data.epochDuration;

    if (!data.minimumStake) {
      throw Error("Invalid data, 'minimumStake' not defined");
    }
    this.minimumStake = data.minimumStake;

    if (!data.token) {
      throw Error("Invalid data, 'token' not defined");
    }
    this.token = data.token;

    if (!data.validators) {
      throw Error("Invalid data, 'validators' not defined");
    }
    this.validators = data.validators;

    if (!data.nominatedValidators) {
      throw Error("Invalid data, 'nominatedValidators' not defined");
    }
    this.nominatedValidators = data.nominatedValidators;

    if (!data.slashProposals) {
      throw Error("Invalid data, 'slashProposals' not defined");
    }
    this.slashProposals = data.slashProposals;
  }
}

export type Vote = "for" | "against";

export class SlashProposal {
  id: string;
  size: number;
  fee: string;
  currency: string;
  block: string;
  validator: string;
  signature: string;

  constructor(data: {
    id: string;
    size: number;
    fee: string;
    currency: string;
    block: string;
    validator: string;
    signature: string;
  }) {
    if (!data.id) {
      throw Error("Invalid data, 'id' not defined");
    }
    this.id = data.id;

    if (!data.size) {
      throw Error("Invalid data, 'size' not defined");
    }
    this.size = data.size;

    if (!data.fee) {
      throw Error("Invalid data, 'fee' not defined");
    }
    this.fee = data.fee;

    if (!data.currency) {
      throw Error("Invalid data, 'currency' not defined");
    }
    this.currency = data.currency;

    if (!data.block) {
      throw Error("Invalid data, 'block' not defined");
    }
    this.block = data.block;

    if (!data.validator) {
      throw Error("Invalid data, 'validator' not defined");
    }
    this.validator = data.validator;

    if (!data.signature) {
      throw Error("Invalid data, 'signature' not defined");
    }
    this.signature = data.signature;
  }
}

export interface ValidatorsContract extends Contract<State> {
  currentState(height?: number): Promise<State>;
  validators(): Promise<string[]>;
  nominatedValidators(): Promise<string[]>;
  bundler(): Promise<string>;
  bundlersContract(): Promise<string>;
  minimumStake(): Promise<bigint>;
  token(): Promise<string>;
  epoch(): Promise<{ seq: string; tx: string; height: string }>;
  epochDuration(): Promise<number>;
  updateEpoch(): Promise<string | null>;
  join(stake: bigint, url: URL): Promise<string | null>;
  leave(): Promise<string | null>;
  proposeSlash(proposal: SlashProposal): Promise<string | null>;
  voteSlash(tx: string, vote: "for" | "against"): Promise<string | null>;
}

class ValidatorsContractImpl
  extends HandlerBasedContract<State>
  implements ValidatorsContract
{
  async currentState(height?: number) {
    let state = await super.readState(height).then((res) => res.state);
    return new State(state);
  }
  async bundler() {
    const interactionResult = await this.viewState({
      function: "bundler",
    });
    if (interactionResult.type !== "ok") {
      throw Error(interactionResult.errorMessage);
    }
    return interactionResult.result as string;
  }
  async bundlersContract() {
    const interactionResult = await this.viewState({
      function: "bundlerContract",
    });
    if (interactionResult.type !== "ok") {
      throw Error(interactionResult.errorMessage);
    }
    return interactionResult.result as string;
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
  async minimumStake() {
    const interactionResult = await this.viewState({
      function: "minimumStake",
    });
    if (interactionResult.type !== "ok") {
      throw Error(interactionResult.errorMessage);
    }
    return BigInt(interactionResult.result as string);
  }
  async epoch() {
    const interactionResult = await this.viewState({
      function: "epoch",
    });
    if (interactionResult.type !== "ok") {
      throw Error(interactionResult.errorMessage);
    }

    return interactionResult.result as {
      seq: string;
      tx: string;
      height: string;
    };
  }
  async epochDuration() {
    const interactionResult = await this.viewState({
      function: "epochDuration",
    });
    if (interactionResult.type !== "ok") {
      throw Error(interactionResult.errorMessage);
    }
    return interactionResult.result as number;
  }
  async validators() {
    const interactionResult = await this.viewState({
      function: "validators",
    });
    if (interactionResult.type !== "ok") {
      throw Error(interactionResult.errorMessage);
    }
    return interactionResult.result as string[];
  }
  async nominatedValidators() {
    const interactionResult = await this.viewState({
      function: "nominatedValidators",
    });
    if (interactionResult.type !== "ok") {
      throw Error(interactionResult.errorMessage);
    }
    return interactionResult.result as string[];
  }
  async updateEpoch() {
    return this.writeInteraction({
      function: "updateEpoch",
    });
  }
  async join(stake: bigint, url: URL) {
    return this.writeInteraction({
      function: "join",
      stake: stake.toString(),
      url: url.toString(),
    });
  }
  async leave() {
    return this.writeInteraction({
      function: "leave",
    });
  }
  async proposeSlash(proposal: SlashProposal) {
    return this.writeInteraction({
      function: "proposeSlash",
      proposal,
    });
  }
  async voteSlash(tx: string, vote: "for" | "against") {
    return this.writeInteraction({
      function: "voteSlash",
      tx,
      vote,
    });
  }
}

export async function deploy(
  smartweave: SmartWeave,
  token: string,
  bundlersContract: string,
  stake: bigint,
  bundler: { address: string; wallet: ArWallet }
): Promise<[State, string]> {
  let contractSrc = fs.readFileSync(
    path.join(__dirname, "../pkg/rust-contract_bg.wasm")
  );
  const stateFromFile: State = JSON.parse(
    fs.readFileSync(path.join(__dirname, "./data/validators.json"), "utf8")
  );

  let networkInfo = await smartweave.arweave.network.getInfo();

  let initialState = {
    ...stateFromFile,
    token,
    bundlersContract,
    minimumStake: stake.toString(),
    bundler: bundler.address,
    epoch: {
      seq: "0",
      tx: networkInfo.current,
      height: networkInfo.height.toString(),
    },
    epochDuration: 3,
  };

  // deploying contract using the new SDK.
  return smartweave.createContract
    .deploy({
      wallet: bundler.wallet,
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
  wallet: ArWallet
): Promise<ValidatorsContract> {
  let contract = new ValidatorsContractImpl(
    contractTxId,
    smartweave
  ).setEvaluationOptions({
    internalWrites: true,
  }) as ValidatorsContract;

  return contract.connect(wallet) as ValidatorsContract;
}
