/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.35.3.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { Coin, StdFee } from "@cosmjs/amino";
import { TokenModeMsgForCw20ModeBridgedAndCw20ModeCollateral, Uint128, Logo, EmbeddedLogo, Binary, InstantiateMsg, Cw20ModeBridged, InstantiateMsg1, Cw20Coin, InstantiateMarketingInfo, MinterResponse, Cw20ModeCollateral, ExecuteMsg, OwnableMsg, RouterMsgForHexBinary, HexBinary, DomainRouteSetForHexBinary, HandleMsg, Cw20ReceiveMsg, QueryMsg, OwnableQueryMsg, RouterQueryForHexBinary, Order, TokenWarpDefaultQueryMsg, DomainsResponse, Addr, OwnerResponse, PendingOwnerResponse, RouteResponseForHexBinary, RoutesResponseForHexBinary, Empty, TokenMode, TokenModeResponse, TokenType, TokenTypeNative, TokenTypeResponse } from "./WarpCw20.types";
export interface WarpCw20ReadOnlyInterface {
  contractAddress: string;
  ownable: (ownableQueryMsg: OwnableQueryMsg) => Promise<OwnableResponse>;
  router: (routerQueryForHexBinary: RouterQueryForHexBinary) => Promise<RouterResponse>;
  tokenDefault: (tokenWarpDefaultQueryMsg: TokenWarpDefaultQueryMsg) => Promise<TokenDefaultResponse>;
}
export class WarpCw20QueryClient implements WarpCw20ReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.ownable = this.ownable.bind(this);
    this.router = this.router.bind(this);
    this.tokenDefault = this.tokenDefault.bind(this);
  }

  ownable = async (ownableQueryMsg: OwnableQueryMsg): Promise<OwnableResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      ownable: ownableQueryMsg
    });
  };
  router = async (routerQueryForHexBinary: RouterQueryForHexBinary): Promise<RouterResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      router: routerQueryForHexBinary
    });
  };
  tokenDefault = async (tokenWarpDefaultQueryMsg: TokenWarpDefaultQueryMsg): Promise<TokenDefaultResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      token_default: tokenWarpDefaultQueryMsg
    });
  };
}
export interface WarpCw20Interface extends WarpCw20ReadOnlyInterface {
  contractAddress: string;
  sender: string;
  ownable: (ownableMsg: OwnableMsg, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  router: (routerMsgForHexBinary: RouterMsgForHexBinary, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  handle: ({
    body,
    origin,
    sender
  }: {
    body: HexBinary;
    origin: number;
    sender: HexBinary;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  receive: ({
    amount,
    msg,
    sender
  }: {
    amount: Uint128;
    msg: Binary;
    sender: string;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
}
export class WarpCw20Client extends WarpCw20QueryClient implements WarpCw20Interface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.ownable = this.ownable.bind(this);
    this.router = this.router.bind(this);
    this.handle = this.handle.bind(this);
    this.receive = this.receive.bind(this);
  }

  ownable = async (ownableMsg: OwnableMsg, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      ownable: ownableMsg
    }, fee, memo, _funds);
  };
  router = async (routerMsgForHexBinary: RouterMsgForHexBinary, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      router: routerMsgForHexBinary
    }, fee, memo, _funds);
  };
  handle = async ({
    body,
    origin,
    sender
  }: {
    body: HexBinary;
    origin: number;
    sender: HexBinary;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      handle: {
        body,
        origin,
        sender
      }
    }, fee, memo, _funds);
  };
  receive = async ({
    amount,
    msg,
    sender
  }: {
    amount: Uint128;
    msg: Binary;
    sender: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      receive: {
        amount,
        msg,
        sender
      }
    }, fee, memo, _funds);
  };
}