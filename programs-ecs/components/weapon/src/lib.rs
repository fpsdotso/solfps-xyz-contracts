use bolt_lang::*;

declare_id!("CBbM9mKimGEoMiKoY1bUiAjURTDcqC3k6qr1iXxLfSzk");

#[component]
pub struct Weapon {
    pub primary_weapon: Pubkey,          
    pub secondary_weapon: Pubkey,        
    pub current_weapon: u8,           // Currently equipped weapon (1=primary, 2=secondary)
    pub primary_ammo: u32,            
    pub primary_ammo_reserve: u32,    
    pub secondary_ammo: u32,            
    pub secondary_ammo_reserve: u32,  
    pub primary_damage: u32,          
    pub secondary_damage: u32,        
    pub can_switch_weapon: bool,
    pub reload_time: u32,
    
    // NEW: Shooting mechanics
    pub last_shot_timestamp: i64,
    pub is_reloading: bool,
    pub reload_start_timestamp: Option<i64>,
}

impl Default for Weapon {
    fn default() -> Self {
        Self::new(WeaponInit{
            primary_weapon: Pubkey::default(),
            secondary_weapon: Pubkey::default(),
            current_weapon: 1,
            primary_ammo: 30,
            primary_ammo_reserve: 90,
            secondary_ammo: 15,
            secondary_ammo_reserve: 45,
            primary_damage: 25,
            secondary_damage: 50,
            reload_time: 2000,
            can_switch_weapon: true,
            last_shot_timestamp: 0,
            is_reloading: false,
            reload_start_timestamp: None,
        }) 
    }
}