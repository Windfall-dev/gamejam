use solana_program::program_pack::Pack;
use solana_program_test::tokio;
use solana_sdk::signer::Signer;

#[allow(dead_code)]
mod common;

use common::helpers::*;
use common::utils::*;

// test scenario in which authority change occurs
#[tokio::test]
async fn test_authority_change() {
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
    assert_eq!(vault_type_data.is_locked, false);

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

    // lock vault type

    let result = helper.lock_vault_type(vault_type, &admin, true).await;
    assert!(result.is_ok());

    let vault_type_data = helper.get_vault_type(&vault_type).await.unwrap();
    assert_eq!(vault_type_data.is_locked, true);

    // update vault type authority

    let new_authority = generate_signer();

    let result = helper
        .nominate_vault_type_authority(vault_type, &admin, new_authority.pubkey())
        .await;
    assert!(result.is_ok());

    let vault_type_data = helper.get_vault_type(&vault_type).await.unwrap();
    assert_eq!(vault_type_data.authority, admin.pubkey());
    assert_eq!(vault_type_data.pending_authority, Some(new_authority.pubkey()));

    // accept vault type authority to finalize the change

    let result = helper.accept_vault_type_authority(vault_type, &new_authority).await;
    assert!(result.is_ok());

    let vault_type_data = helper.get_vault_type(&vault_type).await.unwrap();
    assert_eq!(vault_type_data.authority, new_authority.pubkey());
    assert_eq!(vault_type_data.pending_authority, None);

    // unlock vault type (fail due to authority mismatch)

    let result = helper.lock_vault_type(vault_type, &admin, false).await;

    // an error must occur and it should be "Error Code: InvalidStatus. Error Number: 6002. Error Message: Invalid status" which is error code 0x1772
    let err_string = result.unwrap_err().to_string();
    assert!(
        err_string.contains("0x7d1"), // ConstraintHasOne
        "Unexpected error: {}",
        err_string
    );

    // unlock vault type

    let result = helper
        .lock_vault_type(vault_type, &new_authority, false)
        .await;
    assert!(result.is_ok());

    let vault_type_data = helper.get_vault_type(&vault_type).await.unwrap();
    assert_eq!(vault_type_data.is_locked, false);
}
