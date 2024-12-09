use anchor_lang::{AccountDeserialize, InstructionData, ToAccountMetas};
use solana_program_test::{processor, BanksClientError, ProgramTest, ProgramTestContext};
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::Keypair, signature::Signer};

use super::utils::*;
use crate::convert_entry;

const PROGRAM_ID: Pubkey = vault::ID_CONST; // Define the program ID constant.
const PROGRAM_NAME: &str = "vault"; // Define the program name.

pub struct TestHelper {
    pub context: ProgramTestContext,
    pub payer: Keypair,
}

pub struct SetupForVaultTypeResult {
    pub vault_type: Pubkey,
    pub mint_authority: Keypair,
    pub mint: Pubkey,
    pub pool: Pubkey,
    pub reserve: Pubkey,
    pub decimals: u8,
}

pub struct SetupForVaultTypeWithNativeResult {
    pub vault_type: Pubkey,
    pub mint: Pubkey,
    pub pool: Pubkey,
    pub reserve: Pubkey,
    pub decimals: u8,
}

impl TestHelper {
    pub async fn new(sol_amount: u64) -> Self {
        let mut program_test = ProgramTest::new(
            PROGRAM_NAME,
            PROGRAM_ID,
            processor!(convert_entry!(vault::entry)),
        );

        let payer = generate_signer();
        airdrop(&mut program_test, payer.pubkey(), sol_amount * LAMPORTS_PER_SOL);

        let context = program_test.start_with_context().await;

        Self { context, payer }
    }

    pub async fn get_vault_type(
        &mut self,
        vault_type: &Pubkey,
    ) -> anchor_lang::Result<vault::states::VaultType> {
        let vault_type_account = self
            .context
            .banks_client
            .get_account(*vault_type)
            .await
            .unwrap()
            .unwrap();
        vault::states::VaultType::try_deserialize(&mut &vault_type_account.data[..])
    }

    pub async fn get_vault(&mut self, vault: &Pubkey) -> anchor_lang::Result<vault::states::Vault> {
        let vault_account = self
            .context
            .banks_client
            .get_account(*vault)
            .await
            .unwrap()
            .unwrap();
        vault::states::Vault::try_deserialize(&mut &vault_account.data[..])
    }

    pub async fn setup_for_vault_type(
        &mut self,
        admin: &Keypair,
        decimals: Option<u8>,
    ) -> SetupForVaultTypeResult {
        let mint_authority = generate_signer();
        let mint = mint_authority.pubkey();

        let (vault_type, _bump) = vault::states::VaultType::pda(&mint, &admin.pubkey());
        let actual_decimals = decimals.unwrap_or(9);

        create_mint(
            &mut self.context,
            &mint_authority,
            &self.payer,
            actual_decimals,
        )
        .await;

        // let (pool, _pool_bump) = Pubkey::find_program_address(
        //     &[
        //         POOL_PREFIX,
        //         vault_type.as_ref()
        //     ],
        //     &vault::id()
        // );

        // let (reserve, _reserve_bump) = Pubkey::find_program_address(
        //     &[
        //         RESERVE_PREFIX,
        //         vault_type.as_ref()
        //     ],
        //     &vault::id()
        // );

        let pool = create_token_account(&mut self.context, &mint, &vault_type, &self.payer).await.unwrap();
        let reserve = create_token_account(&mut self.context, &mint, &vault_type, &self.payer).await.unwrap();

        return SetupForVaultTypeResult {
            vault_type,
            mint_authority,
            mint,
            pool,
            reserve,
            decimals: actual_decimals,
        };
    }

    pub async fn setup_for_vault_type_with_native(
        &mut self,
        admin: &Keypair,
    ) -> SetupForVaultTypeWithNativeResult {
        let mint = spl_token::native_mint::id();

        let (vault_type, _bump) = vault::states::VaultType::pda(&mint, &admin.pubkey());
        let actual_decimals = 9;

        // let (pool, _pool_bump) = Pubkey::find_program_address(
        //     &[
        //         POOL_PREFIX,
        //         vault_type.as_ref()
        //     ],
        //     &vault::id()
        // );

        // let (reserve, _reserve_bump) = Pubkey::find_program_address(
        //     &[
        //         RESERVE_PREFIX,
        //         vault_type.as_ref()
        //     ],
        //     &vault::id()
        // );

        let pool = create_token_account(&mut self.context, &mint, &vault_type, &self.payer).await.unwrap();
        let reserve = create_token_account(&mut self.context, &mint, &vault_type, &self.payer).await.unwrap();

        // let pool = get_ata_address(&mint, &vault_type);

        // let pool = create_ata(&mut self.context, &mint, &vault_type, &self.payer)
        //     .await
        //     .unwrap();

        return SetupForVaultTypeWithNativeResult {
            vault_type,
            mint,
            pool,
            reserve,
            decimals: actual_decimals,
        };
    }

    pub async fn new_vault_type(
        &mut self,
        authority: &Keypair,
        vault_type: Pubkey,
        mint: Pubkey,
        pool: Pubkey,
        reserve: Pubkey,
        token_program: Pubkey,
        season_start: i64,
        season_duration: i64,
        deactivation_lock_window: i64,
        cooldown_window: i64,
        max_deposit_per_user: u64,
        instant_deactivation: bool,
    ) -> Result<(), BanksClientError> {
        let ix = vault::instruction::NewVaultType {
            season_start,
            season_duration,
            deactivation_lock_window,
            cooldown_window,
            max_deposit_per_user,
            instant_deactivation,
        };

        let accounts = vault::accounts::NewVaultType {
            authority: authority.pubkey(),
            vault_type,
            mint,
            pool,
            reserve,
            payer: self.payer.pubkey(),
            system_program: solana_sdk::system_program::id(),
            token_program: token_program,
        };

        let instruction = Instruction {
            program_id: PROGRAM_ID,
            accounts: accounts.to_account_metas(Some(true)),
            data: ix.data(),
        };

        let signers = vec![&self.payer, &authority];

        process_instruction(
            &mut self.context,
            instruction,
            &self.payer.pubkey(),
            signers,
        )
        .await
    }   

    pub async fn nominate_vault_type_authority(
        &mut self,
        vault_type: Pubkey,
        authority: &Keypair,
        new_authority: Pubkey,
    ) -> Result<(), BanksClientError> {
        let ix = vault::instruction::NominateVaultTypeAuthority {};

        let accounts = vault::accounts::NominateVaultTypeAuthority {
            vault_type,
            authority: authority.pubkey(),
            new_authority: new_authority,
        };

        let instruction = Instruction {
            program_id: PROGRAM_ID,
            accounts: accounts.to_account_metas(Some(true)),
            data: ix.data(),
        };

        let signers = vec![&self.payer, &authority];

        process_instruction(
            &mut self.context,
            instruction,
            &self.payer.pubkey(),
            signers,
        )
        .await
    }

    pub async fn accept_vault_type_authority(
        &mut self,
        vault_type: Pubkey,
        new_authority: &Keypair,
    ) -> Result<(), BanksClientError> {
        let ix = vault::instruction::AcceptVaultTypeAuthority {};

        let accounts = vault::accounts::AcceptVaultTypeAuthority {
            vault_type,
            new_authority: new_authority.pubkey(),
        };

        let instruction = Instruction {
            program_id: PROGRAM_ID,
            accounts: accounts.to_account_metas(Some(true)),
            data: ix.data(),
        };

        let signers = vec![&self.payer, &new_authority];

        process_instruction(
            &mut self.context,
            instruction,
            &self.payer.pubkey(),
            signers,
        )
        .await
    }

    pub async fn lock_vault_type(
        &mut self,
        vault_type: Pubkey,
        authority: &Keypair,
        is_locked: bool,
    ) -> Result<(), BanksClientError> {
        let ix = vault::instruction::LockVaultType { is_locked };

        let accounts = vault::accounts::LockVaultType {
            vault_type,
            authority: authority.pubkey(),
        };

        let instruction = Instruction {
            program_id: PROGRAM_ID,
            accounts: accounts.to_account_metas(Some(true)),
            data: ix.data(),
        };

        let signers = vec![&self.payer, &authority];

        process_instruction(
            &mut self.context,
            instruction,
            &self.payer.pubkey(),
            signers,
        )
        .await
    }

    pub async fn new_vault(
        &mut self,
        vault: Pubkey,
        user_authority: &Keypair,
        vault_type: Pubkey,
    ) -> Result<(), BanksClientError> {
        let ix = vault::instruction::NewVault {};

        let accounts = vault::accounts::NewVault {
            vault,
            user_authority: user_authority.pubkey(),
            vault_type,
            payer: self.payer.pubkey(),
            system_program: solana_sdk::system_program::id(),
        };

        let instruction = Instruction {
            program_id: PROGRAM_ID,
            accounts: accounts.to_account_metas(Some(true)),
            data: ix.data(),
        };

        let signers = vec![&self.payer, &user_authority];

        process_instruction(
            &mut self.context,
            instruction,
            &self.payer.pubkey(),
            signers,
        )
        .await
    }

    pub async fn deposit(
        &mut self,
        vault: Pubkey,
        user_authority: &Keypair,
        vault_type: Pubkey,
        mint: Pubkey,
        pool: Pubkey,
        from: Pubkey,
        amount: u64,
    ) -> Result<(), BanksClientError> {
        let ix = vault::instruction::Deposit { amount };

        let accounts = vault::accounts::Deposit {
            vault,
            vault_type,
            user_authority: user_authority.pubkey(),
            mint,
            pool,
            from,
            token_program: spl_token::id(),
        };

        let instruction = Instruction {
            program_id: PROGRAM_ID,
            accounts: accounts.to_account_metas(Some(true)),
            data: ix.data(),
        };

        let signers = vec![&self.payer, &user_authority];

        process_instruction(
            &mut self.context,
            instruction,
            &self.payer.pubkey(),
            signers,
        )
        .await
    }

    pub async fn transfer_vault_type_token(
        &mut self,
        vault_type: Pubkey,
        authority: &Keypair,
        mint: Pubkey,
        source: Pubkey,
        destination: Pubkey,
        amount: u64,
    ) -> Result<(), BanksClientError> {
        let ix = vault::instruction::TransferVaultTypeToken { amount };

        let accounts = vault::accounts::TransferVaultTypeToken {
            vault_type,
            authority: authority.pubkey(),
            mint,
            source,
            destination,
            token_program: spl_token::id(),
        };

        let instruction = Instruction {
            program_id: PROGRAM_ID,
            accounts: accounts.to_account_metas(Some(true)),
            data: ix.data(),
        };

        let signers = vec![&self.payer, &authority];

        process_instruction(
            &mut self.context,
            instruction,
            &self.payer.pubkey(),
            signers,
        )
        .await
    }

    pub async fn activate(
        &mut self,
        vault: Pubkey,
        user_authority: &Keypair,
        vault_type: Pubkey,
    ) -> Result<(), BanksClientError> {
        let ix = vault::instruction::Activate {};

        let accounts = vault::accounts::Activate {
            vault,
            vault_type,
            user_authority: user_authority.pubkey(),
        };

        let instruction = Instruction {
            program_id: PROGRAM_ID,
            accounts: accounts.to_account_metas(Some(true)),
            data: ix.data(),
        };

        let signers = vec![&self.payer, &user_authority];

        process_instruction(
            &mut self.context,
            instruction,
            &self.payer.pubkey(),
            signers,
        )
        .await
    }

    pub async fn deactivate(
        &mut self,
        vault: Pubkey,
        user_authority: &Keypair,
        vault_type: Pubkey,
    ) -> Result<(), BanksClientError> {
        let ix = vault::instruction::Deactivate {};

        let accounts = vault::accounts::Deactivate {
            vault,
            vault_type,
            user_authority: user_authority.pubkey(),
        };

        let instruction = Instruction {
            program_id: PROGRAM_ID,
            accounts: accounts.to_account_metas(Some(true)),
            data: ix.data(),
        };

        let signers = vec![&self.payer, &user_authority];

        process_instruction(
            &mut self.context,
            instruction,
            &self.payer.pubkey(),
            signers,
        )
        .await
    }

    pub async fn withdraw(
        &mut self,
        vault: Pubkey,
        user_authority: &Keypair,
        vault_type: Pubkey,
        mint: Pubkey,
        pool: Pubkey,
        reserve: Pubkey,
        to: Pubkey,
        amount: u64,
    ) -> Result<(), BanksClientError> {
        let ix = vault::instruction::Withdraw { amount };

        let accounts = vault::accounts::Withdraw {
            vault,
            vault_type,
            user_authority: user_authority.pubkey(),
            mint,
            pool,
            reserve,
            to,
            token_program: spl_token::id(),
        };

        let instruction = Instruction {
            program_id: PROGRAM_ID,
            accounts: accounts.to_account_metas(Some(true)),
            data: ix.data(),
        };

        let signers = vec![&self.payer, &user_authority];

        process_instruction(
            &mut self.context,
            instruction,
            &self.payer.pubkey(),
            signers,
        )
        .await
    }

    pub async fn close_vault(
        &mut self,
        vault: Pubkey,
        user_authority: &Keypair,
        vault_type: Pubkey,
    ) -> Result<(), BanksClientError> {
        let ix = vault::instruction::CloseVault {};

        let accounts = vault::accounts::CloseVault {
            vault,
            vault_type,
            user_authority: user_authority.pubkey(),
            payer: self.payer.pubkey(),
        };

        let instruction = Instruction {
            program_id: PROGRAM_ID,
            accounts: accounts.to_account_metas(Some(true)),
            data: ix.data(),
        };

        let signers = vec![&self.payer, &user_authority];

        process_instruction(
            &mut self.context,
            instruction,
            &self.payer.pubkey(),
            signers,
        )
        .await
    }

    pub async fn close_vault_type(
        &mut self,
        vault_type: Pubkey,
        authority: &Keypair,
        pool: Pubkey,
        reserve: Pubkey,
    ) -> Result<(), BanksClientError> {
        let ix = vault::instruction::CloseVaultType {};

        let accounts = vault::accounts::CloseVaultType {
            vault_type,
            authority: authority.pubkey(),
            payer: self.payer.pubkey(),
            pool,
            reserve,
            token_program: spl_token::id(),
        };
        
        let instruction = Instruction {
            program_id: PROGRAM_ID,
            accounts: accounts.to_account_metas(Some(true)),
            data: ix.data(),
        };

        let signers = vec![&self.payer, &authority];

        process_instruction(
            &mut self.context,
            instruction,
            &self.payer.pubkey(),
            signers,
        )
        .await
    }
}
