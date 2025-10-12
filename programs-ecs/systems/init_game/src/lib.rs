use bolt_lang::*;
use game::Game;
use player::Player;

declare_id!("5ZhDRGff5T4dyvdiDF6Mg729VLQ2EVU4ecqY3ni3PQGo");

#[system]
pub mod init_game {
    pub fn execute(ctx: Context<Components>, _args: Vec<u8>) -> Result<Components> {
        let game = &mut ctx.accounts.game;
        let player = &ctx.accounts.player;
        let clock = Clock::get()?;
        
        require!(player.is_logged_in, InitGameError::PlayerNotRegistered);
        require!(player.lobby_id.is_none(), InitGameError::PlayerAlreadyInLobby);
        require!(player.match_id.is_none(), InitGameError::PlayerInMatch);
        
        game.match_id = Pubkey::new_unique();
        game.lobby_id = Pubkey::new_unique();
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
pub enum InitGameError {
    #[msg("Player must be registered to create a lobby")]
    PlayerNotRegistered,
    #[msg("Player is already in a lobby")]
    PlayerAlreadyInLobby,
    #[msg("Player is already in a match")]
    PlayerInMatch,
}