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
  u64,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  expectPublicKey,
  getAccountMetasAndSigners,
} from '../shared';

// Accounts.
export type WithdrawInstructionAccounts = {
  vault?: PublicKey | Pda;
  vaultType: PublicKey | Pda;
  userAuthority: Signer;
  mint: PublicKey | Pda;
  pool?: PublicKey | Pda;
  to: PublicKey | Pda;
  systemProgram?: PublicKey | Pda;
  tokenProgram?: PublicKey | Pda;
};

// Data.
export type WithdrawInstructionData = {
  discriminator: Uint8Array;
  amount: bigint;
};

export type WithdrawInstructionDataArgs = { amount: number | bigint };

export function getWithdrawInstructionDataSerializer(): Serializer<
  WithdrawInstructionDataArgs,
  WithdrawInstructionData
> {
  return mapSerializer<
    WithdrawInstructionDataArgs,
    any,
    WithdrawInstructionData
  >(
    struct<WithdrawInstructionData>(
      [
        ['discriminator', bytes({ size: 8 })],
        ['amount', u64()],
      ],
      { description: 'WithdrawInstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: new Uint8Array([183, 18, 70, 156, 148, 109, 161, 34]),
    })
  ) as Serializer<WithdrawInstructionDataArgs, WithdrawInstructionData>;
}

// Args.
export type WithdrawInstructionArgs = WithdrawInstructionDataArgs;

// Instruction.
export function withdraw(
  context: Pick<Context, 'eddsa' | 'programs'>,
  input: WithdrawInstructionAccounts & WithdrawInstructionArgs
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
      isWritable: true as boolean,
      value: input.vaultType ?? null,
    },
    userAuthority: {
      index: 2,
      isWritable: false as boolean,
      value: input.userAuthority ?? null,
    },
    mint: { index: 3, isWritable: false as boolean, value: input.mint ?? null },
    pool: { index: 4, isWritable: true as boolean, value: input.pool ?? null },
    to: { index: 5, isWritable: true as boolean, value: input.to ?? null },
    systemProgram: {
      index: 6,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
    tokenProgram: {
      index: 7,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: WithdrawInstructionArgs = { ...input };

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
  if (!resolvedAccounts.tokenProgram.value) {
    resolvedAccounts.tokenProgram.value = context.programs.getPublicKey(
      'splToken',
      'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'
    );
    resolvedAccounts.tokenProgram.isWritable = false;
  }
  if (!resolvedAccounts.pool.value) {
    resolvedAccounts.pool.value = context.eddsa.findPda(programId, [
      publicKeySerializer().serialize(
        expectPublicKey(resolvedAccounts.vaultType.value)
      ),
      publicKeySerializer().serialize(
        expectPublicKey(resolvedAccounts.tokenProgram.value)
      ),
      publicKeySerializer().serialize(
        expectPublicKey(resolvedAccounts.mint.value)
      ),
    ]);
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
  const data = getWithdrawInstructionDataSerializer().serialize(
    resolvedArgs as WithdrawInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
