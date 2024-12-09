use solana_program::program_pack::Pack;
use solana_program_test::tokio;
use solana_sdk::signer::Signer;

#[allow(dead_code)]
mod common;

use common::helpers::*;
use common::utils::*;
use solana_sdk::transaction::Transaction;

// Normal test scenario with instant deactivation enabled
#[tokio::test]
async fn test_normal() {
    let mut helper = TestHelper::new(10).await;

    let admin = generate_signer();

    // new_vault_type (prepare)
    // - create new mint
    // - create pool/reserve token accounts for the new vault type which will be created soon

    let SetupForVaultTypeResult {
        vault_type,
        mint_authority,
        mint,
        pool,
        reserve,
        decimals,
    } = helper.setup_for_vault_type(&admin, None).await;

    assert!(helper
        .context
        .banks_client
        .get_account(vault_type)
        .await
        .unwrap()
        .is_none());

    let mint_account = helper
        .context
        .banks_client
        .get_account(mint)
        .await
        .unwrap()
        .unwrap();
    let mint_data = spl_token::state::Mint::unpack(&mint_account.data).unwrap();
    assert_eq!(mint_data.mint_authority.unwrap(), mint_authority.pubkey());
    assert_eq!(mint_data.decimals, decimals);

    // new_vault_type

    let season_start = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let season_duration = 60 * 60 * 24 * 14;
    let deactivation_lock_window = 60 * 60 * 24 * 3;
    let cooldown_window = 60 * 60 * 6;
    let max_deposit_per_user = 0;
    let instant_deactivation = true;

    let result = helper
        .new_vault_type(
            &admin,
            vault_type,
            mint,
            pool,
            reserve,
            spl_token::id(),
            season_start,
            season_duration,
            deactivation_lock_window,
            cooldown_window,
            max_deposit_per_user,
            instant_deactivation,
        )
        .await;

    assert!(result.is_ok());

    let vault_type_data = helper.get_vault_type(&vault_type).await.unwrap();

    assert_eq!(vault_type_data.authority, admin.pubkey());
    assert_eq!(vault_type_data.mint, mint);
    assert_eq!(vault_type_data.pool, pool);
    assert_eq!(vault_type_data.reserve, reserve);
    assert_eq!(vault_type_data.season_start, season_start);
    assert_eq!(vault_type_data.season_duration, season_duration);
    assert_eq!(
        vault_type_data.deactivation_lock_window,
        deactivation_lock_window
    );
    assert_eq!(vault_type_data.cooldown_window, cooldown_window);
    assert_eq!(vault_type_data.max_deposit_per_user, max_deposit_per_user);
    assert_eq!(vault_type_data.instant_deactivation, instant_deactivation);

    // check token accounts
    let pool_account = helper
        .context
        .banks_client
        .get_account(pool)
        .await
        .unwrap()
        .unwrap();
    let pool_account_data = spl_token::state::Account::unpack(&pool_account.data).unwrap();
    assert_eq!(pool_account_data.mint, mint);
    assert_eq!(pool_account_data.owner, vault_type);
    assert_eq!(pool_account_data.amount, 0);

    let reserve_account = helper
        .context
        .banks_client
        .get_account(reserve)
        .await
        .unwrap()
        .unwrap();
    let reserve_account_data = spl_token::state::Account::unpack(&reserve_account.data).unwrap();
    assert_eq!(reserve_account_data.mint, mint);
    assert_eq!(reserve_account_data.owner, vault_type);
    assert_eq!(reserve_account_data.amount, 0);

    // new_vault

    let user = generate_signer();
    let (vault, vault_bump) = vault::states::Vault::pda(&vault_type, &user.pubkey());

    let result = helper.new_vault(vault, &user, vault_type).await;

    assert!(result.is_ok());

    let vault_data = helper.get_vault(&vault).await.unwrap();
    assert_eq!(vault_data.user_authority, user.pubkey());
    assert_eq!(vault_data.vault_type, vault_type);
    assert_eq!(vault_data.amount, 0);
    assert_eq!(vault_data.inactive_at, 0);
    assert_eq!(vault_data.status, vault::states::VaultStatus::Inactive);
    assert_eq!(vault_data.bump, vault_bump);

    // deposit (prepare)
    // - mint token to user token account

    let user_token_account = create_ata(&mut helper.context, &mint, &user.pubkey(), &helper.payer)
        .await
        .unwrap();
    let mint_amount = 10_000_000_000;

    mint_to(
        &mut helper.context,
        &mint,
        &user_token_account,
        &mint_authority,
        mint_amount,
        &helper.payer,
    )
    .await
    .unwrap();

    assert_eq!(
        get_token_amount(&mut helper.context, &user_token_account)
            .await
            .unwrap(),
        mint_amount
    );

    // deposit

    let deposit_amount = 1_000_000_000;

    let result = helper
        .deposit(
            vault,
            &user,
            vault_type,
            mint,
            pool,
            user_token_account,
            deposit_amount,
        )
        .await;

    assert!(result.is_ok());

    let vault_data = helper.get_vault(&vault).await.unwrap();
    assert_eq!(vault_data.user_authority, user.pubkey());
    assert_eq!(vault_data.vault_type, vault_type);
    assert_eq!(vault_data.amount, deposit_amount);
    assert_eq!(vault_data.status, vault::states::VaultStatus::Active);

    assert_eq!(
        get_token_amount(&mut helper.context, &user_token_account)
            .await
            .unwrap(),
        mint_amount - deposit_amount
    );

    assert_eq!(
        get_token_amount(&mut helper.context, &pool).await.unwrap(),
        deposit_amount
    );

    // transfer_vault_type_token
    // - create another empty token account owned by admin
    // - withdraw from the pool to this token account
    // - assert result
    // - send back token to the pool
    // - assert result

    let admin_token_account =
        create_token_account(&mut helper.context, &mint, &admin.pubkey(), &helper.payer)
            .await
            .unwrap();
    assert_eq!(
        get_token_amount(&mut helper.context, &admin_token_account)
            .await
            .unwrap(),
        0
    );

    let external_transfer_amount = 700_000_000;

    let result = helper
        .transfer_vault_type_token(
            vault_type,
            &admin,
            mint,
            pool,
            admin_token_account,
            external_transfer_amount,
        )
        .await;

    assert!(result.is_ok());

    // Verify pool and admin's token account balance
    assert_eq!(
        get_token_amount(&mut helper.context, &pool).await.unwrap(),
        deposit_amount - external_transfer_amount
    );
    assert_eq!(
        get_token_amount(&mut helper.context, &admin_token_account)
            .await
            .unwrap(),
        external_transfer_amount
    );

    // Send back token to pool account
    let transfer_instruction = spl_token::instruction::transfer(
        &spl_token::id(),
        &admin_token_account,
        &pool,
        &admin.pubkey(),
        &[],
        external_transfer_amount,
    )
    .unwrap();

    let mut transaction =
        Transaction::new_with_payer(&[transfer_instruction], Some(&helper.payer.pubkey()));
    let blockhash = helper
        .context
        .banks_client
        .get_latest_blockhash()
        .await
        .unwrap();
    transaction.sign(&[&helper.payer, &admin], blockhash);
    helper
        .context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    // Verify balances after transfer back
    assert_eq!(
        get_token_amount(&mut helper.context, &pool).await.unwrap(),
        deposit_amount
    );
    assert_eq!(
        get_token_amount(&mut helper.context, &admin_token_account)
            .await
            .unwrap(),
        0
    );

    // withdraw (fail due to active status)

    let result = helper
        .withdraw(
            vault,
            &user,
            vault_type,
            mint,
            pool,
            reserve,
            user_token_account,
            deposit_amount,
        )
        .await;

    // an error must occur and it should be "Error Code: InvalidStatus. Error Number: 6002. Error Message: Invalid status" which is error code 0x1772
    let err_string = result.unwrap_err().to_string();
    assert!(
        err_string.contains("0x1772"),
        "Unexpected error: {}",
        err_string
    );

    // deactivate

    let result = helper.deactivate(vault, &user, vault_type).await;

    assert!(result.is_ok());

    let vault_data = helper.get_vault(&vault).await.unwrap();
    assert_eq!(vault_data.status, vault::states::VaultStatus::Inactive);

    // withdraw

    let withdraw_amount = 400_000_000;
    let remaining_amount = deposit_amount - withdraw_amount;

    let result = helper
        .withdraw(
            vault,
            &user,
            vault_type,
            mint,
            pool,
            reserve,
            user_token_account,
            withdraw_amount,
        )
        .await;

    assert!(result.is_ok());

    // Verify balances after withdrawal
    assert_eq!(
        get_token_amount(&mut helper.context, &pool).await.unwrap(),
        remaining_amount
    );
    assert_eq!(
        get_token_amount(&mut helper.context, &user_token_account)
            .await
            .unwrap(),
        mint_amount - remaining_amount
    );

    // close vault (fail due to remaining deposit)

    let result = helper.close_vault(vault, &user, vault_type).await;

    // an error must occur and it should be "Error Code: DepositRemaining. Error Number: 6004. Error Message: Deposit remaining" which is error code 0x1774
    let err_string = result.unwrap_err().to_string();
    assert!(
        err_string.contains("0x1774"),
        "Unexpected error: {}",
        err_string
    );

    // activate

    let result = helper.activate(vault, &user, vault_type).await;

    assert!(result.is_ok());

    let vault_data = helper.get_vault(&vault).await.unwrap();
    assert_eq!(vault_data.status, vault::states::VaultStatus::Active);

    // withdraw (remaining) (fail due to active status)

    let result = helper
        .withdraw(
            vault,
            &user,
            vault_type,
            mint,
            pool,
            reserve,
            user_token_account,
            remaining_amount,
        )
        .await;

    // an error must occur and it should be "Error Code: InvalidStatus. Error Number: 6002. Error Message: Invalid status" which is error code 0x1772
    let err_string = result.unwrap_err().to_string();
    assert!(
        err_string.contains("0x1772"),
        "Unexpected error: {}",
        err_string
    );

    // deactivate

    advance_slot(&mut helper.context).await;

    let result = helper.deactivate(vault, &user, vault_type).await;

    assert!(result.is_ok());

    let vault_data = helper.get_vault(&vault).await.unwrap();
    assert_eq!(vault_data.status, vault::states::VaultStatus::Inactive);

    // withdraw the remaining token

    let result = helper
        .withdraw(
            vault,
            &user,
            vault_type,
            mint,
            pool,
            reserve,
            user_token_account,
            remaining_amount,
        )
        .await;

    assert!(result.is_ok());

    // Verify balances after withdrawal
    assert_eq!(
        get_token_amount(&mut helper.context, &pool).await.unwrap(),
        0
    );
    assert_eq!(
        get_token_amount(&mut helper.context, &user_token_account)
            .await
            .unwrap(),
        mint_amount
    );

    // close vault (need slot advancement)

    advance_slot(&mut helper.context).await;

    let result = helper.close_vault(vault, &user, vault_type).await;

    assert!(result.is_ok());

    // Verify vault account is closed
    let vault_account = helper
        .context
        .banks_client
        .get_account(vault)
        .await
        .unwrap();
    assert!(vault_account.is_none(), "Vault account should not exist");

    // close vault type

    let result = helper
        .close_vault_type(vault_type, &admin, pool, reserve)
        .await;

    assert!(result.is_ok());

    // Verify vault type account is closed
    let vault_type_account = helper
        .context
        .banks_client
        .get_account(vault_type)
        .await
        .unwrap();
    assert!(
        vault_type_account.is_none(),
        "Vault type account should not exist"
    );

    // Verify pool account is closed
    let pool_account = helper.context.banks_client.get_account(pool).await.unwrap();
    assert!(pool_account.is_none(), "Pool account should not exist");

    // Verify reserve account is closed
    let reserve_account = helper.context.banks_client.get_account(reserve).await.unwrap();
    assert!(reserve_account.is_none(), "Reserve account should not exist");
}
