use bolt_lang::*;
use weapon::Weapon;

declare_id!("FpY75Ly4uRawJUfmyKcLps9Z1Kytz6BvypwMyyHJWh6d");

#[system]
pub mod switch_weapon {

    pub fn execute(ctx: Context<Components>, _args_p: Vec<u8>) -> Result<Components> {
        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub weapon: Weapon,
    }

}