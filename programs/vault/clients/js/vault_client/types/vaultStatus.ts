/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  combineCodec,
  getEnumDecoder,
  getEnumEncoder,
  type Codec,
  type Decoder,
  type Encoder,
} from '@solana/web3.js';

/**
 * An enum representing the status of a Vault.
 * After deposit, it becomes Active, and must be Inactive to withdraw.
 */

export enum VaultStatus {
  Active,
  Deactivating,
  Inactive,
}

export type VaultStatusArgs = VaultStatus;

export function getVaultStatusEncoder(): Encoder<VaultStatusArgs> {
  return getEnumEncoder(VaultStatus);
}

export function getVaultStatusDecoder(): Decoder<VaultStatus> {
  return getEnumDecoder(VaultStatus);
}

export function getVaultStatusCodec(): Codec<VaultStatusArgs, VaultStatus> {
  return combineCodec(getVaultStatusEncoder(), getVaultStatusDecoder());
}
