use bolt_lang::*;
use position::Position;

declare_id!("5ZhDRGff5T4dyvdiDF6Mg729VLQ2EVU4ecqY3ni3PQGo");

#[system]
pub mod init_game {

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