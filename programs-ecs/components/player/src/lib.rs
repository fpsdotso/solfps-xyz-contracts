use bolt_lang::*;

declare_id!("hfbkKtwhWiTnCySqtkVwoti1AF7Xv3MYRdwxmXA1WeD");

#[component(delegate)]
pub struct Player {
    pub authority: Pubkey,          
    #[max_len(32)]
    pub username: String,            
    pub has_logged_in: bool,          
    pub team: u8,                    // 0 = no team, 1 = Team A, 2 = Team B
    pub current_game: Option<Pubkey>, // PDA of the current game the player is in
    pub is_alive: bool,              
    pub last_login_timestamp: i64,   
    pub total_matches_played: u32,   
    pub level: u32,
    
    // NEW: Lobby state
    pub is_ready: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self::new(PlayerInit{
            authority: Pubkey::default(),
            username: "Player".to_string(),
            has_logged_in: false,
            team: 0,
            current_game: None,
            is_alive: true,
            last_login_timestamp: 0,
            total_matches_played: 0,
            level: 1,
            is_ready: false,
        }) 
    }
}