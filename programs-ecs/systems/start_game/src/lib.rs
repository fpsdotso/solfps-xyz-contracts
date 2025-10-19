use bolt_lang::*;
use game::Game;
use player::Player;

declare_id!("3DUusbymEx6PDZbeHtiZK9vKzGf8M3ePDG7vYegSKBH7");

#[system]
pub mod start_game {

    pub fn execute(ctx: Context<Components>, args: Vec<u8>) -> Result<Components> {
        let game = &mut ctx.accounts.game;
        let player = &ctx.accounts.player;
        let clock = Clock::get()?;
        
        require!(game.game_state == 0, StartGameError::GameAlreadyStarted);

        let total_players = game.current_players_team_a + game.current_players_team_b;
        require!(total_players >= 2, StartGameError::NotEnoughPlayers);

        let is_lobby_owner = player.key() == game.created_by;
        let all_ready = game.ready_players >= total_players;

        require!(is_lobby_owner || all_ready, StartGameError::CannotStartGame);

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