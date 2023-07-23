import { readdirSync } from "fs";
import { Context, Contract, ContractConstructor } from "../types";
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { Container } from "inversify";

const contractNames: string[] = readdirSync(__dirname)
  .filter((f) => f !== "index.ts")
  .map((f) => f.replace(".ts", ""));

const contractHandlers: { [key: string]: ContractConstructor } = contractNames
  .reduce((acc, cur) =>{
    acc[cur] = require(`./${cur}`).default
    return acc;
  }, {} as { [key: string]: ContractConstructor});


export function getTargetContractName(): string[] {
  return contractNames;
}

export function getTargetContract(ctx: Context, client: SigningCosmWasmClient, signer: string, container: Container): { [key: string]: Contract } {
  return Object.keys(contractHandlers).reduce((acc, cur) => {
    const { codeId, digest, address } = ctx.contracts[cur] || {};
    acc[cur] = new contractHandlers[cur](address, codeId, digest, signer, client);

    container.bind(contractHandlers[cur]).toConstantValue(acc[cur]);
    return acc;
  }, {} as { [key: string]: Contract });
}

