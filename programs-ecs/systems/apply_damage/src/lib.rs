use bolt_lang::*;
use player::Player;

declare_id!("GYpCrumupdHMPpke9fZf5Y66WfX2sYKs1xx22yRhYpoq");

#[system]
pub mod apply_damage {

    pub fn execute(ctx: Context<Components>, _args_p: Vec<u8>) -> Result<Components> {
        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub player: Player
    }

}