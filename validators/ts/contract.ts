import fs from "fs";

import {
  ArWallet,
  Contract,
  HandlerBasedContract,
  Warp,
} from "warp-contracts";
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
  constructor(_contractTxId: string, warp: Warp, private _mainnet: boolean = false) {
    super(_contractTxId, warp);
  }

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
    return this.write({
      function: "updateEpoch",
    });
  }

  async join(stake: bigint, url: URL) {
    return this.write({
      function: "join",
      stake: stake.toString(),
      url: url.toString(),
    });
  }

  async leave() {
    return this.write({
      function: "leave",
    });
  }

  async proposeSlash(proposal: SlashProposal) {
    return this.write({
      function: "proposeSlash",
      proposal,
    });
  }

  async voteSlash(tx: string, vote: "for" | "against") {
    return this.write({
      function: "voteSlash",
      tx,
      vote,
    });
  }

  write(input: any,): Promise<string | null> {
    return this._mainnet ? this.bundleInteraction(input) : this.writeInteraction(input);
  }
}

export async function deploy(
  warp: Warp,
  wallet: ArWallet,
  initialState: State,
  useBundler: boolean = false
): Promise<string> {
  let contractSrc = fs.readFileSync(
    path.join(__dirname, "../pkg/rust-contract_bg.wasm")
  );

  // deploying contract using the new SDK.
  return warp.createContract.deploy({
    wallet: wallet,
    initState: JSON.stringify(initialState),
    src: contractSrc,
    wasmSrcCodeDir: path.join(__dirname, "../src"),
    wasmGlueCode: path.join(__dirname, "../pkg/rust-contract.js"),
  }, useBundler);
}

export async function connect(
  warp: Warp,
  contractTxId: string,
  wallet: ArWallet
): Promise<ValidatorsContract> {
  let contract = new ValidatorsContractImpl(
    contractTxId,
    warp,
    warp.useWarpGwInfo // We assume that if we're using the Warp gateway then we're on mainnet
  ).setEvaluationOptions({
    internalWrites: true,
  }) as ValidatorsContract;

  return contract.connect(wallet) as ValidatorsContract;
}
