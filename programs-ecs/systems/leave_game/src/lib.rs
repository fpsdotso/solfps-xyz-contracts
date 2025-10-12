use bolt_lang::*;
use game::Game;
use player::Player;

declare_id!("8QczrLnLiJeQDk3QjhTyba3KNGXN2Z9fM2Kg2H3hD5RG");

#[system]
pub mod leave_game {

    pub fn execute(ctx: Context<Components>, _args: Vec<u8>) -> Result<Components> {
        let game = &mut ctx.accounts.game;
        let player = &mut ctx.accounts.player;
        
        require!(player.is_logged_in, LeaveGameError::PlayerNotInGame);
        
        if player.team == 1 {
            game.current_players_team_a = game.current_players_team_a.saturating_sub(1);
        } else if player.team == 2 {
            game.current_players_team_b = game.current_players_team_b.saturating_sub(1);
        }
        
        player.is_logged_in = false;
        player.is_alive = false;
        player.team = 0;
        player.lobby_id = None;
        player.match_id = None;
        
        let total_players = game.current_players_team_a + game.current_players_team_b;
        if total_players == 0 && game.game_state == 1 {
            game.game_state = 2;
            game.match_end_timestamp = Some(Clock::get()?.unix_timestamp);
        }
        
        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub game: Game,
        pub player: Player,
    }

}

#[error_code]
pub enum LeaveGameError {
    #[msg("Player is not currently in a game")]
    PlayerNotInGame,
}