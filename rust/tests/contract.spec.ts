import fs from "fs";

import ArLocal from "arlocal";
import Arweave from "arweave";
import { JWKInterface } from "arweave/node/lib/wallet";
import {
  getTag,
  Contract,
  HandlerBasedContract,
  LoggerFactory,
  SmartWeave,
  SmartWeaveNodeFactory,
  SmartWeaveTags,
} from "redstone-smartweave";
import path from "path";
import { addFunds, mineBlock } from "../utils";

jest.setTimeout(30000);

class TokenState {
  ticker: string;
  name: string | null | unknown;
  decimals: number;
  totalSupply: bigint;
  owner: string;
  balances: {
    [key: string]: bigint;
  };
}

class Balance {
  ticker: string;
  target: string;
  balance: bigint;

  constructor({
    ticker,
    target,
    balance,
  }: {
    ticker: string;
    target: string;
    balance: string;
  }) {
    this.ticker = ticker;
    this.target = target;
    this.balance = BigInt(balance);
  }
}

interface TokenContract extends Contract<TokenState> {
  balanceOf(target: string): Promise<Balance>;
  currentState(): Promise<TokenState>;
  name(): Promise<string | null | unknown>;
  symbol(): Promise<string>;
  decimals(): Promise<number>;
  totalSupply(): Promise<bigint>;
}

class TokenContractImpl
  extends HandlerBasedContract<TokenState>
  implements TokenContract
{
  async currentState() {
    return (await super.readState()).state;
  }
  async name() {
    const interactionResult = await this.viewState({
      function: "name",
    });
    if (interactionResult.type !== "ok") {
      throw Error(interactionResult.errorMessage);
    }
    return interactionResult.result as string;
  }
  async symbol() {
    const interactionResult = await this.viewState({
      function: "symbol",
    });
    if (interactionResult.type !== "ok") {
      throw Error(interactionResult.errorMessage);
    }
    return interactionResult.result as string;
  }
  async decimals() {
    const interactionResult = await this.viewState({
      function: "decimals",
    });
    if (interactionResult.type !== "ok") {
      throw Error(interactionResult.errorMessage);
    }
    return interactionResult.result as number;
  }
  async totalSupply() {
    const interactionResult = await this.viewState({
      function: "totalSupply",
    });
    if (interactionResult.type !== "ok") {
      throw Error(interactionResult.errorMessage);
    }
    return BigInt(interactionResult.result as string);
  }
  async balanceOf(target) {
    const interactionResult = await this.viewState({
      function: "balanceOf",
      target,
    });
    if (interactionResult.type !== "ok") {
      throw Error(interactionResult.errorMessage);
    }
    return new Balance(
      interactionResult.result as {
        ticker: string;
        target: string;
        balance: string;
      }
    );
  }
}

describe("Test Token", () => {
  let contractSrc: Buffer;

  let wallet: JWKInterface;
  let walletAddress: string;

  let initialState: TokenState;

  let arweave: Arweave;
  let arlocal: ArLocal;
  let smartweave: SmartWeave;
  let token: TokenContract;

  let contractTxId: string;

  beforeAll(async () => {
    // note: each tests suit (i.e. file with tests that Jest is running concurrently
    // with another files has to have ArLocal set to a different port!)
    arlocal = new ArLocal(1820, false);
    await arlocal.start();

    arweave = Arweave.init({
      host: "localhost",
      port: 1820,
      protocol: "http",
    });

    LoggerFactory.INST.logLevel("error");
    // LoggerFactory.INST.logLevel("debug", "WASM:Rust");
    // LoggerFactory.INST.logLevel("debug", "WasmContractHandlerApi");

    smartweave = SmartWeaveNodeFactory.memCached(arweave);

    wallet = await arweave.wallets.generate();
    await addFunds(arweave, wallet);
    walletAddress = await arweave.wallets.jwkToAddress(wallet);

    contractSrc = fs.readFileSync(
      path.join(__dirname, "../pkg/rust-contract_bg.wasm")
    );
    const stateFromFile: TokenState = JSON.parse(
      fs.readFileSync(path.join(__dirname, "./data/token.json"), "utf8")
    );

    initialState = {
      ...stateFromFile,
      ...{
        owner: walletAddress,
        balances: {
          [walletAddress]: stateFromFile.totalSupply,
        },
      },
    };

    // deploying contract using the new SDK.
    contractTxId = await smartweave.createContract.deploy(
      {
        wallet,
        initState: JSON.stringify(initialState),
        src: contractSrc,
      },
      path.join(__dirname, "../src"),
      path.join(__dirname, "../pkg/rust-contract.js")
    );

    console.log(`Contract TX ID: ${contractTxId}`);
    token = new TokenContractImpl(
      contractTxId,
      smartweave
    ).setEvaluationOptions({
      internalWrites: true,
    }) as TokenContract;

    token.connect(wallet);

    await mineBlock(arweave);
  });

  afterAll(async () => {
    await arlocal.stop();
  });

  it("should properly deploy contract", async () => {
    const contractTx = await arweave.transactions.get(contractTxId);

    expect(contractTx).not.toBeNull();
    expect(getTag(contractTx, SmartWeaveTags.CONTRACT_TYPE)).toEqual("wasm");
    expect(getTag(contractTx, SmartWeaveTags.WASM_LANG)).toEqual("rust");

    const contractSrcTx = await arweave.transactions.get(
      getTag(contractTx, SmartWeaveTags.CONTRACT_SRC_TX_ID)
    );
    expect(getTag(contractSrcTx, SmartWeaveTags.CONTENT_TYPE)).toEqual(
      "application/wasm"
    );
    expect(getTag(contractSrcTx, SmartWeaveTags.WASM_LANG)).toEqual("rust");
  });

  it("should get token name", async () => {
    expect(await token.currentState()).toEqual(initialState);
    expect(await token.name()).toEqual("Test Token");
  });

  it("should get token symbol", async () => {
    expect(await token.currentState()).toEqual(initialState);

    expect(await token.symbol()).toEqual("TST");
  });

  it("should get token decimals", async () => {
    expect(await token.currentState()).toEqual(initialState);

    expect(await token.decimals()).toEqual(10);
  });

  it("should get token total supply", async () => {
    expect(await token.currentState()).toEqual(initialState);

    expect(await token.totalSupply()).toEqual(BigInt("10000000000000000000"));
  });

  it("should get token balance for an address", async () => {
    expect(await token.currentState()).toEqual(initialState);

    expect((await token.balanceOf(walletAddress)).balance).toEqual(
      BigInt("10000000000000000000")
    );
  });
});
