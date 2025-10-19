use bolt_lang::*;
use player::Player;
use weapon::Weapon;

declare_id!("8UWDB2GtMLKkF1Xx1a54x8MtbhzyTTX8RuoSXBG3xrsN");

#[system]
pub mod reload {
    pub fn execute(ctx: Context<Components>, args: Vec<u8>) -> Result<Components> {
        // Args: [weapon_slot(u8)] - 1=primary, 2=secondary
        require!(args.len() >= 1, ReloadError::InvalidArgs);
        
        let weapon_slot = args[0];
        let player = &ctx.accounts.player;
        let weapon = &mut ctx.accounts.weapon;
        let current_time = Clock::get()?.unix_timestamp;
        
        require!(player.is_alive, ReloadError::PlayerNotAlive);
        require!(weapon_slot == 1 || weapon_slot == 2, ReloadError::InvalidWeaponSlot);
        
        // If already reloading, check if reload is complete
        if weapon.is_reloading {
            if let Some(reload_start) = weapon.reload_start_timestamp {
                let elapsed = (current_time - reload_start) as u32 * 1000; // Convert to ms
                if elapsed >= weapon.reload_time {
                    // Complete reload
                    if weapon_slot == 1 {
                        let ammo_needed = 30 - weapon.primary_ammo;
                        let ammo_to_reload = ammo_needed.min(weapon.primary_ammo_reserve);
                        weapon.primary_ammo += ammo_to_reload;
                        weapon.primary_ammo_reserve -= ammo_to_reload;
                    } else {
                        let ammo_needed = 15 - weapon.secondary_ammo;
                        let ammo_to_reload = ammo_needed.min(weapon.secondary_ammo_reserve);
                        weapon.secondary_ammo += ammo_to_reload;
                        weapon.secondary_ammo_reserve -= ammo_to_reload;
                    }
                    weapon.is_reloading = false;
                    weapon.reload_start_timestamp = None;
                } else {
                    return Err(ReloadError::ReloadInProgress.into());
                }
            }
        } else {
            // Start reload
            let needs_reload = match weapon_slot {
                1 => weapon.primary_ammo < 30 && weapon.primary_ammo_reserve > 0,
                2 => weapon.secondary_ammo < 15 && weapon.secondary_ammo_reserve > 0,
                _ => false,
            };
            
            require!(needs_reload, ReloadError::NoReloadNeeded);
            
            weapon.is_reloading = true;
            weapon.reload_start_timestamp = Some(current_time);
        }
        
        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub player: Player,
        pub weapon: Weapon,
    }
}

#[error_code]
pub enum ReloadError {
    #[msg("Invalid arguments")]
    InvalidArgs,
    #[msg("Player is not alive")]
    PlayerNotAlive,
    #[msg("Invalid weapon slot")]
    InvalidWeaponSlot,
    #[msg("Reload already in progress")]
    ReloadInProgress,
    #[msg("Weapon is full or no reserve ammo")]
    NoReloadNeeded,
}
