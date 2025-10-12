use bolt_lang::*;
use player::Player;
use health::Health;
use position::Position;

declare_id!("FVZKXQwmxnnKBhiyvBU9psjg3RdmGz87hotvGV18V2un");

#[system]
pub mod respawn {

    pub fn execute(ctx: Context<Components>, _args: Vec<u8>) -> Result<Components> {
        let player = &mut ctx.accounts.player;
        let health = &mut ctx.accounts.health;
        let position = &mut ctx.accounts.position;
        let clock = Clock::get()?;
        
        require!(player.has_logged_in, RespawnError::PlayerNotLoggedIn);
        require!(player.lobby_id.is_some(), RespawnError::PlayerNotInLobby);
        require!(player.match_id.is_some(), RespawnError::PlayerNotInMatch);
        require!(!player.is_alive, RespawnError::PlayerAlreadyAlive);
        
        if let Some(respawn_time) = health.respawn_timestamp {
            let time_since_death = clock.unix_timestamp - respawn_time;
            require!(time_since_death >= 5, RespawnError::RespawnCooldownActive);
        } else {
            return Err(RespawnError::NoDeathRecord.into());
        }
        
        player.is_alive = true;
        health.is_alive = true;
        health.current_hp = health.max_hp;
        health.armor = 0;
        health.respawn_timestamp = None;
        health.invulnerable_until = Some(clock.unix_timestamp + 3);
        
        let spawn_point_x = match player.team {
            1 => 10.0,  // Team A spawn
            2 => -10.0, // Team B spawn
            _ => 0.0,   // Fallback spawn
        };
        
        position.x = spawn_point_x;
        position.y = 0.0;
        position.z = 0.0;
        position.velocity_x = 0.0;
        position.velocity_y = 0.0;
        position.velocity_z = 0.0;
        position.is_moving = false;
        position.is_jumping = false;
        
        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub player: Player,
        pub health: Health,
        pub position: Position,
    }

}

#[error_code]
pub enum RespawnError {
    #[msg("Player is not logged in")]
    PlayerNotLoggedIn,
    #[msg("Player is not in a lobby")]
    PlayerNotInLobby,
    #[msg("Player is not in a match")]
    PlayerNotInMatch,
    #[msg("Player is already alive")]
    PlayerAlreadyAlive,
    #[msg("Must wait 5 seconds before respawning")]
    RespawnCooldownActive,
    #[msg("No death record found")]
    NoDeathRecord,
}