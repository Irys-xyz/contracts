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
  allowances: {
    [key: string]: {
      [key: string]: bigint;
    };
  };
}

class Balance {
  balance: bigint;
  target: string;
  ticker: string;

  constructor({
    balance,
    ticker,
    target,
  }: {
    balance: string;
    ticker: string;
    target: string;
  }) {
    this.balance = BigInt(balance);
    this.ticker = ticker;
    this.target = target;
  }
}

class Allowance {
  allowance: bigint;
  ticker: string;
  owner: string;
  spender: string;

  constructor({
    allowance,
    ticker,
    owner,
    spender,
  }: {
    allowance: string;
    ticker: string;
    owner: string;
    spender: string;
  }) {
    this.allowance = BigInt(allowance);
    this.ticker = ticker;
    this.owner = owner;
    this.spender = spender;
  }
}

interface TokenContract extends Contract<TokenState> {
  allowance(owner: string, spender: string): Promise<Allowance>;
  balanceOf(target: string): Promise<Balance>;
  currentState(): Promise<TokenState>;
  decimals(): Promise<number>;
  name(): Promise<string | null | unknown>;
  symbol(): Promise<string>;
  totalSupply(): Promise<bigint>;

  approve(spender: string, value: bigint): Promise<string | null>;
  transfer(to: string, value: bigint): Promise<string | null>;
  transferFrom(from: string, to: string, value: bigint): Promise<string | null>;
}

class TokenContractImpl
  extends HandlerBasedContract<TokenState>
  implements TokenContract
{
  async currentState() {
    return (await super.readState()).state as TokenState;
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
  async transfer(to: string, value: bigint) {
    return this.writeInteraction({
      function: "transfer",
      to,
      amount: value.toString(),
    });
  }
  async transferFrom(from: string, to: string, value: BigInt) {
    return this.writeInteraction({
      function: "transferFrom",
      from,
      to,
      amount: value.toString(),
    });
  }
  async approve(spender: string, value: BigInt) {
    return this.writeInteraction({
      function: "approve",
      spender,
      amount: value.toString(),
    });
  }
  async allowance(owner: string, spender: string) {
    const interactionResult = await this.viewState({
      function: "allowance",
      owner,
      spender,
    });
    if (interactionResult.type !== "ok") {
      throw Error(interactionResult.errorMessage);
    }
    return new Allowance(
      interactionResult.result as {
        allowance: string;
        ticker: string;
        owner: string;
        spender: string;
      }
    );
  }
}

describe("Test Token", () => {
  let contractSrc: Buffer;

  let wallets: { wallet: JWKInterface; address: string }[];

  let initialState: TokenState;

  let arweave: Arweave;
  let arlocal: ArLocal;
  let smartweave: SmartWeave;
  let connections: TokenContract[];

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

    // Create wallets, fund them and get address
    wallets = await Promise.all(
      [0, 1].map(async (_) => {
        let wallet = await arweave.wallets.generate();
        await addFunds(arweave, wallet);
        let address = await arweave.wallets.jwkToAddress(wallet);
        return {
          wallet,
          address,
        };
      })
    );

    contractSrc = fs.readFileSync(
      path.join(__dirname, "../pkg/rust-contract_bg.wasm")
    );
    const stateFromFile: TokenState = JSON.parse(
      fs.readFileSync(path.join(__dirname, "./data/token.json"), "utf8")
    );

    initialState = {
      ...stateFromFile,
      ...{
        owner: wallets[0].address,
        balances: {
          [wallets[0].address]: stateFromFile.totalSupply,
        },
      },
    };

    // deploying contract using the new SDK.
    contractTxId = await smartweave.createContract.deploy(
      {
        wallet: wallets[0].wallet,
        initState: JSON.stringify(initialState),
        src: contractSrc,
      },
      path.join(__dirname, "../src"),
      path.join(__dirname, "../pkg/rust-contract.js")
    );

    console.log(`Contract TX ID: ${contractTxId}`);
    connections = [
      new TokenContractImpl(contractTxId, smartweave).setEvaluationOptions({
        internalWrites: true,
      }) as TokenContract,
      new TokenContractImpl(contractTxId, smartweave).setEvaluationOptions({
        internalWrites: true,
      }) as TokenContract,
    ];

    connections[0].connect(wallets[0].wallet);
    connections[1].connect(wallets[1].wallet);

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
    expect(await connections[0].currentState()).toEqual(initialState);
    expect(await connections[0].name()).toEqual("Test Token");
  });

  it("should get token symbol", async () => {
    expect(await connections[0].currentState()).toEqual(initialState);

    expect(await connections[0].symbol()).toEqual("TST");
  });

  it("should get token decimals", async () => {
    expect(await connections[0].currentState()).toEqual(initialState);

    expect(await connections[0].decimals()).toEqual(10);
  });

  it("should get token total supply", async () => {
    expect(await connections[0].currentState()).toEqual(initialState);

    expect(await connections[0].totalSupply()).toEqual(
      BigInt("10000000000000000000")
    );
  });

  it("should get token balance for an address", async () => {
    expect(await connections[0].currentState()).toEqual(initialState);

    expect(
      (await connections[0].balanceOf(wallets[0].address)).balance
    ).toEqual(BigInt("10000000000000000000"));
  });

  it("should properly transfer tokens", async () => {
    // FIXME: why reading balance from balances returns a string?
    let balanceBefore = BigInt(
      (await connections[0].currentState()).balances[wallets[0].address]
    );
    let amount = BigInt("555");

    let balanceAfter = balanceBefore - amount;

    await connections[0].transfer(
      "uhE-QeYS8i4pmUtnxQyHD7dzXFNaJ9oMK-IM-QPNY6M",
      amount
    );
    await mineBlock(arweave);

    expect(
      BigInt((await connections[0].currentState()).balances[wallets[0].address])
    ).toEqual(balanceAfter);
    expect(
      BigInt(
        (await connections[0].currentState()).balances[
          "uhE-QeYS8i4pmUtnxQyHD7dzXFNaJ9oMK-IM-QPNY6M"
        ]
      )
    ).toEqual(amount);
  });

  it("should properly transfer tokens using allowance", async () => {
    let to_address = "uhE-QeYS8i4pmUtnxQyHD7dzXFNaJ9oMK-IM-QPNY6M";

    // FIXME: why reading balance from balances returns a string?
    let balancesBefore = [
      (await connections[0].balanceOf(wallets[0].address)).balance,
      (await connections[0].balanceOf(wallets[1].address)).balance,
      (await connections[0].balanceOf(to_address)).balance,
    ];
    let allowance = BigInt("555");
    let transferAmount = BigInt("111");

    let expectedBalancesAfter = [
      balancesBefore[0] - transferAmount,
      balancesBefore[1],
      balancesBefore[2] + transferAmount,
    ];

    let expectedAllowanceAfter = BigInt("555") - BigInt("111");

    await connections[0].approve(wallets[1].address, allowance);
    await mineBlock(arweave);

    expect(
      (await connections[0].allowance(wallets[0].address, wallets[1].address))
        .allowance
    ).toEqual(allowance);

    // FIXME: how to execute this transaction using different wallet?
    await connections[1].transferFrom(
      wallets[0].address,
      to_address,
      transferAmount
    );
    await mineBlock(arweave);

    expect(
      (await connections[0].balanceOf(wallets[0].address)).balance
    ).toEqual(expectedBalancesAfter[0]);

    expect((await connections[0].balanceOf(to_address)).balance).toEqual(
      expectedBalancesAfter[2]
    );

    expect(
      (await connections[1].allowance(wallets[0].address, wallets[1].address))
        .allowance
    ).toEqual(expectedAllowanceAfter);
  });
});
