use bolt_lang::*;

declare_id!("GkWTKgPGiNhG5LFdXWCZdDrqFKfBvhVQ8D5cKpn1M3Y8");

#[component]
pub struct Game {
    pub match_id: Pubkey,           
    pub lobby_id: Pubkey,            
    pub team_a_score: u32,           
    pub team_b_score: u32,           
    pub match_duration: u32,         
    pub match_start_timestamp: i64,  
    pub match_end_timestamp: Option<i64>,  
    pub game_state: u8,             // 0=waiting, 1=active, 2=ended, 3=paused
    pub max_players_per_team: u8,    
    pub current_players_team_a: u8,  
    pub current_players_team_b: u8, 
    pub winning_team: Option<u8>,   // Winning team (0=draw, 1=team_a, 2=team_b)
    pub match_type: u8,             // Match type (1=team_deathmatch) for now
    #[max_len(50)]
    pub map_name: String,            
}

impl Default for Game {
    fn default() -> Self {
        Self::new(GameInit{
            match_id: Pubkey::default(),
            lobby_id: Pubkey::default(),
            team_a_score: 0,
            team_b_score: 0,
            match_duration: 300, // 5 minutes a match
            match_start_timestamp: 0,
            match_end_timestamp: None,
            game_state: 0,
            max_players_per_team: 5,
            current_players_team_a: 0,
            current_players_team_b: 0,
            winning_team: None,
            match_type: 1, // Default to team deathmatch
            map_name: "New Arena".to_string(),
        }) 
    }
}