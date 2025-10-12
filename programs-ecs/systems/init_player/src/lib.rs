use bolt_lang::*;
use player::Player;
use health::Health;
use weapon::Weapon;
use player_stats::PlayerStats;

declare_id!("DdfNp9wUnuuuTPN1uihkATQUNj9Hzxj6xuZQsBamfzr3");

#[system]
pub mod init_player {
    pub fn execute(ctx: Context<Components>, args: Vec<u8>) -> Result<Components> {
        let player = &mut ctx.accounts.player;
        let clock = Clock::get()?;
        
        require!(!player.has_logged_in, InitPlayerError::AlreadyLoggedIn);
        
        let username = if args.is_empty() {
            let timestamp_suffix = (clock.unix_timestamp as u32) % 100000;
            format!("Player{}", timestamp_suffix)
        } else {
            let username_len = args[0] as usize;
            if username_len == 0 || username_len > 32 || args.len() < 1 + username_len {
                return Err(InitPlayerError::InvalidUsername.into());
            }
            
            let username_bytes = &args[1..1 + username_len];
            String::from_utf8(username_bytes.to_vec())
                .map_err(|_| InitPlayerError::InvalidUsername)?
        };
        
        require!(username.len() >= 3 && username.len() <= 32, InitPlayerError::InvalidUsernameLength);
        
        player.authority = Pubkey::new_unique();
        player.username = username;
        player.has_logged_in = true;

        // is_alive is true when player joins a game
        player.is_alive = false;
        player.team = 0;
        player.lobby_id = None;
        player.match_id = None;
        player.last_login_timestamp = clock.unix_timestamp;
        player.total_matches_played = 0;
        player.level = 1;
        
        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub player: Player,
        pub health: Health,
        pub weapon: Weapon,
        pub player_stats: PlayerStats,
    }

}

#[error_code]
pub enum InitPlayerError {
    #[msg("Player is already logged in")]
    AlreadyLoggedIn,
    #[msg("Invalid username format")]
    InvalidUsername,
    #[msg("Username must be between 3-32 characters")]
    InvalidUsernameLength,
}