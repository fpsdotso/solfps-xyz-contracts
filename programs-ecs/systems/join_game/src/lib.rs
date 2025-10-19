use bolt_lang::*;
use player::Player;
use game::Game;

declare_id!("H2ezdHmHvnoQc5T7RCTh8tEEfmvCaSHD7trSAMsVxBAv");

#[system]
pub mod join_game {

    pub fn execute(ctx: Context<Components>, _args_p: Vec<u8>) -> Result<Components> {
        let player = &mut ctx.accounts.player;
        let game = &mut ctx.accounts.game;
        
        require!(player.has_logged_in, JoinGameError::PlayerNotRegistered);
        require!(player.current_game.is_none(), JoinGameError::PlayerAlreadyInGame);
        require!(game.game_state == 0, JoinGameError::GameAlreadyStarted);
        
        // Check if game has space
        let total_players = game.current_players_team_a + game.current_players_team_b;
        require!(total_players < (game.max_players_per_team * 2) as u8, JoinGameError::GameFull);
        
        player.is_alive = true;
        player.team = if game.current_players_team_a <= game.current_players_team_b { 1 } else { 2 };
        player.current_game = Some(game.key());
        player.is_ready = false; // Reset ready state when joining
        
        if player.team == 1 {
            game.current_players_team_a += 1;
        } else {
            game.current_players_team_b += 1;
        }
        
        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub player: Player,
        pub game: Game,
    }

}

#[error_code]
pub enum JoinGameError {
    #[msg("Player is not registered")]
    PlayerNotRegistered,
    #[msg("Player is already in a game")]
    PlayerAlreadyInGame,
    #[msg("Game has already started")]
    GameAlreadyStarted,
    #[msg("Game is full")]
    GameFull,
}   