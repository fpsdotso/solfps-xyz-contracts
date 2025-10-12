use bolt_lang::*;
use position::Position;

declare_id!("FpY75Ly4uRawJUfmyKcLps9Z1Kytz6BvypwMyyHJWh6d");

#[system]
pub mod switch_weapon {

    pub fn execute(ctx: Context<Components>, _args_p: Vec<u8>) -> Result<Components> {
        let position = &mut ctx.accounts.position;
        position.x += 1.0;
        position.y += 1.0;
        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub position: Position,
    }

}