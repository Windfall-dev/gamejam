import { program, defaultConnection, defaultKeypairPath } from "./index";
import { AnchorProvider, Program, Wallet } from '@coral-xyz/anchor';
import * as anchor from '@coral-xyz/anchor';
import { Connection, PublicKey } from '@solana/web3.js';
import { loadKeypair } from "./utils";
import { Vault } from "../../target/types/vault";

export function defineCommands() {
    program
        .command('list-vaults')
        .description('List all vaults')
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
        .action(listVaults);

    program
        .command('new-vault')
        .description('Create a new vault')
        .option(
            '-o, --owner <keypair-path>',
            'The keypair path of the owner (default: payer keypair path)'
        )
        .requiredOption(
            '-t, --vault-type <pubkey>',
            'The public key of the vault type'
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
        .action(newVault);

    program
        .command('close-vault')
        .description('Close a vault')
        .requiredOption(
            '-v, --vault <pubkey>',
            'The public key of the vault account to close'
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
        .action(closeVault);
}

async function listVaults({ connection, keypairPath, programId }: {
    connection: string
    keypairPath: string
    programId: string
}) {
    const conn = new Connection(connection ? connection : defaultConnection);
    const wallet = new Wallet(loadKeypair(keypairPath ? keypairPath : defaultKeypairPath));
    const provider = new AnchorProvider(conn, wallet, {});
    anchor.setProvider(provider);

    const program = anchor.workspace.Vault as Program<Vault>;

    const vaults = await program.account.vault.all();
    vaults.forEach((r, index) => {
        const vault = r.account;
        console.log(`Vault [${index + 1}/${vaults.length}]: Pubkey ${r.publicKey}`);
        console.log(`  User Authority: ${vault.userAuthority.toString()}`);
        console.log(`  Vault Type: ${vault.vaultType.toString()}`);
        console.log(`  Amount: ${vault.amount.toNumber()}`);
        console.log(`  Inactive At: ${vault.inactiveAt.toNumber()}`);
        console.log(`  Status: ${vault.status.active ? "active" : (vault.status.deactivating ? "deactivating" : "inactive")}`);
        console.log('');
    });
}

async function newVault({ owner, vaultType, connection, keypairPath, programId }: {
    owner: string
    vaultType: string
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

    const [vault, _] = PublicKey.findProgramAddressSync(
        [
            anchor.utils.bytes.utf8.encode('vault'),
            vaultTypePubkey.toBuffer(),
            ownerKp.publicKey.toBuffer(),
        ],
        program.programId
    );

    // Create a new vault
    try {
        const newVaultTypeTx = await program.methods.newVault(
        )
            .accountsStrict({
                vault,
                vaultType,
                userAuthority: ownerKp.publicKey,
                payer: wallet.payer.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([wallet.payer, ownerKp])
            .rpc();
        console.log("Transaction signature:", newVaultTypeTx);

        console.log("Created vault:", vault.toString());
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

async function closeVault({ vault, owner, connection, keypairPath, programId }: {
    vault: string
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

    const vaultPubkey = new PublicKey(vault);
    const ownerKp = loadKeypair(owner ?? payer);

    const vaultAccount = await program.account.vault.fetch(vaultPubkey);

    // Close a vault type
    try {
        const closeVaultTx = await program.methods.closeVault(
        )
            .accountsStrict({
                vault: vaultPubkey,
                vaultType: vaultAccount.vaultType,
                userAuthority: ownerKp.publicKey,
                payer: wallet.payer.publicKey,
            })
            .signers([wallet.payer, ownerKp])
            .rpc();
        console.log("Transaction signature:", closeVaultTx);
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
