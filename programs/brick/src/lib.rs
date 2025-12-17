use anchor_lang::prelude::*;

declare_id!("4sbjN16fgtnGMFaJ6s7aw9ha1uUQ9qLBv5gHSui2aNUU");

#[program]
pub mod brick {
    use super::*;

    pub fn fallback(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> Result<()> {
        Err(ErrorCode::ProgramDisabled.into())
    }

    pub fn initialize(_ctx: Context<Initialize>, _val: u64) -> Result<()> {
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("This program is temporarily disabled.")]
    ProgramDisabled,
}

#[derive(Accounts)]
pub struct Initialize {}
