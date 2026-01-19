use crate::Result;

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{
    program_pack::Pack,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use spl_token_2022_interface::{
    id as token_2022_program_id, instruction::initialize_mint, state::Mint,
};

use tracing::info;

/// The example below creates and fetches a Token Mint account owned by the Token 2022 program.
pub async fn run() -> Result<()> {
    // Create connection to local validator
    // so, we need to first start local validator by: solana-test-validator
    let client = RpcClient::new_with_commitment(
        String::from("http://localhost:8899"),
        CommitmentConfig::confirmed(),
    );
    let recent_blockhash = client.get_latest_blockhash().await?;

    // Generate a new keypair for the fee payer
    let fee_payer = Keypair::new();

    // Airdrop 1 SOL to fee payer
    let airdrop_signature = client
        .request_airdrop(&fee_payer.pubkey(), 1_000_000_000)
        .await?;

    loop {
        let confirmed = client.confirm_transaction(&airdrop_signature).await?;
        if confirmed {
            break;
        }
    }

    // Generate keypair to use as address of mint
    let mint = Keypair::new();

    let space = Mint::LEN;
    let rent = client.get_minimum_balance_for_rent_exemption(space).await?;

    // Create the mint account instruction
    let create_account_instruction = create_account(
        &fee_payer.pubkey(),      // fee payer
        &mint.pubkey(),           // mint address
        rent,                     // rent
        space as u64,             // space
        &token_2022_program_id(), // program id
    );

    // Initialize mint instruction
    let initialize_mint_instruction = initialize_mint(
        &token_2022_program_id(),
        &mint.pubkey(),            // mint address
        &fee_payer.pubkey(),       // mint authority
        Some(&fee_payer.pubkey()), // freeze authority
        9,                         // decimals
    )?;

    // Create transaction and add instructions
    let transaction = Transaction::new_signed_with_payer(
        &[create_account_instruction, initialize_mint_instruction],
        Some(&fee_payer.pubkey()),
        &[&fee_payer, &mint],
        recent_blockhash,
    );

    // Send and confirm transaction
    let transaction_signature = client.send_and_confirm_transaction(&transaction).await?;

    info!("Mint Address: {}", mint.pubkey());
    info!("Transaction Signature: {}", transaction_signature);

    let account_info = client.get_account(&mint.pubkey()).await?;
    info!("{:#?}", account_info);

    let mint_account = Mint::unpack(&account_info.data)?;
    info!("{:#?}", mint_account);

    Ok(())
}

// Based on the code context, the run function appears to be an asynchronous function that:

//    1. Creates a new keypair for a fee payer account (which will pay for transaction fees)
//    2. Initializes a Solana RPC client connected to the local test validator
//    3. Requests an airdrop of 1 billion lamports (1 SOL) to the fee payer account to fund transactions
//    4. Defines the space required for a new token mint account
//    5. Gets the rent exemption amount needed for the mint account
//    6. Creates a new keypair for the mint account
//    7. Creates an instruction to initialize a new token mint account
//    8. Builds and sends a transaction containing the mint creation instruction
//    9. Waits for the transaction to be confirmed

//   Essentially, this function demonstrates how to create a new token mint on Solana using the Token-2022 program. It handles the prerequisite steps like funding the account and
//   calculating rent requirements before creating the actual token mint.

// so, step 3 is needed which does " 3. Requests an airdrop of 1 billion lamports (1 SOL) to the fee payer account to fund transactions" before create and submitting the transaction
//   which init the new token mint?

// ✦ Yes, that's correct! The airdrop in step 3 is essential because:

//    1. Transaction Fees: Creating a new token mint requires a transaction to be submitted to the Solana network, and all transactions require SOL to pay fees.

//    2. Rent Requirements: The new mint account needs to be rent-exempt, meaning it requires a certain amount of SOL to remain stored on-chain permanently. This is calculated based on
//       the space required for the account data.

//    3. Account Funding: The fee payer account needs sufficient SOL to cover both the transaction fees and the rent deposit for the new mint account.

//   Without the airdrop to fund the fee payer account, the transaction to create the token mint would fail due to insufficient funds. The airdrop essentially provides the necessary
//   SOL to pay for creating and initializing the new token mint account on the blockchain.
