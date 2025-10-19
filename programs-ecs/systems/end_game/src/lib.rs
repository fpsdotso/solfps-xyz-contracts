use bolt_lang::*;
use game::Game;

declare_id!("9WgqyxzyiCZpDSPSMJ1ef59LD1yrL23N5Yauje6eha54");

#[system]
pub mod end_game {
    pub fn execute(ctx: Context<Components>, _args_p: Vec<u8>) -> Result<Components> {
        let game = &mut ctx.accounts.game;
        
        require!(game.game_state == 1, EndGameError::GameNotInProgress);
        game.game_state = 2;
        game.match_end_timestamp = Some(Clock::get()?.unix_timestamp);

        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub game: Game,
    }

}

#[error_code]
pub enum EndGameError {
    #[msg("Game is not currently in progress")]
    GameNotInProgress,
}