use bolt_lang::*;
use position::Position;

declare_id!("3DUusbymEx6PDZbeHtiZK9vKzGf8M3ePDG7vYegSKBH7");

#[system]
pub mod start_game {

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