use bolt_lang::*;
use game::Game;
use player::Player;

declare_id!("5ZhDRGff5T4dyvdiDF6Mg729VLQ2EVU4ecqY3ni3PQGo");

#[system]
pub mod init_game {
    pub fn execute(ctx: Context<Components>, _args: Vec<u8>) -> Result<Components> {
        let game = &mut ctx.accounts.game;
        let player = &mut ctx.accounts.player;
        let clock = Clock::get()?;
        
        require!(player.has_logged_in, InitGameError::PlayerNotRegistered);
        require!(player.current_game.is_none(), InitGameError::PlayerAlreadyInGame);
        
        // Initialize game state - the game account itself is the PDA that tracks the room
        game.match_start_timestamp = clock.unix_timestamp;
        game.game_state = 0; // waiting state
        game.team_a_score = 0;
        game.team_b_score = 0;
        game.current_players_team_a = 0;
        game.current_players_team_b = 0;
        game.match_duration = 300; // 5 minutes
        game.max_players_per_team = 5;
        game.match_type = 1; // team deathmatch
        game.map_name = "New Arena".to_string();
        
        // NEW: Initialize lobby features
        game.lobby_name = "New Game Room".to_string();
        game.created_by = player.key();
        game.is_private = false;
        game.ready_players = 0;
        game.map_selection = 0;
        
        // Set player as the first player in the game
        player.team = 0; // No team assigned yet
        // Set the current game PDA for the player to track this room
        player.current_game = Some(ctx.accounts.game.key());

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
    #[msg("Player must be registered to create a game")]
    PlayerNotRegistered,
    #[msg("Player is already in a game")]
    PlayerAlreadyInGame,
}