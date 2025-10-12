use bolt_lang::*;
use position::Position;

declare_id!("GYpCrumupdHMPpke9fZf5Y66WfX2sYKs1xx22yRhYpoq");

#[system]
pub mod apply_damage {

    pub fn execute(ctx: Context<Components>, _args_p: Vec<u8>) -> Result<Components> {
        let position = &mut ctx.accounts.position;
        position.x += 1;
        position.y += 1;
        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub position: Position,
    }

}