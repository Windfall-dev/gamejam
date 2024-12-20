import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { createAccount, createAssociatedTokenAccount, createMint, getAssociatedTokenAddress, getOrCreateAssociatedTokenAccount, mintTo, transfer } from "@solana/spl-token";
import { Vault } from "../target/types/vault";
import { assert } from "chai";

describe("vault", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const isLocalnet = provider.connection.rpcEndpoint.includes('localhost') || provider.connection.rpcEndpoint.includes('127.0.0.1');

  const program = anchor.workspace.Vault as Program<Vault>;

  const wallet = anchor.workspace.Vault.provider.wallet;

  const adminKeypair = anchor.web3.Keypair.generate();
  const admin = adminKeypair.publicKey;

  const userAuthorityKeypair = anchor.web3.Keypair.generate();
  const userAuthority = userAuthorityKeypair.publicKey;

  const mintAuthority = anchor.web3.Keypair.generate();
  const decimals = 9;
  let tokenMint: PublicKey;

  before(async () => {
    if (isLocalnet) {
      console.log(`Executing tests on localnet`);

      const airdropSignature = await program.provider.connection.requestAirdrop(
        wallet.publicKey,
        1 * anchor.web3.LAMPORTS_PER_SOL
      );

      await program.provider.connection.getSignatureStatuses([airdropSignature]);
      console.log(`Airdropped 1 SOL to ${wallet.publicKey.toBase58()}`);

      await new Promise(resolve => setTimeout(resolve, 1000));

    } else {
      console.log(`Executing tests on devnet / mainnet`);
    }

    // Create a mock token mint for test
    tokenMint = await createMint(
      program.provider.connection,
      wallet.payer,
      mintAuthority.publicKey,
      null,
      decimals,
    );


  });

  // Create a new token account for the pool and reserve
  const poolTokenAccountKeypair = anchor.web3.Keypair.generate();
  const reserveTokenAccountKeypair = anchor.web3.Keypair.generate();

  // Season duration is 2 weeks. Create vault type with initial season starting more than 2 weeks ago.
  // After creating vault type, immediately test the roll over.
  const dateStart = Date.now();
  const seasonStart = new anchor.BN(dateStart / 1000 - 60 * 60 * 24 * 14 - 10);
  const seasonDuration = new anchor.BN(60 * 60 * 24 * 14);
  const deactivationLockWindow = new anchor.BN(60 * 60 * 24 * 3);
  const cooldownWindow = new anchor.BN(60 * 60 * 6);
  const maxDepositPerUser = new anchor.BN(1_000_000_000_000);

  it("new_vault_type", async () => {
    const [vaultType, vaultTypeBump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('vault_type'),
        tokenMint.toBuffer(),
        admin.toBuffer(),
      ],
      program.programId
    );

    // const [poolTokenAccount, _poolBump] = PublicKey.findProgramAddressSync(
    //   [
    //     anchor.utils.bytes.utf8.encode('pool'),
    //     vaultType.toBuffer(),
    //   ],
    //   program.programId
    // );
    // console.log(`pool account: ${poolTokenAccount.toBase58()}`);

    // const [reserveTokenAccount, _reserveBump] = PublicKey.findProgramAddressSync(
    //   [
    //     anchor.utils.bytes.utf8.encode('reserve'),
    //     vaultType.toBuffer(),
    //   ],
    //   program.programId
    // );
    // console.log(`reserve account: ${reserveTokenAccount.toBase58()}`);

    // const poolTokenAccount = await getAssociatedTokenAddress(
    //   tokenMint,
    //   vaultType,
    //   true,
    // );
    // console.log(`ATA pool account: ${poolTokenAccount.toBase58()}`);

    const pool = await createAccount(provider.connection, wallet.payer, tokenMint, vaultType, poolTokenAccountKeypair);
    const reserve = await createAccount(provider.connection, wallet.payer, tokenMint, vaultType, reserveTokenAccountKeypair);

    const instantDeactivation = true;

    const txSig = await program.methods.newVaultType(
      seasonStart,
      seasonDuration,
      deactivationLockWindow,
      cooldownWindow,
      maxDepositPerUser,
      instantDeactivation,
    )
      .accountsStrict({
        vaultType,
        mint: tokenMint,
        authority: admin,
        pool,
        reserve,
        payer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .signers([wallet.payer, adminKeypair])
      .rpc();
    console.log("Transaction signature", txSig);

    const vaultTypeAccount = await program.account.vaultType.fetch(vaultType);

    assert.ok(vaultTypeAccount.authority.equals(admin), "Authority pubkey should match");
    assert.ok(vaultTypeAccount.mint.equals(tokenMint), "Mint pubkey should match");
    assert.ok(vaultTypeAccount.pool.equals(pool), "Pool pubkey should match");
    assert.ok(vaultTypeAccount.reserve.equals(reserve), "Reserve pubkey should match");
    assert.equal(vaultTypeAccount.bump, vaultTypeBump, `Bump should match`);
    assert.equal(vaultTypeAccount.seasonStart.toNumber(), seasonStart.toNumber(), `seasonStart should match`);
    assert.equal(vaultTypeAccount.seasonDuration.toNumber(), seasonDuration.toNumber(), `seasonDuration should match`);
    assert.equal(vaultTypeAccount.cooldownWindow.toNumber(), cooldownWindow.toNumber(), `cooldownWindow should match`);
    assert.equal(vaultTypeAccount.maxDepositPerUser.toNumber(), maxDepositPerUser.toNumber(), `maxDepositPerUser should match`);
    assert.equal(vaultTypeAccount.totalDeposit.toNumber(), 0, `totalDeposit should match`);
    assert.equal(vaultTypeAccount.instantDeactivation, instantDeactivation, `instantDeactivation should match`);
  });

  it("roll_over_vault_type", async () => {
    const [vaultType, vaultTypeBump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('vault_type'),
        tokenMint.toBuffer(),
        admin.toBuffer(),
      ],
      program.programId
    );

  
    // Initial season is more than 2 weeks ago, so at this pointanyone can roll over.
    const txSig = await program.methods.rollOverVaultType(
    )
      .accounts({
        vaultType,
      })
      .rpc();
    console.log("Transaction signature", txSig);

    const vaultTypeAccount = await program.account.vaultType.fetch(vaultType);
    assert.equal(vaultTypeAccount.seasonStart.toNumber(), seasonStart.toNumber() + seasonDuration.toNumber(), `seasonStart should match`);
  });

  it("new_vault", async () => {
    const [vaultType, vaultTypeBump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('vault_type'),
        tokenMint.toBuffer(),
        admin.toBuffer(),
      ],
      program.programId
    );

    const [vault, vaultBump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('vault'),
        vaultType.toBuffer(),
        userAuthority.toBuffer(),
      ],
      program.programId
    );

    const txSig = await program.methods.newVault(
    )
      .accounts({
        // vault,
        vaultType,
        userAuthority,
        payer: wallet.publicKey,
        // systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([wallet.payer, userAuthorityKeypair])
      .rpc();
    console.log("Transaction signature", txSig);

    const vaultAccount = await program.account.vault.fetch(vault);

    assert.ok(vaultAccount.userAuthority.equals(userAuthority), "UserAuthority pubkey should match");
    assert.ok(vaultAccount.vaultType.equals(vaultType), "VaultType pubkey should match");
    assert.equal(vaultAccount.bump, vaultBump, `Bump should match`);
    assert.equal(vaultAccount.amount.toNumber(), 0, `amount should match`);
    assert.equal(vaultAccount.inactiveAt.toNumber(), 0, `inactiveAt should match`);
    assert.notOk(vaultAccount.status.active, `status should match`);
    assert.notOk(vaultAccount.status.deactivating, `status should match`);
    assert.ok(vaultAccount.status.inactive, `status should match`);
  });

  it("deposit", async () => {
    const [vaultType, vaultTypeBump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('vault_type'),
        tokenMint.toBuffer(),
        admin.toBuffer(),
      ],
      program.programId
    );

    const [vault, vaultBump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('vault'),
        vaultType.toBuffer(),
        userAuthority.toBuffer(),
      ],
      program.programId
    );

    const userTokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      wallet.payer,
      tokenMint,
      userAuthority,
    );

    const mintAmount = 3_000_000_000;
    const depositAmount = 1_000_000_000;

    const mintSig = await mintTo(
      provider.connection,
      wallet.payer,
      tokenMint,
      userTokenAccount,
      mintAuthority,
      mintAmount,
    );
    console.log(`Mint signature: ${mintSig}`);

    // Verify the balance
    const balance0 = await provider.connection.getTokenAccountBalance(userTokenAccount);
    assert.equal(balance0.value.amount, mintAmount.toString(), `User token account balance should be ${mintAmount}`);

    const txSig = await program.methods.deposit(
      new anchor.BN(depositAmount),
    )
      .accountsStrict({
        vault,
        vaultType,
        userAuthority,
        mint: tokenMint,
        pool: poolTokenAccountKeypair.publicKey,
        from: userTokenAccount,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .signers([userAuthorityKeypair])
      .rpc();
    console.log("Transaction signature", txSig);

    const vaultAccount = await program.account.vault.fetch(vault);

    assert.ok(vaultAccount.userAuthority.equals(userAuthority), "UserAuthority pubkey should match");
    assert.ok(vaultAccount.vaultType.equals(vaultType), "VaultType pubkey should match");
    assert.equal(vaultAccount.bump, vaultBump, `Bump should match`);
    assert.equal(vaultAccount.amount.toNumber(), depositAmount, `amount should match`);
    assert.equal(vaultAccount.inactiveAt.toNumber(), 0, `inactiveAt should match`);
    assert.ok(vaultAccount.status.active, `status should match`);
    assert.notOk(vaultAccount.status.deactivating, `status should match`);
    assert.notOk(vaultAccount.status.inactive, `status should match`);

    // Verify the balance
    const balance1 = await provider.connection.getTokenAccountBalance(userTokenAccount);
    assert.equal(balance1.value.amount, (mintAmount - depositAmount).toString(), `User token account balance should be ${mintAmount - depositAmount}`);
  });

  it("transfer_from_pool", async () => {
    const [vaultType, _vaultTypeBump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('vault_type'),
        tokenMint.toBuffer(),
        admin.toBuffer(),
      ],
      program.programId
    );

    const newWallet = anchor.web3.Keypair.generate();

    const tempTokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      wallet.payer,
      tokenMint,
      newWallet.publicKey,
    );

    const depositAmount = 1_000_000_000;

    const txSig = await program.methods.transferVaultTypeToken(
      new anchor.BN(depositAmount),
    )
      .accountsStrict({
        vaultType,
        authority: admin,
        mint: tokenMint,
        source: poolTokenAccountKeypair.publicKey,
        destination: tempTokenAccount,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .signers([adminKeypair])
      .rpc();
    console.log("Transaction signature", txSig);

    // Verify the balance
    const balance = await provider.connection.getTokenAccountBalance(tempTokenAccount);
    assert.equal(balance.value.amount, depositAmount.toString(), `User token account balance should be ${depositAmount}`);

    // Transfer tokens back from tempTokenAccount to pool
    const transferBackTxSig = await transfer(
      provider.connection,
      wallet.payer,
      tempTokenAccount,
      poolTokenAccountKeypair.publicKey,
      newWallet,
      depositAmount
    );
    console.log("Transfer back transaction signature", transferBackTxSig);

    // Verify the pool balance
    const poolBalance = await provider.connection.getTokenAccountBalance(poolTokenAccountKeypair.publicKey);
    assert.equal(poolBalance.value.amount, depositAmount.toString(), `Pool token account balance should be ${depositAmount}`);

    // Verify the temp account balance is now zero
    const tempBalance = await provider.connection.getTokenAccountBalance(tempTokenAccount);
    assert.equal(tempBalance.value.amount, "0", "Temp token account balance should be 0");
  });

  it("deactivate", async () => {
    const [vaultType, _vaultTypeBump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('vault_type'),
        tokenMint.toBuffer(),
        admin.toBuffer(),
      ],
      program.programId
    );

    const [vault, _vaultBump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('vault'),
        vaultType.toBuffer(),
        userAuthority.toBuffer(),
      ],
      program.programId
    );

    const txSig = await program.methods.deactivate(
    )
      .accountsStrict({
        vault,
        vaultType,
        userAuthority,
      })
      .signers([userAuthorityKeypair])
      .rpc();
    console.log("Transaction signature", txSig);

    const vaultAccount = await program.account.vault.fetch(vault);
    console.log("Vault Account Contents:");
    console.log(JSON.stringify(vaultAccount, null, 2));
    assert.notOk(vaultAccount.status.active, `status should match`);
    assert.notOk(vaultAccount.status.deactivating, `status should match`);
    assert.ok(vaultAccount.status.inactive, `status should match`);
  });

  it("withdraw", async () => {
    const [vaultType, vaultTypeBump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('vault_type'),
        tokenMint.toBuffer(),
        admin.toBuffer(),
      ],
      program.programId
    );

    const [vault, vaultBump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('vault'),
        vaultType.toBuffer(),
        userAuthority.toBuffer(),
      ],
      program.programId
    );

    const userTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      wallet.payer,
      tokenMint,
      userAuthority,
    );

    const mintAmount = 3_000_000_000;
    const depositAmount = 1_000_000_000;
    const remainingAmount = mintAmount - depositAmount;

    // Verify the balance
    const balance0 = await provider.connection.getTokenAccountBalance(userTokenAccount.address);
    assert.equal(balance0.value.amount, remainingAmount.toString(), `User token account balance should be ${remainingAmount}`);

    const txSig = await program.methods.withdraw(
      new anchor.BN(depositAmount),
    )
      .accountsStrict({
        vault,
        vaultType,
        userAuthority,
        mint: tokenMint,
        pool: poolTokenAccountKeypair.publicKey,
        reserve: reserveTokenAccountKeypair.publicKey,
        to: userTokenAccount.address,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .signers([userAuthorityKeypair])
      .rpc();
    console.log("Transaction signature", txSig);

    const vaultAccount = await program.account.vault.fetch(vault);

    assert.ok(vaultAccount.userAuthority.equals(userAuthority), "UserAuthority pubkey should match");
    assert.ok(vaultAccount.vaultType.equals(vaultType), "VaultType pubkey should match");
    assert.equal(vaultAccount.bump, vaultBump, `Bump should match`);
    assert.equal(vaultAccount.amount.toNumber(), 0, `amount should match`);

    // Verify the balance
    const balance1 = await provider.connection.getTokenAccountBalance(userTokenAccount.address);
    assert.equal(balance1.value.amount, mintAmount.toString(), `User token account balance should be ${mintAmount}`);
  });

  it("close_vault", async () => {
    const [vaultType, _vaultTypeBump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('vault_type'),
        tokenMint.toBuffer(),
        admin.toBuffer(),
      ],
      program.programId
    );

    const [vault, _vaultBump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('vault'),
        vaultType.toBuffer(),
        userAuthority.toBuffer(),
      ],
      program.programId
    );

    const txSig = await program.methods.closeVault(
    )
      .accountsStrict({
        vault,
        vaultType,
        userAuthority,
        payer: wallet.publicKey,
      })
      .signers([wallet.payer, userAuthorityKeypair])
      .rpc();
    console.log("Transaction signature", txSig);

    const vaults = await program.account.vault.all();
    assert.equal(vaults.length, 0, `${vaults.length} Vault accounts still exist`);
  });

  it("close_vault_type", async () => {
    const [vaultType, vaultTypeBump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('vault_type'),
        tokenMint.toBuffer(),
        admin.toBuffer(),
      ],
      program.programId
    );

    const vaultTypeAccount0 = await program.account.vaultType.fetch(vaultType);

    assert.ok(vaultTypeAccount0.authority.equals(admin), "Authority pubkey should match");
    assert.ok(vaultTypeAccount0.mint.equals(tokenMint), "Mint pubkey should match");
    assert.ok(vaultTypeAccount0.pool.equals(poolTokenAccountKeypair.publicKey), "Pool pubkey should match");
    assert.ok(vaultTypeAccount0.reserve.equals(reserveTokenAccountKeypair.publicKey), "Reserve pubkey should match");
    assert.equal(vaultTypeAccount0.bump, vaultTypeBump, `Bump should match`);

    const txSig = await program.methods.closeVaultType(
    )
      .accountsStrict({
        vaultType,
        authority: admin,
        pool: poolTokenAccountKeypair.publicKey,
        reserve: reserveTokenAccountKeypair.publicKey,
        payer: wallet.publicKey,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .signers([adminKeypair])
      .rpc();
    console.log("Transaction signature", txSig);

    const vaultTypes = await program.account.vaultType.all();
    assert.equal(vaultTypes.length, 0, `${vaultTypes.length} VaultType accounts still exist`);
  });
});
