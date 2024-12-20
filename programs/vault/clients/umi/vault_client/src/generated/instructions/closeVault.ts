/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  Context,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  bytes,
  mapSerializer,
  publicKey as publicKeySerializer,
  struct,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  expectPublicKey,
  getAccountMetasAndSigners,
} from '../shared';

// Accounts.
export type CloseVaultInstructionAccounts = {
  vault?: PublicKey | Pda;
  vaultType: PublicKey | Pda;
  userAuthority: Signer;
  payer?: Signer;
  systemProgram?: PublicKey | Pda;
};

// Data.
export type CloseVaultInstructionData = { discriminator: Uint8Array };

export type CloseVaultInstructionDataArgs = {};

export function getCloseVaultInstructionDataSerializer(): Serializer<
  CloseVaultInstructionDataArgs,
  CloseVaultInstructionData
> {
  return mapSerializer<
    CloseVaultInstructionDataArgs,
    any,
    CloseVaultInstructionData
  >(
    struct<CloseVaultInstructionData>([['discriminator', bytes({ size: 8 })]], {
      description: 'CloseVaultInstructionData',
    }),
    (value) => ({
      ...value,
      discriminator: new Uint8Array([141, 103, 17, 126, 72, 75, 29, 29]),
    })
  ) as Serializer<CloseVaultInstructionDataArgs, CloseVaultInstructionData>;
}

// Instruction.
export function closeVault(
  context: Pick<Context, 'eddsa' | 'payer' | 'programs'>,
  input: CloseVaultInstructionAccounts
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'vault',
    'Ds1WLBK4R44S3bk1DeDiUrwxQ9ydrdjn7hkrY61EDVXF'
  );

  // Accounts.
  const resolvedAccounts = {
    vault: {
      index: 0,
      isWritable: true as boolean,
      value: input.vault ?? null,
    },
    vaultType: {
      index: 1,
      isWritable: false as boolean,
      value: input.vaultType ?? null,
    },
    userAuthority: {
      index: 2,
      isWritable: false as boolean,
      value: input.userAuthority ?? null,
    },
    payer: {
      index: 3,
      isWritable: true as boolean,
      value: input.payer ?? null,
    },
    systemProgram: {
      index: 4,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Default values.
  if (!resolvedAccounts.vault.value) {
    resolvedAccounts.vault.value = context.eddsa.findPda(programId, [
      bytes().serialize(new Uint8Array([118, 97, 117, 108, 116])),
      publicKeySerializer().serialize(
        expectPublicKey(resolvedAccounts.vaultType.value)
      ),
      publicKeySerializer().serialize(
        expectPublicKey(resolvedAccounts.userAuthority.value)
      ),
    ]);
  }
  if (!resolvedAccounts.payer.value) {
    resolvedAccounts.payer.value = context.payer;
  }
  if (!resolvedAccounts.systemProgram.value) {
    resolvedAccounts.systemProgram.value = context.programs.getPublicKey(
      'systemProgram',
      '11111111111111111111111111111111'
    );
    resolvedAccounts.systemProgram.isWritable = false;
  }

  // Accounts in order.
  const orderedAccounts: ResolvedAccount[] = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    'programId',
    programId
  );

  // Data.
  const data = getCloseVaultInstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
