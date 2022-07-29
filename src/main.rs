use solana_sdk::signature::{Keypair, Signer};

fn main() {
    let wallet = Keypair::new();
    println!("{:?}", wallet.pubkey());
}