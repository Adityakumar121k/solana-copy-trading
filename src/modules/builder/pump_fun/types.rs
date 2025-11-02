use crate::kernel::wallet::signer::SignerKeypair;
use crate::modules::decoder::pump_fun::config::{PUMP_FUN_BUY, PUMP_FUN_PROGRAM_ID_PUBKEY};
use solana_sdk::pubkey::Pubkey;
use std::sync::LazyLock;

pub const D_LEN: usize = PUMP_FUN_BUY.len();
pub const ARGS_LEN: usize = D_LEN + 16;
pub const WRITABLE_MASK: u16 = (1 << 1) | (1 << 3) | (1 << 4) | (1 << 5) | (1 << 6);
pub static USER_VOLUME_ACCUMULATOR: LazyLock<Pubkey> = LazyLock::new(|| {
    Pubkey::find_program_address(
        &[b"user_volume_accumulator", SignerKeypair::pubkey().as_ref()],
        &PUMP_FUN_PROGRAM_ID_PUBKEY,
    )
    .0
});
