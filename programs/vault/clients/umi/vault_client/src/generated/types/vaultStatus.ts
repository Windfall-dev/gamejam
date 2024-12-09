/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import { Serializer, scalarEnum } from '@metaplex-foundation/umi/serializers';

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

export function getVaultStatusSerializer(): Serializer<
  VaultStatusArgs,
  VaultStatus
> {
  return scalarEnum<VaultStatus>(VaultStatus, {
    description: 'VaultStatus',
  }) as Serializer<VaultStatusArgs, VaultStatus>;
}
