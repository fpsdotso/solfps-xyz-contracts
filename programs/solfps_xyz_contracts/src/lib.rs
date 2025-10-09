use anchor_lang::prelude::*;

declare_id!("3GZWaipTS7mqc8X3TYDV9MaeP2kCpbwuzrrVgsZxYKEN");

#[program]
pub mod solfps_xyz_contracts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
