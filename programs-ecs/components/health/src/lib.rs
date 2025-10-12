use bolt_lang::*;

declare_id!("8c4sj72LjKi9azxGadFLQ89fvQrgQd3eiP8KfDHa67Rv");

#[component]
pub struct Health {
    pub current_hp: u32,             
    pub max_hp: u32,                 
    pub armor: u32,                  
    pub max_armor: u32,              
    pub is_alive: bool,              
    pub last_damage_timestamp: i64,
    pub last_damage_amount: u32,     
    pub last_damage_source: Option<Pubkey>, 
    pub respawn_timestamp: Option<i64>, 
    pub invulnerable_until: Option<i64>, 
}

impl Default for Health {
    fn default() -> Self {
        Self::new(HealthInit{
            current_hp: 100,
            max_hp: 100,
            armor: 50,
            max_armor: 50,
            is_alive: true,
            last_damage_timestamp: 0,
            last_damage_amount: 0,
            last_damage_source: None,
            respawn_timestamp: None,
            invulnerable_until: None,
        }) 
    }
}