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

    // Create account instruction
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
