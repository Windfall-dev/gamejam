use solana_program::program_pack::Pack;
use solana_program_test::tokio;
use solana_sdk::signer::Signer;

#[allow(dead_code)]
mod common;

use common::helpers::*;
use common::utils::*;

// test scenario where withdrawals fail a few times because of the season duration and the status of the vault
#[tokio::test]
async fn test_season01() {
    let mut helper = TestHelper::new(10).await;

    let admin = generate_signer();

    // new_vault_type (prepare)

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
    let cooldown_window = 60 * 60 * 6;
    let deactivation_lock_window = 60 * 60 * 24 * 3;
    let max_deposit_per_user = 0;
    let instant_deactivation = false;

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

    // lock vault type

    let result = helper.lock_vault_type(vault_type, &admin, true).await;
    assert!(result.is_ok());

    // deactivate (fail due to locked vault type)

    let result = helper.deactivate(vault, &user, vault_type).await;
    // an error must occur and its error code 0x1773
    let err_string = result.unwrap_err().to_string();
    assert!(
        err_string.contains("0x1773"),
        "Unexpected error: {}",
        err_string
    );

    // unlock vault type

    let result = helper.lock_vault_type(vault_type, &admin, false).await;
    assert!(result.is_ok());

    // deactivate

    advance_slot(&mut helper.context).await;

    let result = helper.deactivate(vault, &user, vault_type).await;

    assert!(result.is_ok());

    let vault_data = helper.get_vault(&vault).await.unwrap();
    assert_eq!(vault_data.status, vault::states::VaultStatus::Deactivating);

    // withdraw (fail due to active status)

    advance_slot(&mut helper.context).await;

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

    // forward time to half of the season passed and withdraw (fail due to active status)

    forward_time(&mut helper.context, season_duration / 2).await;

    advance_slot(&mut helper.context).await;

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

    // forward time past the season end and withdraw (should succeed this time)

    advance_slot(&mut helper.context).await;

    forward_time(&mut helper.context, season_duration / 2 + 5).await;

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

    assert!(result.is_ok());
}
