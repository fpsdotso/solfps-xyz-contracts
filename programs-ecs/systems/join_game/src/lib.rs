use bolt_lang::*;
use player::Player;
use game::Game;

declare_id!("H2ezdHmHvnoQc5T7RCTh8tEEfmvCaSHD7trSAMsVxBAv");

#[system]
pub mod join_game {

    pub fn execute(ctx: Context<Components>, _args_p: Vec<u8>) -> Result<Components> {
        let player = &mut ctx.accounts.player;
        let game = &mut ctx.accounts.game;
        require!(player.lobby_id.is_some(), JoinGameError::PlayerAlreadyInGame);
        require!(player.match_id.is_some(), JoinGameError::PlayerAlreadyInGame);
        player.is_alive = true;
        player.has_logged_in = true;
        player.team = if game.current_players_team_a <= game.current_players_team_b { 1 } else { 2 };
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
    #[msg("Player is already in a game")]
    PlayerAlreadyInGame,
}   