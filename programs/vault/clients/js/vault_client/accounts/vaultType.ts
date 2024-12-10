/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  assertAccountExists,
  assertAccountsExist,
  combineCodec,
  decodeAccount,
  fetchEncodedAccount,
  fetchEncodedAccounts,
  fixDecoderSize,
  fixEncoderSize,
  getAddressDecoder,
  getAddressEncoder,
  getBooleanDecoder,
  getBooleanEncoder,
  getBytesDecoder,
  getBytesEncoder,
  getI64Decoder,
  getI64Encoder,
  getStructDecoder,
  getStructEncoder,
  getU64Decoder,
  getU64Encoder,
  getU8Decoder,
  getU8Encoder,
  transformEncoder,
  type Account,
  type Address,
  type Codec,
  type Decoder,
  type EncodedAccount,
  type Encoder,
  type FetchAccountConfig,
  type FetchAccountsConfig,
  type MaybeAccount,
  type MaybeEncodedAccount,
  type ReadonlyUint8Array,
} from '@solana/web3.js';

export const VAULT_TYPE_DISCRIMINATOR = new Uint8Array([
  251, 71, 249, 103, 117, 71, 62, 101,
]);

export function getVaultTypeDiscriminatorBytes() {
  return fixEncoderSize(getBytesEncoder(), 8).encode(VAULT_TYPE_DISCRIMINATOR);
}

export type VaultType = {
  discriminator: ReadonlyUint8Array;
  /** The pubkey of the authority (usually the owner). */
  authority: Address;
  /** The pubkey of the token mint to be deposited to vaults. */
  mint: Address;
  /** The pubkey of the pool token account where deposited tokens are collected. */
  pool: Address;
  /** The pubkey of the token program (spl_token or spl_token_2022). */
  tokenProgram: Address;
  /** The start timestamp of the current season. */
  seasonStart: bigint;
  /** The duration of each season in seconds. */
  seasonDuration: bigint;
  /** The duration of the cooldown period at the end of each season, in seconds. */
  cooldownWindow: bigint;
  /** The maximum amount of tokens that can be deposited to each vault. No limit if 0. */
  maxDepositPerUser: bigint;
  /** The total amount of tokens deposited across all vaults belonging to this vault type. */
  totalDeposit: bigint;
  /**
   * If true, users can instantly deactivate their vaults to Inactive state.
   * Otherwise, vaults enter Deactivating state and can transition to Inactive at the start of next season.
   */
  instantDeactivation: boolean;
  /** The bump seed of this pda. */
  bump: number;
};

export type VaultTypeArgs = {
  /** The pubkey of the authority (usually the owner). */
  authority: Address;
  /** The pubkey of the token mint to be deposited to vaults. */
  mint: Address;
  /** The pubkey of the pool token account where deposited tokens are collected. */
  pool: Address;
  /** The pubkey of the token program (spl_token or spl_token_2022). */
  tokenProgram: Address;
  /** The start timestamp of the current season. */
  seasonStart: number | bigint;
  /** The duration of each season in seconds. */
  seasonDuration: number | bigint;
  /** The duration of the cooldown period at the end of each season, in seconds. */
  cooldownWindow: number | bigint;
  /** The maximum amount of tokens that can be deposited to each vault. No limit if 0. */
  maxDepositPerUser: number | bigint;
  /** The total amount of tokens deposited across all vaults belonging to this vault type. */
  totalDeposit: number | bigint;
  /**
   * If true, users can instantly deactivate their vaults to Inactive state.
   * Otherwise, vaults enter Deactivating state and can transition to Inactive at the start of next season.
   */
  instantDeactivation: boolean;
  /** The bump seed of this pda. */
  bump: number;
};

export function getVaultTypeEncoder(): Encoder<VaultTypeArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', fixEncoderSize(getBytesEncoder(), 8)],
      ['authority', getAddressEncoder()],
      ['mint', getAddressEncoder()],
      ['pool', getAddressEncoder()],
      ['tokenProgram', getAddressEncoder()],
      ['seasonStart', getI64Encoder()],
      ['seasonDuration', getI64Encoder()],
      ['cooldownWindow', getI64Encoder()],
      ['maxDepositPerUser', getU64Encoder()],
      ['totalDeposit', getU64Encoder()],
      ['instantDeactivation', getBooleanEncoder()],
      ['bump', getU8Encoder()],
    ]),
    (value) => ({ ...value, discriminator: VAULT_TYPE_DISCRIMINATOR })
  );
}

export function getVaultTypeDecoder(): Decoder<VaultType> {
  return getStructDecoder([
    ['discriminator', fixDecoderSize(getBytesDecoder(), 8)],
    ['authority', getAddressDecoder()],
    ['mint', getAddressDecoder()],
    ['pool', getAddressDecoder()],
    ['tokenProgram', getAddressDecoder()],
    ['seasonStart', getI64Decoder()],
    ['seasonDuration', getI64Decoder()],
    ['cooldownWindow', getI64Decoder()],
    ['maxDepositPerUser', getU64Decoder()],
    ['totalDeposit', getU64Decoder()],
    ['instantDeactivation', getBooleanDecoder()],
    ['bump', getU8Decoder()],
  ]);
}

export function getVaultTypeCodec(): Codec<VaultTypeArgs, VaultType> {
  return combineCodec(getVaultTypeEncoder(), getVaultTypeDecoder());
}

export function decodeVaultType<TAddress extends string = string>(
  encodedAccount: EncodedAccount<TAddress>
): Account<VaultType, TAddress>;
export function decodeVaultType<TAddress extends string = string>(
  encodedAccount: MaybeEncodedAccount<TAddress>
): MaybeAccount<VaultType, TAddress>;
export function decodeVaultType<TAddress extends string = string>(
  encodedAccount: EncodedAccount<TAddress> | MaybeEncodedAccount<TAddress>
): Account<VaultType, TAddress> | MaybeAccount<VaultType, TAddress> {
  return decodeAccount(
    encodedAccount as MaybeEncodedAccount<TAddress>,
    getVaultTypeDecoder()
  );
}

export async function fetchVaultType<TAddress extends string = string>(
  rpc: Parameters<typeof fetchEncodedAccount>[0],
  address: Address<TAddress>,
  config?: FetchAccountConfig
): Promise<Account<VaultType, TAddress>> {
  const maybeAccount = await fetchMaybeVaultType(rpc, address, config);
  assertAccountExists(maybeAccount);
  return maybeAccount;
}

export async function fetchMaybeVaultType<TAddress extends string = string>(
  rpc: Parameters<typeof fetchEncodedAccount>[0],
  address: Address<TAddress>,
  config?: FetchAccountConfig
): Promise<MaybeAccount<VaultType, TAddress>> {
  const maybeAccount = await fetchEncodedAccount(rpc, address, config);
  return decodeVaultType(maybeAccount);
}

export async function fetchAllVaultType(
  rpc: Parameters<typeof fetchEncodedAccounts>[0],
  addresses: Array<Address>,
  config?: FetchAccountsConfig
): Promise<Account<VaultType>[]> {
  const maybeAccounts = await fetchAllMaybeVaultType(rpc, addresses, config);
  assertAccountsExist(maybeAccounts);
  return maybeAccounts;
}

export async function fetchAllMaybeVaultType(
  rpc: Parameters<typeof fetchEncodedAccounts>[0],
  addresses: Array<Address>,
  config?: FetchAccountsConfig
): Promise<MaybeAccount<VaultType>[]> {
  const maybeAccounts = await fetchEncodedAccounts(rpc, addresses, config);
  return maybeAccounts.map((maybeAccount) => decodeVaultType(maybeAccount));
}

export function getVaultTypeSize(): number {
  return 178;
}