use crate::kernel::cache::block_hash_cache::BlockHashCache;
use crate::kernel::wallet::signer::SignerKeypair;
use crate::modules::builder::program_instruction_builder::ProgramInstructionBuilder;
use crate::modules::builder::system_instruction_builder::SystemInstructionBuilder;
use crate::modules::decoder::types::ParsedTransaction;
use anyhow::Result;
use solana_sdk::message::VersionedMessage;
use solana_sdk::message::v0::Message;
use solana_sdk::transaction::VersionedTransaction;
use std::time::Instant;

pub struct TransactionBuilder;

impl TransactionBuilder {
    pub fn build_transaction(
        target_transaction: &ParsedTransaction,
    ) -> Result<VersionedTransaction> {
        let start = Instant::now();

        let instructions = &mut ProgramInstructionBuilder::build_swap(
            &target_transaction.instruction.program_id,
            &target_transaction.instruction,
        )?;

        instructions.insert(0, SystemInstructionBuilder::build_compute_unit_limit());

        instructions.insert(
            1,
            SystemInstructionBuilder::build_compute_unit_price(target_transaction.priority_fee),
        );

        instructions.push(SystemInstructionBuilder::build_tip());

        let message = Message::try_compile(
            SignerKeypair::pubkey(),
            instructions,
            &[],
            BlockHashCache::get(),
        )?;

        let message = VersionedMessage::V0(message);

        let transaction = VersionedTransaction::try_new(message, &[SignerKeypair::keypair()])?;

        println!(
            "built transaction {} µs",
            start.elapsed().as_nanos() as f64 / 1_000.0
        );

        Ok(transaction)
    }
}
