use bolt_lang::*;
use position::Position;

declare_id!("FVZKXQwmxnnKBhiyvBU9psjg3RdmGz87hotvGV18V2un");

#[system]
pub mod respawn {

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