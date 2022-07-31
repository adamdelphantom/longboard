use std::error::Error;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::{Keypair, Signer, Signature}, pubkey::Pubkey, system_transaction};

fn main() {
    // let tenth_sol_lamps = 100000000;
    // let rpc_client = RpcClient::new("https://api.devnet.solana.com");

    // let alice_wallet = Keypair::new();
    // let bob_wallet = Keypair::new();

    // println!("Alice: {:?}", alice_wallet.pubkey());
    // println!("Alice's privkey: {:?}", alice_wallet.secret());
    // println!("Bob: {:?}", bob_wallet.pubkey());

    // let alice_airdrop_request_sig = request_one_sol(&rpc_client, &alice_wallet.pubkey()).expect("alice's airdrop request failed");
    // let bob_airdrop_request_sig = request_one_sol(&rpc_client, &bob_wallet.pubkey()).expect("bob's airdrop request failed");

    // println!("Alice airdrop sig: {:?}", alice_airdrop_request_sig);
    // println!("Bob airdrop sig: {:?}", bob_airdrop_request_sig);

    // println!("Alice's balance: {:?}", rpc_client.get_balance(&alice_wallet.pubkey()).expect("Could not get Alice's balance."));
    // println!("Bob's balance: {:?}", rpc_client.get_balance(&bob_wallet.pubkey()).expect("Could not get Bob's balance."));

    // // TODO: handle error
    // transfer_sol(&rpc_client, &alice_wallet, &bob_wallet.pubkey(), tenth_sol_lamps);
    
    // println!("Alice's balance: {:?}", rpc_client.get_balance(&alice_wallet.pubkey()).expect("Could not get Alice's balance."));
    // println!("Bob's balance: {:?}", rpc_client.get_balance(&bob_wallet.pubkey()).expect("Could not get Bob's balance."));

    main_menu_loop()
}

pub fn main_menu_loop() {
    println!("Menu");
    println!("---------------------------");
    println!("1. List wallets - TODO: not implemented");
    println!("2. Generate Keypair/Wallet");
    println!("3. Mint Token - TODO: not implemented");
    println!("New Wallet: {:?}", generate_wallet());
}

pub fn generate_wallet() -> Pubkey {
    let w = Keypair::new();
    w.pubkey()
}

pub fn request_one_sol(rpc_client: &RpcClient, pub_key: &Pubkey) -> Result<Signature, Box<dyn Error>> {
    let sig = rpc_client.request_airdrop(pub_key, 1000000000)?;
    // TODO: async this transaction confirmation
    loop {
        let confirmed = rpc_client.confirm_transaction(&sig)?;
        if confirmed {
            break;
        }
    }
    Ok(sig)
}

pub fn transfer_sol(client: &RpcClient, 
    from_keypair: &Keypair, 
    to_pubkey: &Pubkey, 
    lamports_to_send: u64) -> Result<Signature, solana_client::client_error::ClientError>{
    client.send_and_confirm_transaction(&system_transaction::transfer(
                from_keypair, 
                to_pubkey, 
                lamports_to_send, 
                // TODO: handle client call failure
                client.get_latest_blockhash().unwrap()))
}