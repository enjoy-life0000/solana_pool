use anyhow::Result;
use solana_program_test::{tokio, ProgramTest};
use solana_sdk::{
    program_pack::Pack, signature::Keypair, signer::Signer, system_instruction,
    transaction::Transaction,
};
use spl_token::{
    id, instruction,
    state::{Account, Mint},
};

#[tokio::main]
async fn main() -> Result<()> {
    let program_test = ProgramTest::default();
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let mint_account = Keypair::new();
    let owner = Keypair::new();
    let token_program = &id();
    let rent = banks_client.get_rent().await.unwrap();
    let mint_rent = rent.minimum_balance(Mint::LEN);

    let token_mint_a_account_ix = solana_program::system_instruction::create_account(
        &payer.pubkey(),
        &mint_account.pubkey(),
        mint_rent,
        Mint::LEN as u64,
        token_program,
    );

    let token_mint_a_ix = instruction::initialize_mint(
        token_program,
        &mint_account.pubkey(),
        &owner.pubkey(),
        None,
        9,
    )
    .unwrap();

    // create mint transaction
    let token_mint_a_tx = Transaction::new_signed_with_payer(
        &[token_mint_a_account_ix, token_mint_a_ix],
        Some(&payer.pubkey()),
        &[&payer, &mint_account],
        recent_blockhash,
    );
    Ok(())
}
