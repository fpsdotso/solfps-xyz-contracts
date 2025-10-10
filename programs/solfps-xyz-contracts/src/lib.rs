use bolt_lang::prelude::*;

declare_id!("2Z86V9gUcrMREK997BALjyKwo73MB34FAFiMfhKti5Da");

#[program]
pub mod solfps_xyz_contracts {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
