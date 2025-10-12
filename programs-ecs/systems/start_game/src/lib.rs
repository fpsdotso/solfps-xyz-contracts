use bolt_lang::*;
use game::Game;
use player::Player;

declare_id!("3DUusbymEx6PDZbeHtiZK9vKzGf8M3ePDG7vYegSKBH7");

#[system]
pub mod start_game {

    pub fn execute(ctx: Context<Components>, args: Vec<u8>) -> Result<Components> {
        let game = &mut ctx.accounts.game;
        let clock = Clock::get()?;
        
        require!(game.game_state == 0, StartGameError::GameAlreadyStarted);
        
        let is_lobby_leader = if args.is_empty() { false } else { args[0] == 1 };
        let total_players = game.current_players_team_a + game.current_players_team_b;
        
        let can_start = is_lobby_leader || total_players >= 10;
        require!(can_start, StartGameError::CannotStartGame);
        
        require!(total_players >= 2, StartGameError::NotEnoughPlayers);
        
        game.game_state = 1;
        game.match_start_timestamp = clock.unix_timestamp;
        
        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub game: Game,
        pub player: Player,
    }

}

#[error_code]
pub enum StartGameError {
    #[msg("Game has already started")]
    GameAlreadyStarted,
    #[msg("Cannot start game - need lobby leader or 10 players")]
    CannotStartGame,
    #[msg("Not enough players to start game")]
    NotEnoughPlayers,
}