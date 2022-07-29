use std::error::Error;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::{Keypair, Signer, Signature}, pubkey::Pubkey, system_transaction};

fn main() {
    let tenth_sol_lamps = 100000000;
    let rpc_client = RpcClient::new("https://api.devnet.solana.com");

    let alice_wallet = Keypair::new();
    let bob_wallet = Keypair::new();

    println!("Alice: {:?}", alice_wallet.pubkey());
    println!("Bob: {:?}", bob_wallet.pubkey());

    let alice_airdrop_request_sig = request_airdrop(&rpc_client, &alice_wallet.pubkey(), tenth_sol_lamps).expect("alice's airdrop request failed");
    let bob_airdrop_request_sig = request_airdrop(&rpc_client, &bob_wallet.pubkey(), tenth_sol_lamps).expect("bob's airdrop request failed");

    println!("Alice airdrop sig: {:?}", alice_airdrop_request_sig);
    println!("Bob airdrop sig: {:?}", bob_airdrop_request_sig);

    println!("Alice's balance: {:?}", rpc_client.get_balance(&alice_wallet.pubkey()).expect("Could not get Alice's balance."));
    println!("Bob's balance: {:?}", rpc_client.get_balance(&bob_wallet.pubkey()).expect("Could not get Bob's balance."));

    rpc_client.send_and_confirm_transaction(&system_transaction::transfer(
        &alice_wallet,
        &bob_wallet.pubkey(),
        tenth_sol_lamps/2,
        rpc_client.get_latest_blockhash().unwrap()
    ));

    
    println!("Alice's balance: {:?}", rpc_client.get_balance(&alice_wallet.pubkey()).expect("Could not get Alice's balance."));
    println!("Bob's balance: {:?}", rpc_client.get_balance(&bob_wallet.pubkey()).expect("Could not get Bob's balance."));

}

pub fn request_airdrop(rpc_client: &RpcClient, pub_key: &Pubkey, amount_sol: u64) -> Result<Signature, Box<dyn Error>> {
    let sig = rpc_client.request_airdrop(pub_key, amount_sol)?;
    loop {
        let confirmed = rpc_client.confirm_transaction(&sig)?;
        if confirmed {
            break;
        }
    }
    Ok(sig)
}