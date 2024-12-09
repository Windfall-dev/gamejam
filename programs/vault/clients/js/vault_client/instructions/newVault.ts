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
  type WritableSignerAccount,
} from '@solana/web3.js';
import { VAULT_PROGRAM_ADDRESS } from '../programs';
import {
  expectAddress,
  getAccountMetaFactory,
  type ResolvedAccount,
} from '../shared';

export const NEW_VAULT_DISCRIMINATOR = new Uint8Array([
  0, 196, 119, 39, 154, 60, 10, 44,
]);

export function getNewVaultDiscriminatorBytes() {
  return fixEncoderSize(getBytesEncoder(), 8).encode(NEW_VAULT_DISCRIMINATOR);
}

export type NewVaultInstruction<
  TProgram extends string = typeof VAULT_PROGRAM_ADDRESS,
  TAccountVault extends string | IAccountMeta<string> = string,
  TAccountVaultType extends string | IAccountMeta<string> = string,
  TAccountUserAuthority extends string | IAccountMeta<string> = string,
  TAccountPayer extends string | IAccountMeta<string> = string,
  TAccountSystemProgram extends
    | string
    | IAccountMeta<string> = '11111111111111111111111111111111',
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
      TAccountPayer extends string
        ? WritableSignerAccount<TAccountPayer> &
            IAccountSignerMeta<TAccountPayer>
        : TAccountPayer,
      TAccountSystemProgram extends string
        ? ReadonlyAccount<TAccountSystemProgram>
        : TAccountSystemProgram,
      ...TRemainingAccounts,
    ]
  >;

export type NewVaultInstructionData = { discriminator: ReadonlyUint8Array };

export type NewVaultInstructionDataArgs = {};

export function getNewVaultInstructionDataEncoder(): Encoder<NewVaultInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([['discriminator', fixEncoderSize(getBytesEncoder(), 8)]]),
    (value) => ({ ...value, discriminator: NEW_VAULT_DISCRIMINATOR })
  );
}

export function getNewVaultInstructionDataDecoder(): Decoder<NewVaultInstructionData> {
  return getStructDecoder([
    ['discriminator', fixDecoderSize(getBytesDecoder(), 8)],
  ]);
}

export function getNewVaultInstructionDataCodec(): Codec<
  NewVaultInstructionDataArgs,
  NewVaultInstructionData
> {
  return combineCodec(
    getNewVaultInstructionDataEncoder(),
    getNewVaultInstructionDataDecoder()
  );
}

export type NewVaultAsyncInput<
  TAccountVault extends string = string,
  TAccountVaultType extends string = string,
  TAccountUserAuthority extends string = string,
  TAccountPayer extends string = string,
  TAccountSystemProgram extends string = string,
> = {
  vault?: Address<TAccountVault>;
  vaultType: Address<TAccountVaultType>;
  userAuthority: TransactionSigner<TAccountUserAuthority>;
  payer: TransactionSigner<TAccountPayer>;
  systemProgram?: Address<TAccountSystemProgram>;
};

export async function getNewVaultInstructionAsync<
  TAccountVault extends string,
  TAccountVaultType extends string,
  TAccountUserAuthority extends string,
  TAccountPayer extends string,
  TAccountSystemProgram extends string,
  TProgramAddress extends Address = typeof VAULT_PROGRAM_ADDRESS,
>(
  input: NewVaultAsyncInput<
    TAccountVault,
    TAccountVaultType,
    TAccountUserAuthority,
    TAccountPayer,
    TAccountSystemProgram
  >,
  config?: { programAddress?: TProgramAddress }
): Promise<
  NewVaultInstruction<
    TProgramAddress,
    TAccountVault,
    TAccountVaultType,
    TAccountUserAuthority,
    TAccountPayer,
    TAccountSystemProgram
  >
> {
  // Program address.
  const programAddress = config?.programAddress ?? VAULT_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    vault: { value: input.vault ?? null, isWritable: true },
    vaultType: { value: input.vaultType ?? null, isWritable: false },
    userAuthority: { value: input.userAuthority ?? null, isWritable: false },
    payer: { value: input.payer ?? null, isWritable: true },
    systemProgram: { value: input.systemProgram ?? null, isWritable: false },
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
  if (!accounts.systemProgram.value) {
    accounts.systemProgram.value =
      '11111111111111111111111111111111' as Address<'11111111111111111111111111111111'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.vault),
      getAccountMeta(accounts.vaultType),
      getAccountMeta(accounts.userAuthority),
      getAccountMeta(accounts.payer),
      getAccountMeta(accounts.systemProgram),
    ],
    programAddress,
    data: getNewVaultInstructionDataEncoder().encode({}),
  } as NewVaultInstruction<
    TProgramAddress,
    TAccountVault,
    TAccountVaultType,
    TAccountUserAuthority,
    TAccountPayer,
    TAccountSystemProgram
  >;

  return instruction;
}

export type NewVaultInput<
  TAccountVault extends string = string,
  TAccountVaultType extends string = string,
  TAccountUserAuthority extends string = string,
  TAccountPayer extends string = string,
  TAccountSystemProgram extends string = string,
> = {
  vault: Address<TAccountVault>;
  vaultType: Address<TAccountVaultType>;
  userAuthority: TransactionSigner<TAccountUserAuthority>;
  payer: TransactionSigner<TAccountPayer>;
  systemProgram?: Address<TAccountSystemProgram>;
};

export function getNewVaultInstruction<
  TAccountVault extends string,
  TAccountVaultType extends string,
  TAccountUserAuthority extends string,
  TAccountPayer extends string,
  TAccountSystemProgram extends string,
  TProgramAddress extends Address = typeof VAULT_PROGRAM_ADDRESS,
>(
  input: NewVaultInput<
    TAccountVault,
    TAccountVaultType,
    TAccountUserAuthority,
    TAccountPayer,
    TAccountSystemProgram
  >,
  config?: { programAddress?: TProgramAddress }
): NewVaultInstruction<
  TProgramAddress,
  TAccountVault,
  TAccountVaultType,
  TAccountUserAuthority,
  TAccountPayer,
  TAccountSystemProgram
> {
  // Program address.
  const programAddress = config?.programAddress ?? VAULT_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    vault: { value: input.vault ?? null, isWritable: true },
    vaultType: { value: input.vaultType ?? null, isWritable: false },
    userAuthority: { value: input.userAuthority ?? null, isWritable: false },
    payer: { value: input.payer ?? null, isWritable: true },
    systemProgram: { value: input.systemProgram ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Resolve default values.
  if (!accounts.systemProgram.value) {
    accounts.systemProgram.value =
      '11111111111111111111111111111111' as Address<'11111111111111111111111111111111'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.vault),
      getAccountMeta(accounts.vaultType),
      getAccountMeta(accounts.userAuthority),
      getAccountMeta(accounts.payer),
      getAccountMeta(accounts.systemProgram),
    ],
    programAddress,
    data: getNewVaultInstructionDataEncoder().encode({}),
  } as NewVaultInstruction<
    TProgramAddress,
    TAccountVault,
    TAccountVaultType,
    TAccountUserAuthority,
    TAccountPayer,
    TAccountSystemProgram
  >;

  return instruction;
}

export type ParsedNewVaultInstruction<
  TProgram extends string = typeof VAULT_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    vault: TAccountMetas[0];
    vaultType: TAccountMetas[1];
    userAuthority: TAccountMetas[2];
    payer: TAccountMetas[3];
    systemProgram: TAccountMetas[4];
  };
  data: NewVaultInstructionData;
};

export function parseNewVaultInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedNewVaultInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 5) {
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
      payer: getNextAccount(),
      systemProgram: getNextAccount(),
    },
    data: getNewVaultInstructionDataDecoder().decode(instruction.data),
  };
}
