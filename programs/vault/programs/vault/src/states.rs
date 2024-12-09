use anchor_lang::prelude::*;

use crate::constants::*;

/// A system-wide fund management structure created by the administrator.
/// One is created for each SPL token that can be deposited.
#[account]
pub struct VaultType {
    /// The identity pubkey which matches the authority initially.
    /// It is maintained separately from the authority to allow changing the authority without affecting the PDA.
    pub identity: Pubkey,

    /// The pubkey of the authority which can be changed with `update_vault_type_authority` instruction.
    pub authority: Pubkey,

    /// The pubkey of the pending authority which will be set after `update_vault_type_authority` instruction.
    pub pending_authority: Option<Pubkey>,

    /// The pubkey of the token mint to be deposited to vaults.
    pub mint: Pubkey,

    /// The pubkey of the pool token account where deposited tokens are collected.
    pub pool: Pubkey,

    /// The pubkey of the reserve token account where tokens are collected for withdrawal.
    pub reserve: Pubkey,

    /// The pubkey of the token program (spl_token or spl_token_2022).
    pub token_program: Pubkey,

    /// The start timestamp of the current season.
    pub season_start: i64,

    /// The duration of each season in seconds.
    pub season_duration: i64,

    /// The duration, while users cannot initiate deactivation, at the end of each season, in seconds.
    pub deactivation_lock_window: i64,

    /// The duration of the cooldown period at the end of each season, in seconds,
    /// during which administrators perform maintenance work on the vaults.
    /// Currently not used to restrict any operations, but may be used in the future to disable deposits.
    pub cooldown_window: i64,

    /// The maximum amount of tokens that can be deposited to each vault. No limit if 0.
    pub max_deposit_per_user: u64,

    /// The total amount of tokens deposited across all vaults belonging to this vault type.
    pub total_deposit: u64,

    /// If true, users can instantly deactivate their vaults to Inactive state.
    /// Otherwise, vaults enter Deactivating state and can transition to Inactive at the start of next season.
    pub instant_deactivation: bool,

    /// If true, belonging vaults are locked and cannot be deactivated for withdrawal.
    /// This does not affect already deactivating and inactive vaults.
    pub is_locked: bool,

    /// The bump seed of this pda.
    pub bump: u8,
}

impl VaultType {
    pub const SIZE: usize = std::mem::size_of::<Self>();

    pub fn pda(mint: &Pubkey, authority: &Pubkey) -> (Pubkey, u8) {
        let seed = [VAULT_TYPE_PREFIX, mint.as_ref(), authority.as_ref()];
        Pubkey::find_program_address(&seed, &crate::ID_CONST)
    }
}

/// An enum representing the status of a Vault.
/// After deposit, it becomes Active, and must be Inactive to withdraw.
#[derive(Debug, Clone, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
pub enum VaultStatus {
    Active,
    Deactivating,
    Inactive,
}

// A structure that manages deposit information for each user and each SPL token.
#[account]
pub struct Vault {
    /// The pubkey of the authority (usually the user).
    pub user_authority: Pubkey,

    /// The pubkey of the vault type.
    pub vault_type: Pubkey,

    /// The amount of token the user has deposited.
    pub amount: u64,

    /// The timestamp when the vault becomes inactive.
    pub inactive_at: i64,

    /// Current vault status.
    pub status: VaultStatus,

    /// The bump seed of this pda.
    pub bump: u8,
}

impl Vault {
    pub const SIZE: usize = std::mem::size_of::<Self>();

    pub fn pda(vault_type: &Pubkey, user_authority: &Pubkey) -> (Pubkey, u8) {
        let seed = [VAULT_PREFIX, vault_type.as_ref(), user_authority.as_ref()];
        Pubkey::find_program_address(&seed, &crate::ID_CONST)
    }
}
