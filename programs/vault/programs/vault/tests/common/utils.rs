use anchor_lang::prelude::AccountInfo;
use solana_program_test::{BanksClientError, ProgramTest, ProgramTestContext};
use solana_sdk::system_instruction;
use solana_sdk::{
    account::Account, clock::Clock, entrypoint::ProgramResult, instruction::Instruction,
    program_pack::Pack, pubkey::Pubkey, signature::Keypair, signature::Signer,
    transaction::Transaction,
};

use solana_program::rent::Rent;
use spl_associated_token_account::get_associated_token_address;
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token::{instruction::initialize_mint, state::Mint};

// Type alias for the entry function pointer used to convert the entry function into a ProcessInstruction function pointer.
pub type ProgramEntry = for<'info> fn(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    instruction_data: &[u8],
) -> ProgramResult;

// Macro to convert the entry function into a ProcessInstruction function pointer.
#[macro_export]
macro_rules! convert_entry {
    ($entry:expr) => {
        // Use unsafe block to perform memory transmutation.
        unsafe {
            core::mem::transmute::<ProgramEntry, solana_sdk::entrypoint::ProcessInstruction>($entry)
        }
    };
}

// Function to generate a new keypair for signing transactions.
pub fn generate_signer() -> solana_sdk::signer::keypair::Keypair {
    solana_sdk::signer::keypair::Keypair::new()
}

// Function to add an account with the specified amount of lamports to the program test.
pub fn airdrop(program_test: &mut ProgramTest, address: Pubkey, amount: u64) {
    program_test.add_account(
        address,
        Account::new(amount, 0, &solana_sdk::system_program::ID),
    );
}

pub async fn create_mint(
    program_test_context: &mut ProgramTestContext,
    mint_keypair: &Keypair,
    payer: &Keypair,
    decimals: u8,
) {
    let banks_client = &mut program_test_context.banks_client;
    let rent = Rent::default();
    let mint_rent = rent.minimum_balance(Mint::LEN);
    let mint_account_instruction = solana_sdk::system_instruction::create_account(
        &payer.pubkey(),
        &mint_keypair.pubkey(),
        mint_rent,
        Mint::LEN as u64,
        &spl_token::id(),
    );
    let init_mint_instruction = initialize_mint(
        &spl_token::id(),
        &mint_keypair.pubkey(),
        &mint_keypair.pubkey(),
        Some(&mint_keypair.pubkey()),
        decimals,
    )
    .unwrap();

    let mut transaction = Transaction::new_with_payer(
        &[mint_account_instruction, init_mint_instruction],
        Some(&payer.pubkey()),
    );
    let bh = banks_client.get_latest_blockhash().await;
    transaction.sign(&[&payer, &mint_keypair], bh.unwrap());
    banks_client.process_transaction(transaction).await.unwrap();
}

pub async fn get_token_account(
    context: &mut ProgramTestContext,
    account: &Pubkey,
) -> Result<spl_token::state::Account, anchor_lang::prelude::ProgramError> {
    let token_account_data = context
        .banks_client
        .get_account(*account)
        .await
        .unwrap()
        .unwrap();
    spl_token::state::Account::unpack(&token_account_data.data)
}

pub async fn get_token_amount(
    context: &mut ProgramTestContext,
    account: &Pubkey,
) -> Result<u64, anchor_lang::prelude::ProgramError> {
    let account = get_token_account(context, account).await?;
    Ok(account.amount)
}

pub fn get_ata_address(
    mint: &Pubkey,
    owner: &Pubkey,
) -> Pubkey {
    get_associated_token_address(owner, mint)
}

pub async fn create_ata(
    program_test_context: &mut ProgramTestContext,
    mint: &Pubkey,
    owner: &Pubkey,
    payer: &Keypair,
) -> Result<Pubkey, BanksClientError> {
    let banks_client = &mut program_test_context.banks_client;

    // Create ATA
    let ata_address = get_associated_token_address(owner, mint);
    let create_ata_instruction =
        create_associated_token_account(&payer.pubkey(), owner, mint, &spl_token::id());

    let mut transaction =
        Transaction::new_with_payer(&[create_ata_instruction], Some(&payer.pubkey()));
    let bh = banks_client.get_latest_blockhash().await;
    transaction.sign(&[&payer], bh.unwrap());
    banks_client.process_transaction(transaction).await?;

    Ok(ata_address)
}

pub async fn create_token_account(
    program_test_context: &mut ProgramTestContext,
    mint: &Pubkey,
    owner: &Pubkey,
    payer: &Keypair,
) -> Result<Pubkey, BanksClientError> {
    let banks_client = &mut program_test_context.banks_client;

    // Create a new token account
    let token_account = Keypair::new();
    let rent = banks_client.get_rent().await.unwrap();
    let token_account_rent = rent.minimum_balance(spl_token::state::Account::LEN);

    let create_account_instruction = system_instruction::create_account(
        &payer.pubkey(),
        &token_account.pubkey(),
        token_account_rent,
        spl_token::state::Account::LEN as u64,
        &spl_token::id(),
    );

    let initialize_account_instruction = spl_token::instruction::initialize_account(
        &spl_token::id(),
        &token_account.pubkey(),
        mint,
        owner,
    )
    .unwrap();

    let mut transaction = Transaction::new_with_payer(
        &[create_account_instruction, initialize_account_instruction],
        Some(&payer.pubkey()),
    );
    let bh = banks_client.get_latest_blockhash().await;
    transaction.sign(&[&payer, &token_account], bh.unwrap());
    banks_client.process_transaction(transaction).await?;

    Ok(token_account.pubkey())
}

pub async fn mint_to(
    program_test_context: &mut ProgramTestContext,
    mint: &Pubkey,
    destination: &Pubkey,
    mint_authority: &Keypair,
    amount: u64,
    payer: &Keypair,
) -> Result<(), BanksClientError> {
    let banks_client = &mut program_test_context.banks_client;

    let mint_to_instruction = spl_token::instruction::mint_to(
        &spl_token::id(),
        mint,
        destination,
        &mint_authority.pubkey(),
        &[],
        amount,
    )
    .unwrap();

    let mut transaction =
        Transaction::new_with_payer(&[mint_to_instruction], Some(&payer.pubkey()));
    let bh = banks_client.get_latest_blockhash().await;
    transaction.sign(&[&payer, mint_authority], bh.unwrap());
    banks_client.process_transaction(transaction).await
}

// Function to process an instruction in the program test context and ensure it is finalized.
pub async fn process_instruction(
    program_test_context: &mut ProgramTestContext,
    instruction: Instruction,
    payer: &Pubkey,
    signers: Vec<&Keypair>,
) -> std::result::Result<(), solana_program_test::BanksClientError> {
    // Create a new transaction with the given instruction and payer.
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(payer));

    // Sign the transaction with the provided signers.
    transaction.sign(&signers, program_test_context.last_blockhash);

    // Process the transaction within the program test context with commitment level finalized.
    program_test_context
        .banks_client
        .process_transaction_with_commitment(
            transaction,
            solana_sdk::commitment_config::CommitmentLevel::Finalized,
        )
        .await
}

// Function to forward the program test context time by a specified number of seconds.
pub async fn forward_time(program_test_context: &mut ProgramTestContext, seconds: i64) {
    // Get the current clock state from the program test context.
    let mut clock = program_test_context
        .banks_client
        .get_sysvar::<Clock>()
        .await
        .unwrap();

    // Calculate the new timestamp after advancing time.
    let new_timestamp = clock.unix_timestamp + seconds;

    // Update the Clock instance with the new timestamp.
    clock.unix_timestamp = new_timestamp;

    // Update the sysvar in the program test context with the new Clock state.
    program_test_context.set_sysvar(&clock);
}

// Function to forward the program test context time by a specified number of seconds.
pub async fn advance_slot(program_test_context: &mut ProgramTestContext) {
    // Wait for the next slot.
    let slot = program_test_context.banks_client.get_root_slot().await.unwrap();
    program_test_context.warp_to_slot(slot+1).unwrap();
}
