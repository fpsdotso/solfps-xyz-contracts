use bolt_lang::*;

declare_id!("hfbkKtwhWiTnCySqtkVwoti1AF7Xv3MYRdwxmXA1WeD");

#[component]
pub struct Player {
    pub authority: Pubkey,          
    #[max_len(32)]
    pub username: String,            
    pub has_logged_in: bool,          
    pub team: u8,                    // 0 = no team, 1 = Team A, 2 = Team B
    pub lobby_id: Option<Pubkey>,
    pub match_id: Option<Pubkey>,    
    pub is_alive: bool,              
    pub last_login_timestamp: i64,   
    pub total_matches_played: u32,   
    pub level: u32,                  
}

impl Default for Player {
    fn default() -> Self {
        Self::new(PlayerInit{
            authority: Pubkey::default(),
            username: "Player".to_string(),
            has_logged_in: false,
            team: 0,
            lobby_id: None,
            match_id: None,
            is_alive: true,
            last_login_timestamp: 0,
            total_matches_played: 0,
            level: 1,
        }) 
    }
}