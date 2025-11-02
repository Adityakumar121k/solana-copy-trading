use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer as KeypairSigner};
use std::env;
use std::sync::LazyLock;

static SIGNER: LazyLock<Keypair> = LazyLock::new(|| {
    let secret = env::var("WALLET_PRIVATE_KEY").expect("WALLET_PRIVATE_KEY not set");

    Keypair::from_base58_string(&secret)
});

static SIGNER_PUBKEY: LazyLock<Pubkey> = LazyLock::new(|| SIGNER.pubkey());

pub struct SignerKeypair;

impl SignerKeypair {
    pub fn init() {
        let _ = &SIGNER;
        let _ = &SIGNER_PUBKEY;

        tracing::info!("SignerKeypair init");
    }

    pub fn keypair() -> &'static Keypair {
        &SIGNER
    }

    pub fn pubkey() -> &'static Pubkey {
        &SIGNER_PUBKEY
    }
}
