use solana_sdk::{
    message::Message,
    program_pack::Pack,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction::create_account,
    transaction::Transaction,
};
use spl_token::instruction::initialize_account;
use spl_token::instruction::initialize_mint;

mod utils;

// If you want to use spl-token library and rust code to create SPL Token, please refer to the following code.

pub fn main(decimals: u8) {
    let my_keypair = utils::load_config_keypair();
    let my_pubkey = my_keypair.pubkey();

    let token_account_size = spl_token::state::Mint::LEN;
    let rpc_client = utils::new_rpc_client();
    let token_balance = rpc_client
        .get_minimum_balance_for_rent_exemption(token_account_size)
        .expect("failed to get min balance");

    let new_account_keypair = Keypair::new(); // New random keypair
    let new_account_pubkey = new_account_keypair.pubkey();

    let create_account_instruction = create_account(
        &my_pubkey,
        &new_account_pubkey,
        token_balance,
        token_account_size as u64,
        &spl_token::ID,
    );

    let initialize_mint_instruction = initialize_mint(
        &spl_token::ID,
        &new_account_pubkey,
        &my_pubkey,
        None,
        decimals,
    )
    .unwrap();

    let rpc_client = utils::new_rpc_client();
    let (recent_blockhash, _fee_calculator) = rpc_client
        .get_recent_blockhash()
        .expect("failed to get recent blockhash");

    let tx = Transaction::new(
        &[&my_keypair, &new_account_keypair],
        Message::new(
            &[create_account_instruction, initialize_mint_instruction],
            Some(&my_pubkey),
        ),
        recent_blockhash,
    );
    
    rpc_client
        .send_and_confirm_transaction_with_spinner(&tx)
        .expect("tx failed");

    println!("Created Token Mint: {}", new_account_pubkey);
    println!("Transaction Signature: {}", utils::tx_signature(&tx));
}
