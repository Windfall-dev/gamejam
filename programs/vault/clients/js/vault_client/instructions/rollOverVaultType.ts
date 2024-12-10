/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  combineCodec,
  fixDecoderSize,
  fixEncoderSize,
  getBytesDecoder,
  getBytesEncoder,
  getStructDecoder,
  getStructEncoder,
  transformEncoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
  type IAccountMeta,
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type ReadonlyUint8Array,
  type WritableAccount,
} from '@solana/web3.js';
import { VAULT_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const ROLL_OVER_VAULT_TYPE_DISCRIMINATOR = new Uint8Array([
  233, 161, 46, 228, 96, 94, 245, 57,
]);

export function getRollOverVaultTypeDiscriminatorBytes() {
  return fixEncoderSize(getBytesEncoder(), 8).encode(
    ROLL_OVER_VAULT_TYPE_DISCRIMINATOR
  );
}

export type RollOverVaultTypeInstruction<
  TProgram extends string = typeof VAULT_PROGRAM_ADDRESS,
  TAccountVaultType extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountVaultType extends string
        ? WritableAccount<TAccountVaultType>
        : TAccountVaultType,
      ...TRemainingAccounts,
    ]
  >;

export type RollOverVaultTypeInstructionData = {
  discriminator: ReadonlyUint8Array;
};

export type RollOverVaultTypeInstructionDataArgs = {};

export function getRollOverVaultTypeInstructionDataEncoder(): Encoder<RollOverVaultTypeInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([['discriminator', fixEncoderSize(getBytesEncoder(), 8)]]),
    (value) => ({ ...value, discriminator: ROLL_OVER_VAULT_TYPE_DISCRIMINATOR })
  );
}

export function getRollOverVaultTypeInstructionDataDecoder(): Decoder<RollOverVaultTypeInstructionData> {
  return getStructDecoder([
    ['discriminator', fixDecoderSize(getBytesDecoder(), 8)],
  ]);
}

export function getRollOverVaultTypeInstructionDataCodec(): Codec<
  RollOverVaultTypeInstructionDataArgs,
  RollOverVaultTypeInstructionData
> {
  return combineCodec(
    getRollOverVaultTypeInstructionDataEncoder(),
    getRollOverVaultTypeInstructionDataDecoder()
  );
}

export type RollOverVaultTypeInput<TAccountVaultType extends string = string> =
  {
    vaultType: Address<TAccountVaultType>;
  };

export function getRollOverVaultTypeInstruction<
  TAccountVaultType extends string,
  TProgramAddress extends Address = typeof VAULT_PROGRAM_ADDRESS,
>(
  input: RollOverVaultTypeInput<TAccountVaultType>,
  config?: { programAddress?: TProgramAddress }
): RollOverVaultTypeInstruction<TProgramAddress, TAccountVaultType> {
  // Program address.
  const programAddress = config?.programAddress ?? VAULT_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    vaultType: { value: input.vaultType ?? null, isWritable: true },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [getAccountMeta(accounts.vaultType)],
    programAddress,
    data: getRollOverVaultTypeInstructionDataEncoder().encode({}),
  } as RollOverVaultTypeInstruction<TProgramAddress, TAccountVaultType>;

  return instruction;
}

export type ParsedRollOverVaultTypeInstruction<
  TProgram extends string = typeof VAULT_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    vaultType: TAccountMetas[0];
  };
  data: RollOverVaultTypeInstructionData;
};

export function parseRollOverVaultTypeInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedRollOverVaultTypeInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 1) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      vaultType: getNextAccount(),
    },
    data: getRollOverVaultTypeInstructionDataDecoder().decode(instruction.data),
  };
}