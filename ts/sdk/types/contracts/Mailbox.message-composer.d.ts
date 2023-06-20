/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/
import { Coin } from "@cosmjs/amino";
import { MsgExecuteContractEncodeObject } from "cosmwasm";
import { HexBinary } from "./Mailbox.types";
export interface MailboxMessage {
    contractAddress: string;
    sender: string;
    pause: (funds?: Coin[]) => MsgExecuteContractEncodeObject;
    unpause: (funds?: Coin[]) => MsgExecuteContractEncodeObject;
    setDefaultISM: ({ ism }: {
        ism: string;
    }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
    dispatch: ({ destDomain, msgBody, recipientAddr }: {
        destDomain: number;
        msgBody: HexBinary;
        recipientAddr: HexBinary;
    }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
    process: ({ message, metadata }: {
        message: HexBinary;
        metadata: HexBinary;
    }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
}
export declare class MailboxMessageComposer implements MailboxMessage {
    sender: string;
    contractAddress: string;
    constructor(sender: string, contractAddress: string);
    pause: (funds?: Coin[]) => MsgExecuteContractEncodeObject;
    unpause: (funds?: Coin[]) => MsgExecuteContractEncodeObject;
    setDefaultISM: ({ ism }: {
        ism: string;
    }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
    dispatch: ({ destDomain, msgBody, recipientAddr }: {
        destDomain: number;
        msgBody: HexBinary;
        recipientAddr: HexBinary;
    }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
    process: ({ message, metadata }: {
        message: HexBinary;
        metadata: HexBinary;
    }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
}
//# sourceMappingURL=Mailbox.message-composer.d.ts.map