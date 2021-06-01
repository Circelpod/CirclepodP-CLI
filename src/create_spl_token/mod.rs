use solana_sdk::{
    message::Message,
    program_pack::Pack,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction::create_account,
    transaction::Transaction,
};
use spl_token::instruction::initialize_mint;
use spl_token::instruction::initialize_account;

mod utils;

pub fn main() {

    let new_token_keypair = Keypair::new(); // New random keypair
    let new_token_pubkey = new_token_keypair.pubkey();

    let my_keypair = utils::load_config_keypair();
    let my_pubkey = my_keypair.pubkey();

    let initialize_mint_instruction = initialize_mint(
        &spl_token::id(),
        &new_token_pubkey,
        &my_pubkey,
        Some(&my_pubkey),
        32
    )
    .unwrap();

    let rpc_client = utils::new_rpc_client();
    let (recent_blockhash, _fee_calculator) = rpc_client
        .get_recent_blockhash()
        .expect("failed to get recent blockhash");

    let tx = Transaction::new(
        &[&my_keypair],
        Message::new(
            &[initialize_mint_instruction],
            Some(&my_pubkey),
        ),
        recent_blockhash,
    );
    
    rpc_client
        .send_and_confirm_transaction_with_spinner(&tx)
        .expect("tx failed");

    println!("Transaction Signature: {}", utils::tx_signature(&tx));
}