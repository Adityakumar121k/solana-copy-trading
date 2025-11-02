use base64::Engine;
use solana_sdk::transaction::VersionedTransaction;

pub struct Encoder;

impl Encoder {
    pub fn base64_encode(transaction: &VersionedTransaction) -> String {
        base64::engine::general_purpose::STANDARD.encode(bincode::serialize(transaction).unwrap())
    }
}
