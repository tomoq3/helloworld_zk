#![no_std]

// use core::ffi::{FromBytesUntilNulError, FromBytesWithNulError};

use pinocchio::{AccountView, Address, ProgramResult, address::address, entrypoint, error::ProgramError, nostd_panic_handler};

use crate::instructions::verify::Verify;
entrypoint!(process_instruction);
nostd_panic_handler!();

pub mod state;
pub mod instructions;
pub const ID: Address = address!("22222222222222222222222222222222222222222222");

fn process_instruction(
    _program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((Verify::DISCRIMINATOR, data)) => Verify::try_from((data, accounts))?.process(),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
