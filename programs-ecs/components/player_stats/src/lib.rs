use bolt_lang::*;

declare_id!("3A36U7Y8PqfKN83LdRdPHqTHYdvn3vV1hB8BRcmaBkxK");

#[component]
pub struct PlayerStats {
    pub kills: u32,                  
    pub deaths: u32,                 
    pub assists: u32,                
    pub headshots: u32,              
    pub damage_taken: u32,            
    pub damage_dealt: u32,            
    pub round_wins: u32,             
    pub kda_ratio: f32,              
    pub kill_streak: u32,            
    pub highest_kill_streak: u32,       
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self::new(PlayerStatsInit{
            kills: 0,
            deaths: 0,
            assists: 0,
            headshots: 0,
            damage_dealt: 0,
            damage_taken: 0,
            round_wins: 0,
            kda_ratio: 0.0,
            kill_streak: 0,
            highest_kill_streak: 0,
        }) 
    }
}