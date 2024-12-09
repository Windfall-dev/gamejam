use trident_client::fuzzing::*;
use vault::constants::{VAULT_PREFIX, VAULT_TYPE_PREFIX};
/// FuzzInstruction contains all available Instructions.
/// Below, the instruction arguments (accounts and data) are defined.
#[derive(Arbitrary, DisplayIx, FuzzTestExecutor)]
pub enum FuzzInstruction {
    AcceptVaultTypeAuthority(AcceptVaultTypeAuthority),
    Activate(Activate),
    CloseVault(CloseVault),
    CloseVaultType(CloseVaultType),
    Deactivate(Deactivate),
    Deposit(Deposit),
    LockVaultType(LockVaultType),
    NewVault(NewVault),
    NewVaultType(NewVaultType),
    NominateVaultTypeAuthority(NominateVaultTypeAuthority),
    RollOverVaultType(RollOverVaultType),
    TransferVaultTypeToken(TransferVaultTypeToken),
    Withdraw(Withdraw),
}
#[derive(Arbitrary, Debug)]
pub struct AcceptVaultTypeAuthority {
    pub accounts: AcceptVaultTypeAuthorityAccounts,
    // pub data: AcceptVaultTypeAuthorityData,
}
#[derive(Arbitrary, Debug)]
pub struct AcceptVaultTypeAuthorityAccounts {
    pub vault_type: AccountId,
    pub new_authority: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct AcceptVaultTypeAuthorityData {}
#[derive(Arbitrary, Debug)]
pub struct Activate {
    pub accounts: ActivateAccounts,
    // pub data: ActivateData,
}
#[derive(Arbitrary, Debug)]
pub struct ActivateAccounts {
    pub vault: AccountId,
    pub vault_type: AccountId,
    pub user_authority: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct ActivateData {}
#[derive(Arbitrary, Debug)]
pub struct CloseVault {
    pub accounts: CloseVaultAccounts,
    // pub data: CloseVaultData,
}
#[derive(Arbitrary, Debug)]
pub struct CloseVaultAccounts {
    pub vault: AccountId,
    pub vault_type: AccountId,
    pub user_authority: AccountId,
    pub payer: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct CloseVaultData {}
#[derive(Arbitrary, Debug)]
pub struct CloseVaultType {
    pub accounts: CloseVaultTypeAccounts,
    // pub data: CloseVaultTypeData,
}
#[derive(Arbitrary, Debug)]
pub struct CloseVaultTypeAccounts {
    pub vault_type: AccountId,
    pub authority: AccountId,
    pub pool: AccountId,
    pub reserve: AccountId,
    pub payer: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct CloseVaultTypeData {}
#[derive(Arbitrary, Debug)]
pub struct Deactivate {
    pub accounts: DeactivateAccounts,
    // pub data: DeactivateData,
}
#[derive(Arbitrary, Debug)]
pub struct DeactivateAccounts {
    pub vault: AccountId,
    pub vault_type: AccountId,
    pub user_authority: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct DeactivateData {}
#[derive(Arbitrary, Debug)]
pub struct Deposit {
    pub accounts: DepositAccounts,
    pub data: DepositData,
}
#[derive(Arbitrary, Debug)]
pub struct DepositAccounts {
    pub vault: AccountId,
    pub vault_type: AccountId,
    pub user_authority: AccountId,
    pub mint: AccountId,
    pub pool: AccountId,
    pub from: AccountId,
    // pub token_program: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct DepositData {
    #[arbitrary(with = |u: &mut arbitrary::Unstructured<'arbitrary>| u.int_in_range(0..=10_000_000_000))]
    pub amount: u64,
}
#[derive(Arbitrary, Debug)]
pub struct LockVaultType {
    pub accounts: LockVaultTypeAccounts,
    pub data: LockVaultTypeData,
}
#[derive(Arbitrary, Debug)]
pub struct LockVaultTypeAccounts {
    pub vault_type: AccountId,
    pub authority: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct LockVaultTypeData {
    pub is_locked: bool,
}
#[derive(Arbitrary, Debug)]
pub struct NewVault {
    pub accounts: NewVaultAccounts,
    // pub data: NewVaultData,
}
#[derive(Arbitrary, Debug)]
pub struct NewVaultAccounts {
    pub vault: AccountId,
    pub vault_type: AccountId,
    pub user_authority: AccountId,
    pub payer: AccountId,
    // pub system_program: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct NewVaultData {}
#[derive(Arbitrary, Debug)]
pub struct NewVaultType {
    pub accounts: NewVaultTypeAccounts,
    pub data: NewVaultTypeData,
}
#[derive(Arbitrary, Debug)]
pub struct NewVaultTypeAccounts {
    pub vault_type: AccountId,
    pub authority: AccountId,
    pub mint: AccountId,
    pub pool: AccountId,
    pub reserve: AccountId,
    pub payer: AccountId,
    // pub system_program: AccountId,
    // pub token_program: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct NewVaultTypeData {
    // let fuzzer generate a random offset which is added to the current timestamp
    #[arbitrary(with = |u: &mut arbitrary::Unstructured<'arbitrary>| u.int_in_range(-3600..=3600))]
    pub season_start: i64,
    #[arbitrary(with = |u: &mut arbitrary::Unstructured<'arbitrary>| u.int_in_range(60*60*24*5..=60*60*24*14))]
    pub season_duration: i64,
    #[arbitrary(with = |u: &mut arbitrary::Unstructured<'arbitrary>| u.int_in_range(1..=60*60*24*3))]
    pub deactivation_lock_window: i64,
    #[arbitrary(with = |u: &mut arbitrary::Unstructured<'arbitrary>| u.int_in_range(1..=60*60*6))]
    pub cooldown_window: i64,
    pub max_deposit_per_user: u64,
    pub instant_deactivation: bool,
}
#[derive(Arbitrary, Debug)]
pub struct NominateVaultTypeAuthority {
    pub accounts: NominateVaultTypeAuthorityAccounts,
    // pub data: NominateVaultTypeAuthorityData,
}
#[derive(Arbitrary, Debug)]
pub struct NominateVaultTypeAuthorityAccounts {
    pub vault_type: AccountId,
    pub authority: AccountId,
    pub new_authority: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct NominateVaultTypeAuthorityData {}
#[derive(Arbitrary, Debug)]
pub struct RollOverVaultType {
    pub accounts: RollOverVaultTypeAccounts,
    // pub data: RollOverVaultTypeData,
}
#[derive(Arbitrary, Debug)]
pub struct RollOverVaultTypeAccounts {
    pub vault_type: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct RollOverVaultTypeData {}
#[derive(Arbitrary, Debug)]
pub struct TransferVaultTypeToken {
    pub accounts: TransferVaultTypeTokenAccounts,
    pub data: TransferVaultTypeTokenData,
}
#[derive(Arbitrary, Debug)]
pub struct TransferVaultTypeTokenAccounts {
    pub vault_type: AccountId,
    pub authority: AccountId,
    pub mint: AccountId,
    pub source: AccountId,
    pub destination: AccountId,
    // pub token_program: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct TransferVaultTypeTokenData {
    pub amount: u64,
}
#[derive(Arbitrary, Debug)]
pub struct Withdraw {
    pub accounts: WithdrawAccounts,
    pub data: WithdrawData,
}
#[derive(Arbitrary, Debug)]
pub struct WithdrawAccounts {
    pub vault: AccountId,
    pub vault_type: AccountId,
    pub user_authority: AccountId,
    pub mint: AccountId,
    pub pool: AccountId,
    pub reserve: AccountId,
    pub to: AccountId,
    // pub token_program: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct WithdrawData {
    #[arbitrary(with = |u: &mut arbitrary::Unstructured<'arbitrary>| u.int_in_range(0..=10_000_000_000))]
    pub amount: u64,
}
///IxOps implementation for `AcceptVaultTypeAuthority` with all required
/// functions.
impl IxOps for AcceptVaultTypeAuthority {
    type IxData = vault::instruction::AcceptVaultTypeAuthority;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        vault::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data = vault::instruction::AcceptVaultTypeAuthority {};
        Ok(data)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        _client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let vault_type = fuzz_accounts.vault_type.get(self.accounts.vault_type);
        let new_authority = fuzz_accounts.new_authority.get(self.accounts.new_authority);
        let signers = vec![new_authority.clone()];
        let acc_meta = vault::accounts::AcceptVaultTypeAuthority {
            vault_type,
            new_authority: new_authority.pubkey(),
        }
        .to_account_metas(None);
        Ok((signers, acc_meta))
    }
}
///IxOps implementation for `Activate` with all required functions.
impl IxOps for Activate {
    type IxData = vault::instruction::Activate;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        vault::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data = vault::instruction::Activate {};
        Ok(data)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        _client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let user_authority = fuzz_accounts
            .user_authority
            .get(self.accounts.user_authority);
        let vault_type = fuzz_accounts.vault_type.get(self.accounts.vault_type);
        let vault = fuzz_accounts.vault.get(self.accounts.vault);
        let signers = vec![user_authority.clone()];
        let acc_meta = vault::accounts::Activate {
            vault,
            vault_type,
            user_authority: user_authority.pubkey(),
        }
        .to_account_metas(None);
        Ok((signers, acc_meta))
    }
}
///IxOps implementation for `CloseVault` with all required functions.
impl IxOps for CloseVault {
    type IxData = vault::instruction::CloseVault;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        vault::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data = vault::instruction::CloseVault {};
        Ok(data)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        _client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let user_authority = fuzz_accounts
            .user_authority
            .get(self.accounts.user_authority);
        let vault_type = fuzz_accounts.vault_type.get(self.accounts.vault_type);
        let vault = fuzz_accounts.vault.get(self.accounts.vault);
        let payer = fuzz_accounts.payer.get(self.accounts.payer);

        let signers = vec![user_authority.clone(), payer.clone()];
        let acc_meta = vault::accounts::CloseVault {
            vault,
            vault_type,
            user_authority: user_authority.pubkey(),
            payer: payer.pubkey(),
        }
        .to_account_metas(None);
        Ok((signers, acc_meta))
    }
}
///IxOps implementation for `CloseVaultType` with all required functions.
impl IxOps for CloseVaultType {
    type IxData = vault::instruction::CloseVaultType;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        vault::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data = vault::instruction::CloseVaultType {};
        Ok(data)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        _client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let admin = fuzz_accounts.authority.get(self.accounts.authority);
        let payer = fuzz_accounts.payer.get(self.accounts.payer);
        let vault_type = fuzz_accounts.vault_type.get(self.accounts.vault_type);
        let pool = fuzz_accounts.pool.get(self.accounts.pool);
        let reserve = fuzz_accounts.reserve.get(self.accounts.reserve);

        let signers = vec![admin.clone(), payer.clone()];
        let acc_meta = vault::accounts::CloseVaultType {
            vault_type: vault_type,
            authority: admin.pubkey(),
            pool: pool,
            reserve: reserve,
            payer: payer.pubkey(),
            token_program: anchor_spl::token::ID,
        }
        .to_account_metas(None);
        Ok((signers, acc_meta))
    }
}
///IxOps implementation for `Deactivate` with all required functions.
impl IxOps for Deactivate {
    type IxData = vault::instruction::Deactivate;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        vault::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data = vault::instruction::Deactivate {};
        Ok(data)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        _client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let user_authority = fuzz_accounts
            .user_authority
            .get(self.accounts.user_authority);
        let vault_type = fuzz_accounts.vault_type.get(self.accounts.vault_type);
        let vault = fuzz_accounts.vault.get(self.accounts.vault);
        let signers = vec![user_authority.clone()];
        let acc_meta = vault::accounts::Deactivate {
            vault,
            vault_type,
            user_authority: user_authority.pubkey(),
        }
        .to_account_metas(None);
        Ok((signers, acc_meta))
    }
}
///IxOps implementation for `Deposit` with all required functions.
impl IxOps for Deposit {
    type IxData = vault::instruction::Deposit;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        vault::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data = vault::instruction::Deposit {
            amount: self.data.amount,
        };
        Ok(data)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let user_authority = fuzz_accounts
            .user_authority
            .get(self.accounts.user_authority);
        let vault_type = fuzz_accounts.vault_type.get(self.accounts.vault_type);
        let vault = fuzz_accounts.vault.get(self.accounts.vault);
        let mint = fuzz_accounts.mint.get(self.accounts.mint);
        let pool = fuzz_accounts.pool.get(self.accounts.pool);
        let from = fuzz_accounts.from.get_or_create_account(
            self.accounts.from,
            client,
            mint,
            user_authority.pubkey(),
            10_000_000_000,
            None,
            None,
            0,
            None,
        );
        let signers = vec![user_authority.clone()];
        let acc_meta = vault::accounts::Deposit {
            vault,
            vault_type,
            user_authority: user_authority.pubkey(),
            mint,
            pool,
            from,
            token_program: anchor_spl::token::ID,
        }
        .to_account_metas(None);
        Ok((signers, acc_meta))
    }
}
///IxOps implementation for `LockVaultType` with all required functions.
impl IxOps for LockVaultType {
    type IxData = vault::instruction::LockVaultType;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        vault::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data = vault::instruction::LockVaultType {
            is_locked: self.data.is_locked,
        };
        Ok(data)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        _client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let admin = fuzz_accounts.authority.get(self.accounts.authority);
        let vault_type = fuzz_accounts.vault_type.get(self.accounts.vault_type);

        let signers = vec![admin.clone()];
        let acc_meta = vault::accounts::LockVaultType {
            vault_type: vault_type,
            authority: admin.pubkey(),
        }
        .to_account_metas(None);
        Ok((signers, acc_meta))
    }
}
///IxOps implementation for `NewVault` with all required functions.
impl IxOps for NewVault {
    type IxData = vault::instruction::NewVault;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        vault::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data = vault::instruction::NewVault {};
        Ok(data)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let user_authority = fuzz_accounts.user_authority.get_or_create_account(
            self.accounts.user_authority,
            client,
            10 * LAMPORTS_PER_SOL,
        );
        let vault_type = fuzz_accounts.vault_type.get(self.accounts.vault_type);
        let vault = fuzz_accounts.vault.get_or_create_account(
            self.accounts.vault,
            client,
            &[
                VAULT_PREFIX,
                vault_type.as_ref(),
                user_authority.pubkey().as_ref(),
            ],
            &vault::ID,
        );
        let payer = fuzz_accounts.payer.get_or_create_account(
            self.accounts.payer,
            client,
            10 * LAMPORTS_PER_SOL,
        );
        let signers = vec![user_authority.clone(), payer.clone()];
        let acc_meta = vault::accounts::NewVault {
            vault,
            vault_type,
            user_authority: user_authority.pubkey(),
            payer: payer.pubkey(),
            system_program: solana_sdk::system_program::ID,
        }
        .to_account_metas(None);
        Ok((signers, acc_meta))
    }
}
///IxOps implementation for `NewVaultType` with all required functions.
impl IxOps for NewVaultType {
    type IxData = vault::instruction::NewVaultType;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        vault::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        // add generate `season_start`, which is a random offset (either negative or positive), to now
        let season_start = now as i64 + self.data.season_start;
        let mut season_duration = self.data.season_duration;
        let deactivation_lock_window = self.data.deactivation_lock_window;
        let cooldown_window = self.data.cooldown_window;
        if season_duration <= deactivation_lock_window {
            season_duration = deactivation_lock_window + 1;
        }
        if season_duration <= cooldown_window {
            season_duration = cooldown_window + 1;
        }
        let data = vault::instruction::NewVaultType {
            season_start,
            season_duration,
            deactivation_lock_window,
            cooldown_window,
            max_deposit_per_user: self.data.max_deposit_per_user,
            instant_deactivation: self.data.instant_deactivation,
        };
        Ok(data)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let admin = fuzz_accounts.authority.get_or_create_account(
            self.accounts.authority,
            client,
            10 * LAMPORTS_PER_SOL,
        );
        let payer = fuzz_accounts.payer.get_or_create_account(
            self.accounts.payer,
            client,
            10 * LAMPORTS_PER_SOL,
        );
        let mint = fuzz_accounts.mint.get_or_create_account(
            self.accounts.mint,
            client,
            9,
            &admin.pubkey(),
            None,
        );
        let vault_type = fuzz_accounts.vault_type.get_or_create_account(
            self.accounts.vault_type,
            client,
            &[VAULT_TYPE_PREFIX, mint.as_ref(), admin.pubkey().as_ref()],
            &vault::ID,
        );
        //let pool = get_associated_token_address(&vault_type, &mint);
        let pool = fuzz_accounts.pool.get_or_create_account(
            self.accounts.pool,
            client,
            mint,
            vault_type,
            0,
            None,
            None,
            0,
            None,
        );
        let reserve = fuzz_accounts.reserve.get_or_create_account(
            self.accounts.reserve,
            client,
            mint,
            vault_type,
            0,
            None,
            None,
            0,
            None,
        );

        //let token_program = fuzz_accounts.token_program.get(account_id)
        let signers = vec![admin.clone(), payer.clone()];
        let acc_meta = vault::accounts::NewVaultType {
            vault_type: vault_type,
            authority: admin.pubkey(),
            mint: mint,
            pool: pool,
            reserve: reserve,
            payer: payer.pubkey(),
            system_program: solana_sdk::system_program::ID,
            token_program: anchor_spl::token::ID,
        }
        .to_account_metas(None);
        Ok((signers, acc_meta))
    }
}
///IxOps implementation for `NominateVaultTypeAuthority` with all required
/// functions.
impl IxOps for NominateVaultTypeAuthority {
    type IxData = vault::instruction::NominateVaultTypeAuthority;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        vault::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data = vault::instruction::NominateVaultTypeAuthority {};
        Ok(data)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let admin = fuzz_accounts.authority.get(self.accounts.authority);
        let vault_type = fuzz_accounts.vault_type.get(self.accounts.vault_type);
        let new_authority = fuzz_accounts.new_authority.get_or_create_account(
            self.accounts.new_authority,
            client,
            10 * LAMPORTS_PER_SOL,
        );
        let signers = vec![admin.clone()];
        let acc_meta = vault::accounts::NominateVaultTypeAuthority {
            vault_type: vault_type,
            authority: admin.pubkey(),
            new_authority: new_authority.pubkey(),
        }
        .to_account_metas(None);
        Ok((signers, acc_meta))
    }
}
///IxOps implementation for `RollOverVaultType` with all required functions.
impl IxOps for RollOverVaultType {
    type IxData = vault::instruction::RollOverVaultType;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        vault::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data = vault::instruction::RollOverVaultType {};
        Ok(data)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        _client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let vault_type = fuzz_accounts.vault_type.get(self.accounts.vault_type);

        let signers = vec![];
        let acc_meta = vault::accounts::RollOverVaultType {
            vault_type,
        }
        .to_account_metas(None);
        Ok((signers, acc_meta))
    }
}
///IxOps implementation for `TransferVaultTypeToken` with all required
/// functions.
impl IxOps for TransferVaultTypeToken {
    type IxData = vault::instruction::TransferVaultTypeToken;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        vault::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data = vault::instruction::TransferVaultTypeToken {
            amount: self.data.amount,
        };
        Ok(data)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let authority = fuzz_accounts.authority.get(self.accounts.authority);
        let vault_type = fuzz_accounts.vault_type.get(self.accounts.vault_type);
        let mint = fuzz_accounts.mint.get(self.accounts.mint);
        let source = fuzz_accounts.pool.get(self.accounts.source);
        let destination = fuzz_accounts.destination.get_or_create_account(
            self.accounts.destination,
            client,
            mint,
            authority.pubkey(),
            0,
            None,
            None,
            0,
            None,
        );
        let signers = vec![authority.clone()];
        let acc_meta = vault::accounts::TransferVaultTypeToken {
            vault_type,
            authority: authority.pubkey(),
            source,
            destination,
            mint,
            token_program: anchor_spl::token::ID,
        }
        .to_account_metas(None);
        Ok((signers, acc_meta))
    }
}
///IxOps implementation for `Withdraw` with all required functions.
impl IxOps for Withdraw {
    type IxData = vault::instruction::Withdraw;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        vault::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data = vault::instruction::Withdraw {
            amount: self.data.amount,
        };
        Ok(data)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let user_authority = fuzz_accounts
            .user_authority
            .get(self.accounts.user_authority);
        let vault_type = fuzz_accounts.vault_type.get(self.accounts.vault_type);
        let vault = fuzz_accounts.vault.get(self.accounts.vault);
        let mint = fuzz_accounts.mint.get(self.accounts.mint);
        let pool = fuzz_accounts.pool.get(self.accounts.pool);
        let reserve = fuzz_accounts.reserve.get(self.accounts.reserve);
        let to = fuzz_accounts.to.get_or_create_account(
            self.accounts.to,
            client,
            mint,
            user_authority.pubkey(),
            0,
            None,
            None,
            0,
            None,
        );
        let signers = vec![user_authority.clone()];
        let acc_meta = vault::accounts::Withdraw {
            vault,
            vault_type,
            user_authority: user_authority.pubkey(),
            mint,
            pool,
            reserve: reserve,
            to: to,
            token_program: anchor_spl::token::ID,
        }
        .to_account_metas(None);
        Ok((signers, acc_meta))
    }
}
/// Check supported AccountsStorages at
/// https://ackee.xyz/trident/docs/latest/features/account-storages/
#[derive(Default)]
pub struct FuzzAccounts {
    authority: AccountsStorage<KeypairStore>,
    destination: AccountsStorage<TokenStore>,
    from: AccountsStorage<TokenStore>,
    mint: AccountsStorage<MintStore>,
    new_authority: AccountsStorage<KeypairStore>,
    payer: AccountsStorage<KeypairStore>,
    pool: AccountsStorage<TokenStore>,
    reserve: AccountsStorage<TokenStore>,
    // source: AccountsStorage<TokenStore>,
    //system_program: AccountsStorage<todo!()>,
    to: AccountsStorage<TokenStore>,
    //token_program: AccountsStorage<todo!()>,
    user_authority: AccountsStorage<KeypairStore>,
    vault: AccountsStorage<PdaStore>,
    vault_type: AccountsStorage<PdaStore>,
}
