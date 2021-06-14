// inside instruction.rs
use std::convert::TryInto;
use solana_program::program_error::ProgramError;

use crate::error::EscrowError::InvalidInstruction;

// inside instruction.rs
pub enum EscrowInstruction {

    /// Starts the trade by creating and populating an escrow account and transferring ownership of the given temp token account to the PDA
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the escrow
    /// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer
    InitEscrow {
        /// The amount party A expects to receive of token Y
        amount: u64
    }
}


impl EscrowInstruction {
    /// Unpacks a byte buffer
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            1 => Self::Build {
                amount: Self::build(rest)?
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn build(input: &[u8]) -> Result<u64, ProgramError> {

        Ok(1)
    }
}