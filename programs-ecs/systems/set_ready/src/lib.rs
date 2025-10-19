use bolt_lang::*;
use player::Player;
use game::Game;

declare_id!("ApjerCa4TNZHEPHheK8XSkVdsKdYqq6Yg6fMviECxSqx");

#[system]
pub mod set_ready {
    pub fn execute(ctx: Context<Components>, args: Vec<u8>) -> Result<Components> {
        // Args: [is_ready(u8)] - 0=not ready, 1=ready
        require!(args.len() >= 1, SetReadyError::InvalidArgs);
        
        let is_ready = args[0] == 1;
        let player = &mut ctx.accounts.player;
        let game = &mut ctx.accounts.game;
        
        require!(player.has_logged_in, SetReadyError::PlayerNotRegistered);
        require!(player.current_game.is_some(), SetReadyError::PlayerNotInGame);
        require!(player.current_game.unwrap() == game.key(), SetReadyError::PlayerNotInThisGame);
        require!(game.game_state == 0, SetReadyError::GameAlreadyStarted);
        
        // Update ready count
        if is_ready && !player.is_ready {
            game.ready_players += 1;
        } else if !is_ready && player.is_ready {
            game.ready_players = game.ready_players.saturating_sub(1);
        }
        
        player.is_ready = is_ready;
        
        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub player: Player,
        pub game: Game,
    }
}

#[error_code]
pub enum SetReadyError {
    #[msg("Invalid arguments")]
    InvalidArgs,
    #[msg("Player is not registered")]
    PlayerNotRegistered,
    #[msg("Player is not in a game")]
    PlayerNotInGame,
    #[msg("Player is not in this specific game")]
    PlayerNotInThisGame,
    #[msg("Game has already started")]
    GameAlreadyStarted,
}
