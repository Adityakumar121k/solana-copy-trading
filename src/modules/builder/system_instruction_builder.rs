use crate::kernel::wallet::signer::SignerKeypair;
use crate::modules::builder::config::{
    COMPUTE_UNIT_LIMIT, JITO_DONT_FRONT_ADDRESS, TIP_FEE_LAMPORTS, TIP_RECEIVERS,
};
use rand::seq::IndexedRandom;
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::system_instruction::transfer;

pub struct SystemInstructionBuilder;

impl SystemInstructionBuilder {
    pub fn build_compute_unit_limit() -> Instruction {
        let mut compute_unit_ix =
            ComputeBudgetInstruction::set_compute_unit_limit(COMPUTE_UNIT_LIMIT);

        compute_unit_ix.accounts.push(AccountMeta {
            pubkey: JITO_DONT_FRONT_ADDRESS,
            is_signer: false,
            is_writable: false,
        });

        compute_unit_ix
    }

    pub fn build_compute_unit_price(amount: u64) -> Instruction {
        ComputeBudgetInstruction::set_compute_unit_price(amount)
    }

    pub fn build_tip() -> Instruction {
        let mut rng = rand::rng();
        let tip_pubkey = TIP_RECEIVERS.choose(&mut rng).unwrap();

        transfer(SignerKeypair::pubkey(), tip_pubkey, TIP_FEE_LAMPORTS)
    }
}
