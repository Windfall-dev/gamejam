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
export type DeactivateInstructionAccounts = {
  vault?: PublicKey | Pda;
  vaultType: PublicKey | Pda;
  userAuthority: Signer;
};

// Data.
export type DeactivateInstructionData = { discriminator: Uint8Array };

export type DeactivateInstructionDataArgs = {};

export function getDeactivateInstructionDataSerializer(): Serializer<
  DeactivateInstructionDataArgs,
  DeactivateInstructionData
> {
  return mapSerializer<
    DeactivateInstructionDataArgs,
    any,
    DeactivateInstructionData
  >(
    struct<DeactivateInstructionData>([['discriminator', bytes({ size: 8 })]], {
      description: 'DeactivateInstructionData',
    }),
    (value) => ({
      ...value,
      discriminator: new Uint8Array([44, 112, 33, 172, 113, 28, 142, 13]),
    })
  ) as Serializer<DeactivateInstructionDataArgs, DeactivateInstructionData>;
}

// Instruction.
export function deactivate(
  context: Pick<Context, 'eddsa' | 'programs'>,
  input: DeactivateInstructionAccounts
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
  const data = getDeactivateInstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
