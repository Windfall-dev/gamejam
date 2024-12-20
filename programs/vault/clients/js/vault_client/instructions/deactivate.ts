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
  getAddressEncoder,
  getBytesDecoder,
  getBytesEncoder,
  getProgramDerivedAddress,
  getStructDecoder,
  getStructEncoder,
  transformEncoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
  type IAccountMeta,
  type IAccountSignerMeta,
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type ReadonlyAccount,
  type ReadonlySignerAccount,
  type ReadonlyUint8Array,
  type TransactionSigner,
  type WritableAccount,
} from '@solana/web3.js';
import { VAULT_PROGRAM_ADDRESS } from '../programs';
import {
  expectAddress,
  getAccountMetaFactory,
  type ResolvedAccount,
} from '../shared';

export const DEACTIVATE_DISCRIMINATOR = new Uint8Array([
  44, 112, 33, 172, 113, 28, 142, 13,
]);

export function getDeactivateDiscriminatorBytes() {
  return fixEncoderSize(getBytesEncoder(), 8).encode(DEACTIVATE_DISCRIMINATOR);
}

export type DeactivateInstruction<
  TProgram extends string = typeof VAULT_PROGRAM_ADDRESS,
  TAccountVault extends string | IAccountMeta<string> = string,
  TAccountVaultType extends string | IAccountMeta<string> = string,
  TAccountUserAuthority extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountVault extends string
        ? WritableAccount<TAccountVault>
        : TAccountVault,
      TAccountVaultType extends string
        ? ReadonlyAccount<TAccountVaultType>
        : TAccountVaultType,
      TAccountUserAuthority extends string
        ? ReadonlySignerAccount<TAccountUserAuthority> &
            IAccountSignerMeta<TAccountUserAuthority>
        : TAccountUserAuthority,
      ...TRemainingAccounts,
    ]
  >;

export type DeactivateInstructionData = { discriminator: ReadonlyUint8Array };

export type DeactivateInstructionDataArgs = {};

export function getDeactivateInstructionDataEncoder(): Encoder<DeactivateInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([['discriminator', fixEncoderSize(getBytesEncoder(), 8)]]),
    (value) => ({ ...value, discriminator: DEACTIVATE_DISCRIMINATOR })
  );
}

export function getDeactivateInstructionDataDecoder(): Decoder<DeactivateInstructionData> {
  return getStructDecoder([
    ['discriminator', fixDecoderSize(getBytesDecoder(), 8)],
  ]);
}

export function getDeactivateInstructionDataCodec(): Codec<
  DeactivateInstructionDataArgs,
  DeactivateInstructionData
> {
  return combineCodec(
    getDeactivateInstructionDataEncoder(),
    getDeactivateInstructionDataDecoder()
  );
}

export type DeactivateAsyncInput<
  TAccountVault extends string = string,
  TAccountVaultType extends string = string,
  TAccountUserAuthority extends string = string,
> = {
  vault?: Address<TAccountVault>;
  vaultType: Address<TAccountVaultType>;
  userAuthority: TransactionSigner<TAccountUserAuthority>;
};

export async function getDeactivateInstructionAsync<
  TAccountVault extends string,
  TAccountVaultType extends string,
  TAccountUserAuthority extends string,
  TProgramAddress extends Address = typeof VAULT_PROGRAM_ADDRESS,
>(
  input: DeactivateAsyncInput<
    TAccountVault,
    TAccountVaultType,
    TAccountUserAuthority
  >,
  config?: { programAddress?: TProgramAddress }
): Promise<
  DeactivateInstruction<
    TProgramAddress,
    TAccountVault,
    TAccountVaultType,
    TAccountUserAuthority
  >
> {
  // Program address.
  const programAddress = config?.programAddress ?? VAULT_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    vault: { value: input.vault ?? null, isWritable: true },
    vaultType: { value: input.vaultType ?? null, isWritable: false },
    userAuthority: { value: input.userAuthority ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Resolve default values.
  if (!accounts.vault.value) {
    accounts.vault.value = await getProgramDerivedAddress({
      programAddress,
      seeds: [
        getBytesEncoder().encode(new Uint8Array([118, 97, 117, 108, 116])),
        getAddressEncoder().encode(expectAddress(accounts.vaultType.value)),
        getAddressEncoder().encode(expectAddress(accounts.userAuthority.value)),
      ],
    });
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.vault),
      getAccountMeta(accounts.vaultType),
      getAccountMeta(accounts.userAuthority),
    ],
    programAddress,
    data: getDeactivateInstructionDataEncoder().encode({}),
  } as DeactivateInstruction<
    TProgramAddress,
    TAccountVault,
    TAccountVaultType,
    TAccountUserAuthority
  >;

  return instruction;
}

export type DeactivateInput<
  TAccountVault extends string = string,
  TAccountVaultType extends string = string,
  TAccountUserAuthority extends string = string,
> = {
  vault: Address<TAccountVault>;
  vaultType: Address<TAccountVaultType>;
  userAuthority: TransactionSigner<TAccountUserAuthority>;
};

export function getDeactivateInstruction<
  TAccountVault extends string,
  TAccountVaultType extends string,
  TAccountUserAuthority extends string,
  TProgramAddress extends Address = typeof VAULT_PROGRAM_ADDRESS,
>(
  input: DeactivateInput<
    TAccountVault,
    TAccountVaultType,
    TAccountUserAuthority
  >,
  config?: { programAddress?: TProgramAddress }
): DeactivateInstruction<
  TProgramAddress,
  TAccountVault,
  TAccountVaultType,
  TAccountUserAuthority
> {
  // Program address.
  const programAddress = config?.programAddress ?? VAULT_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    vault: { value: input.vault ?? null, isWritable: true },
    vaultType: { value: input.vaultType ?? null, isWritable: false },
    userAuthority: { value: input.userAuthority ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.vault),
      getAccountMeta(accounts.vaultType),
      getAccountMeta(accounts.userAuthority),
    ],
    programAddress,
    data: getDeactivateInstructionDataEncoder().encode({}),
  } as DeactivateInstruction<
    TProgramAddress,
    TAccountVault,
    TAccountVaultType,
    TAccountUserAuthority
  >;

  return instruction;
}

export type ParsedDeactivateInstruction<
  TProgram extends string = typeof VAULT_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    vault: TAccountMetas[0];
    vaultType: TAccountMetas[1];
    userAuthority: TAccountMetas[2];
  };
  data: DeactivateInstructionData;
};

export function parseDeactivateInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedDeactivateInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 3) {
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
      vault: getNextAccount(),
      vaultType: getNextAccount(),
      userAuthority: getNextAccount(),
    },
    data: getDeactivateInstructionDataDecoder().decode(instruction.data),
  };
}
