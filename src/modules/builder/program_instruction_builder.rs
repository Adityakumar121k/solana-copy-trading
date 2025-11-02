use crate::modules::builder::program_swap_builder::ProgramSwapBuilder;
use crate::modules::builder::pump_fun::instruction_builder::PumpFunBuilder;
use crate::modules::decoder::pump_fun::config::PUMP_FUN_PROGRAM_ID_PUBKEY;
use crate::modules::decoder::types::ParsedInstruction;
use anyhow::Result;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;

pub struct ProgramInstructionBuilder;

impl ProgramInstructionBuilder {
    pub fn build_swap(
        program_id: &Pubkey,
        parsed_instruction: &ParsedInstruction,
    ) -> Result<Vec<Instruction>> {
        match program_id {
            &PUMP_FUN_PROGRAM_ID_PUBKEY => PumpFunBuilder::build_swap(parsed_instruction),
            _ => anyhow::bail!("unsupported program: {program_id}"),
        }
    }
}
