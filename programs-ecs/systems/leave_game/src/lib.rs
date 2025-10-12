use bolt_lang::*;
use position::Position;

declare_id!("8QczrLnLiJeQDk3QjhTyba3KNGXN2Z9fM2Kg2H3hD5RG");

#[system]
pub mod leave_game {

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