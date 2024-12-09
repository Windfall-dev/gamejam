import { program, defaultConnection, defaultKeypairPath } from "./index";
import { AnchorProvider, Program, Wallet } from '@coral-xyz/anchor';
import * as anchor from '@coral-xyz/anchor';
import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import { createAccount } from "@solana/spl-token";
import { getNow, loadKeypair, toUTCDayjs, truncateToHour } from "./utils";
import { Vault } from "../../target/types/vault";

export function defineCommands() {
    program
        .command('list-vault-types')
        .description('List all vault types')
        .option(
            '-c, --connection <connection>',
            'Connection URL of Solana RPC'
        )
        .option(
            '-k, --keypair-path <keypair-path>',
            'Path of the keypair to use (default: ~/.config/solana/id.json)'
        )
        .option(
            '-i, --program-id <program-id>',
            'The program ID'
        )
        .action(listVaultTypes);

    program
        .command('new-vault-type')
        .description('Create a new vault type')
        .option(
            '-o, --owner <keypair-path>',
            'The keypair path of the owner (default: payer keypair path)'
        )
        .requiredOption(
            '-m, --mint <pubkey>',
            'The public key of the pool token mint'
        )
        .requiredOption(
            '-p, --pool <keypair-path>',
            'The keypair path of the pool token account'
        )
        .requiredOption(
            '-r, --reserve <keypair-path>',
            'The keypair path of the reserve token account'
        )
        .option(
            '-s, --start <date-time-string>',
            'The start date and time of the season (default: now, truncated to hour boundary)'
        )
        .option(
            '-d, --duration <seconds>',
            'The time interval of the season in seconds (default: 2 weeks)'
        )
        .option(
            '-a, --deactivation-lock-window <seconds>',
            'The duration of the deactivation lock window in seconds (default: 3 days)'
        )
        .option(
            '-l, --cooldown <seconds>',
            'The duration of the season cooldown period in seconds (default: 6 hours)'
        )
        .option(
            '-x, --max-deposit <token-amount>',
            'The maximum token amount each user can deposit to the vault (default: 0)'
        )
        .option(
            '-n, --instant-deactivation',
            'Allow users to deactivate their vaults instantly (default: false)'
        )
        .option(
            '-c, --connection <connection>',
            'The connection URL of Solana RPC'
        )
        .option(
            '-k, --keypair-path <keypair-path>',
            'Path of the payer keypair to use (default: ~/.config/solana/id.json)'
        )
        .option(
            '-i, --program-id <program-id>',
            'The program ID'
        )
        .action(newVaultType);

        program
        .command('close-vault-type')
        .description('Close a vault type')
        .requiredOption(
            '-v, --vault-type <pubkey>',
            'The public key of the vault type account to close'
        )
        .option(
            '-o, --owner <keypair-path>',
            'The keypair path of the owner (default: payer keypair path)'
        )
        .option(
            '-c, --connection <connection>',
            'Connection URL of Solana RPC'
        )
        .option(
            '-k, --keypair-path <keypair-path>',
            'Path of the keypair to use (default: ~/.config/solana/id.json)'
        )
        .option(
            '-i, --program-id <program-id>',
            'The program ID'
        )
        .action(closeVaultType);
}

async function listVaultTypes({ connection, keypairPath, programId }: {
    connection: string
    keypairPath: string
    programId: string
}) {
    const conn = new Connection(connection ? connection : defaultConnection);
    const wallet = new Wallet(loadKeypair(keypairPath ? keypairPath : defaultKeypairPath));
    const provider = new AnchorProvider(conn, wallet, {});
    anchor.setProvider(provider);

    const program = anchor.workspace.Vault as Program<Vault>;

    const vaultTypes = await program.account.vaultType.all();
    vaultTypes.forEach((r, index) => {
        const vaultType = r.account;
        console.log(`VaultType [${index + 1}/${vaultTypes.length}]: Pubkey ${r.publicKey}`);
        console.log(`  Identity: ${vaultType.identity.toString()}`);
        console.log(`  Authority: ${vaultType.authority.toString()}`);
        console.log(`  Mint: ${vaultType.mint.toString()}`);
        console.log(`  Pool: ${vaultType.pool.toString()}`);
        console.log(`  Reserve: ${vaultType.reserve.toString()}`);
        console.log(`  Token Program: ${vaultType.tokenProgram.toString()}`);
        console.log(`  Start Timestamp: ${new Date(vaultType.seasonStart.toNumber() * 1000).toUTCString()}`);
        console.log(`  Duration: ${vaultType.seasonDuration.toNumber()}`);
        console.log(`  Deactivation Lock Window: ${vaultType.deactivationLockWindow.toNumber()}`);
        console.log(`  Cooldown Window: ${vaultType.cooldownWindow.toNumber()}`);
        console.log(`  Max Deposit Per User: ${vaultType.maxDepositPerUser.toNumber()}`);
        console.log(`  Total Deposit: ${vaultType.totalDeposit.toNumber()}`);
        console.log(`  Instant Deactivation: ${vaultType.instantDeactivation}`);
        console.log(`  Is Locked: ${vaultType.isLocked}`);
        console.log(`  Bump: ${vaultType.bump}`);
        console.log('');
    });
}

async function newVaultType({ owner, mint, pool, reserve, start, duration, deactivationLockWindow, cooldown, maxDeposit, instantDeactivation, connection, keypairPath, programId }: {
    owner: string
    mint: string
    pool: string
    reserve: string
    start: string
    duration: string
    deactivationLockWindow: string
    cooldown: string
    maxDeposit: string
    instantDeactivation: boolean
    connection: string
    keypairPath: string
    programId: string
}) {
    const conn = new Connection(connection ? connection : defaultConnection);
    const payer = keypairPath ?? defaultKeypairPath;
    const wallet = new Wallet(loadKeypair(payer));
    const provider = new AnchorProvider(conn, wallet, {});
    anchor.setProvider(provider);

    const program = anchor.workspace.Vault as Program<Vault>;

    const ownerKp = loadKeypair(owner ?? payer);
    const mintPubkey = new PublicKey(mint);
    const startDayjs = start ? truncateToHour(toUTCDayjs(start)) : truncateToHour(getNow());
    const startTime = new anchor.BN(startDayjs.valueOf() / 1000);
    const durationNum = new anchor.BN(duration ?? 60 * 60 * 24 * 14);
    const deactivationLockWindowNum = new anchor.BN(deactivationLockWindow ?? 60 * 60 * 24 * 3);
    const cooldownWindowNum = new anchor.BN(cooldown ?? 60 * 60 * 6);
    const maxDepositNum = new anchor.BN(maxDeposit ?? 0);

    const [vaultType, _] = PublicKey.findProgramAddressSync(
        [
            anchor.utils.bytes.utf8.encode('vault_type'),
            mintPubkey.toBuffer(),
            ownerKp.publicKey.toBuffer(),
        ],
        program.programId
    );

    // const pool = await getOrCreateAssociatedTokenAccount(
    //     provider.connection,
    //     wallet.payer,
    //     mintPubkey,
    //     vaultType,
    //     true,
    // );
    // console.log(`Created ATA pool account: ${pool.address.toBase58()}`);

    const poolKeypair = loadKeypair(pool);
    try {
        await provider.connection.getTokenAccountBalance(poolKeypair.publicKey);
        console.log(`Pool token account ${poolKeypair.publicKey.toBase58()} already exists`);
    } catch (error) {
        console.log(`Creating new pool token account...`);
        await createAccount(
            provider.connection,
            wallet.payer,
            mintPubkey,
            vaultType,
            poolKeypair
        );
    }
    console.log(`Pool token account: ${poolKeypair.publicKey.toBase58()}`);

    const reserveKeypair = loadKeypair(reserve);
    try {
        await provider.connection.getTokenAccountBalance(reserveKeypair.publicKey);
        console.log(`Reserve token account ${reserveKeypair.publicKey.toBase58()} already exists`);
    } catch (error) {
        console.log(`Creating new reserve token account...`);
        await createAccount(
            provider.connection,
            wallet.payer,
            mintPubkey,
            vaultType,
            reserveKeypair
        );
    }
    console.log(`Reserve token account: ${reserveKeypair.publicKey.toBase58()}`);

    // if (conn.rpcEndpoint.includes('devnet') || conn.rpcEndpoint.includes('mainnet')) {
    //     console.log('Waiting for 10 seconds to confirm token account creation...');
    //     await new Promise(resolve => setTimeout(resolve, 10_000));
    // }

    // Create a new vault type
    try {
        const newVaultTypeTx = await program.methods.newVaultType(
            startTime,
            durationNum,
            deactivationLockWindowNum,
            cooldownWindowNum,
            maxDepositNum,
            instantDeactivation,
        )
            .accountsStrict({
                authority: ownerKp.publicKey,
                mint: mintPubkey,
                pool: poolKeypair.publicKey,
                reserve: reserveKeypair.publicKey,
                vaultType,
                payer: wallet.payer.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
                tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
            })
            .signers([wallet.payer, ownerKp])
            .rpc();
        console.log("Transaction signature:", newVaultTypeTx);

        console.log("Created vaultType:", vaultType.toString());
    } catch (error) {
        if (error instanceof anchor.web3.SendTransactionError) {
            console.error("SendTransactionError occurred:");
            console.error("Error message:", error.message);
            console.error("Error logs:", error.logs);
        } else {
            console.error("An unexpected error occurred:", error);
        }
    }
}

async function closeVaultType({ vaultType, owner, connection, keypairPath, programId }: {
    vaultType: string
    owner: string
    connection: string
    keypairPath: string
    programId: string
}) {
    const conn = new Connection(connection ? connection : defaultConnection);
    const payer = keypairPath ?? defaultKeypairPath;
    const wallet = new Wallet(loadKeypair(payer));
    const provider = new AnchorProvider(conn, wallet, {});
    anchor.setProvider(provider);

    const program = anchor.workspace.Vault as Program<Vault>;

    const vaultTypePubkey = new PublicKey(vaultType);
    const ownerKp = loadKeypair(owner ?? payer);
    
    const vaultTypeAccount = await program.account.vaultType.fetch(vaultTypePubkey);

    // Close a vault type
    try {
        const closeVaultTypeTx = await program.methods.closeVaultType(
        )
            .accountsStrict({
                vaultType: vaultTypePubkey,                
                authority: ownerKp.publicKey,
                pool: vaultTypeAccount.pool,
                reserve: vaultTypeAccount.reserve,
                payer: wallet.payer.publicKey,
                tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
            })
            .signers([wallet.payer, ownerKp])
            .rpc();
        console.log("Transaction signature:", closeVaultTypeTx);
    } catch (error) {
        if (error instanceof anchor.web3.SendTransactionError) {
            console.error("SendTransactionError occurred:");
            console.error("Error message:", error.message);
            console.error("Error logs:", error.logs);
        } else {
            console.error("An unexpected error occurred:", error);
        }
    }
}
