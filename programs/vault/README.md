# Windfall vault program

This repository contains the `vault` program that enables SPL token deposits and withdrawals.

We assume that the deposit and withdrawal amounts are equal, regardless of whether the token is an interest-bearing token or not.

Of course, it is possible to withdraw only a portion of tokens or make additional deposits.

## Prerequisites

- Solana CLI 1.18.26
- Anchor 0.30.1
- pnpm 9.x (npm or yarn should work as well)

## Build

You can build our `vault` program with the following commands:

```shell
pnpm install
anchor keys sync
anchor build
```

## Test

You can run local tests with the following commands:

```shell
# Execute local Typescript tests.
anchor test

# Execute local Rust tests.
pnpm run test:rust
```

Fuzz tests by [Trident](https://github.com/Ackee-Blockchain/trident) are also implemented but I'm not quite sure how to utilize the test results properly.

You can run them with the following command:

```shell
# Run Trident fuzz tests.
pnpm run test:fuzz_0
```

## Program overview

The `vault` program currently has two structures:

- `VaultType`
- `Vault`

### `VaultType`

Created by Windfall team for each SPL token that Windfall accepts deposits, and it controls the overall deposit / withdrawal behavior.

The `VaultType` has a concept of "seasons", where the start and duration of the current season are represented by the `season_start` and `season_duration` fields in the structure.

Additionally, there are two types of cooldown windows (`deactivation_lock_window` and `cooldown_window`) at the end of each season.

The `deactivation_lock_window` field defines the duration during which the `deactivation` operation mentioned below is restricted. For example, in the case of SOL vaults, the `deactivation_lock_window` is set to around three days to allow the Windfall team to unstake tokens in preparation for withdrawals.

The `cooldown_window` field defines a few hours period during which Windfall team performs maintenance work on the vaults, but currently not used to restrict any user operations.

> Note that since Windfall is considering opening this program as infrastructure to other projects, a `VaultType` PDA is derived from the authority as well as the token mint to avoid address collisions.

### `Vault`

A structure created for each `VaultType` and also for each user wallet to manage the user's deposit status.

### Vault Status and Withdrawal Restrictions

When users deposit tokens, their vault transitions to an `active` state. As a general rule, withdrawals are not permitted while vaults are active.

Users who wish to withdraw must `deactivate` their vault, which transitions it to a `deactivating` status.

A deactivating vault transitions to an `inactive` status in the next season, at which point withdrawal becomes possible.

As an exception, if the `instant_deactivation` field of the `VaultType` is true, when users `deactivate` their vault, it transitions immediately to the `inactive` status, allowing them to withdraw funds right away.

If users want to withdraw only a portion of their tokens, they need to first `deactivate` their vault, then `withdraw` the desired amount, and finally `activate` their vault again to continue staking the remaining tokens.

## Operational overview

### New vault type and normal operations

Let's say the Windfall team decides to support deposits of a new SPL token XYZ.

1. The team creates `pool` and `reserve` token accounts, and also initializes a `VaultType` for XYZ (`new_vault_type`).
2. Users create their `Vault` belonging to the `VaultType` and deposit XYZ into the `pool` (`new_vault` and `deposit`).
3. The team transfers XYZ from the `pool` to separate token accounts for operational management (`transfer_vault_type_token`).
4. The team utilizes XYZ through staking and other DeFi protocols.
5. Users, who wish to withdraw, deactivate their vault (`deactivate`).
6. The team unstakes and withdraws some amount of XYZ from protocols in preparation for withdrawals, returning them to the `reserve` account.
7. When the next season begins, either the team or users roll over the `VaultType` to advance the season (`roll_over_vault_type`).
8. Users withdraw from their vault (`withdraw`).
9. If users want to deposit the remaining tokens after a partial withdrawal, they need to activate their vault (`activate`)

The process from steps 2-9 repeats as seasons progress.

### Token accounts associated with vault type

The `pool` and `reserve` token accounts are associated with a `VaultType` and are used to manage the deposit and withdrawal of tokens.

Since all deposits are made to the `pool` token account, tokens held in this account are typically transferred to other token accounts managed by the team for DeFi operations.

The `reserve` token account holds tokens reserved for withdrawals, and the team replenishes it periodically based on users' deactivation status.

User withdrawals are made from the `reserve` token account, but if there are insufficient funds, the system will also use tokens from the `pool` token account if available.

### Sunsetting vault type

If the team wants to stop accepting XYZ, they follow this process:

1. The team decides to stop accepting XYZ and encourages users to withdraw.
2. Users withdraw their XYZ and close their vault (`withdraw` and `close_vault`).
3. Once all users have withdrawn, the team can close the `VaultType` (`close_vault_type`).

Realistically, it may be difficult to get all users to withdraw, but having an uncloseable `VaultType` account is not a significant cost issue. If the team absolutely needs to force refunds to users, they could consider solutions like using ZK Compression technology to airdrop XYZ to all wallets while minimizing costs.

### Locked vault type

When the team wants to prohibit withdrawals for a `VaultType`, they can lock it by calling `lock_vault_type`.

While a `VaultType` is locked, users cannot deactivate their vault, and therefore cannot withdraw, even if seasons progress.

This mechanism allows for initial deposits that cannot be withdrawn until the alpha version is released, with the expectation that the lock will be removed when the alpha version launches and normal season cycles begin.

### Changing authority

Since `VaultType` is operated for a long time, the team may want to change the operating authority.

In that case, they should create a new authority key and follow these steps:

1. Call `nominate_vault_type_authority` to nominate the new authority.
2. Call `accept_vault_type_authority` with the new key's signature to accept the authority change.

This two-step process provides room to handle operational errors.

While there is no explicit way to cancel a nomination, the system can continue operating with the current authority as long as the nomination is not accepted.

If there is a need to change to a different authority later, `nominate_vault_type_authority` can be called again.

