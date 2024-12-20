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
  TransactionBuilder,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  bytes,
  mapSerializer,
  struct,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';

// Accounts.
export type RollOverVaultTypeInstructionAccounts = {
  vaultType: PublicKey | Pda;
};

// Data.
export type RollOverVaultTypeInstructionData = { discriminator: Uint8Array };

export type RollOverVaultTypeInstructionDataArgs = {};

export function getRollOverVaultTypeInstructionDataSerializer(): Serializer<
  RollOverVaultTypeInstructionDataArgs,
  RollOverVaultTypeInstructionData
> {
  return mapSerializer<
    RollOverVaultTypeInstructionDataArgs,
    any,
    RollOverVaultTypeInstructionData
  >(
    struct<RollOverVaultTypeInstructionData>(
      [['discriminator', bytes({ size: 8 })]],
      { description: 'RollOverVaultTypeInstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: new Uint8Array([233, 161, 46, 228, 96, 94, 245, 57]),
    })
  ) as Serializer<
    RollOverVaultTypeInstructionDataArgs,
    RollOverVaultTypeInstructionData
  >;
}

// Instruction.
export function rollOverVaultType(
  context: Pick<Context, 'programs'>,
  input: RollOverVaultTypeInstructionAccounts
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'vault',
    'Ds1WLBK4R44S3bk1DeDiUrwxQ9ydrdjn7hkrY61EDVXF'
  );

  // Accounts.
  const resolvedAccounts = {
    vaultType: {
      index: 0,
      isWritable: true as boolean,
      value: input.vaultType ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

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
  const data = getRollOverVaultTypeInstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
