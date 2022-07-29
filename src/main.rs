use std::error::Error;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::{Keypair, Signer, Signature}, pubkey::Pubkey};

fn main() {
    let rpc_client = RpcClient::new("https://api.devnet.solana.com");

    let wallet = Keypair::new();
    println!("{:?}", wallet.pubkey());

    let airdrop_request_sig = request_airdrop(&rpc_client, &wallet.pubkey(), 1000000).expect("airdrop requeest failed");
    println!("{:?}", airdrop_request_sig);

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