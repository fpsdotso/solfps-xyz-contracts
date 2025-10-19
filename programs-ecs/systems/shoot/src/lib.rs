use bolt_lang::*;
use player::Player;
use weapon::Weapon;
use position::Position;

declare_id!("FDCpMdKzRvkgBnn6BERa9DtqUJY6Fxj3xjsRxFPkhJVh");

#[system]
pub mod shoot {
    pub fn execute(ctx: Context<Components>, args: Vec<u8>) -> Result<Components> {
        // Args: [weapon_slot(u8)] - 1=primary, 2=secondary
        require!(args.len() >= 1, ShootError::InvalidArgs);
        
        let weapon_slot = args[0];
        let player = &ctx.accounts.player;
        let weapon = &mut ctx.accounts.weapon;
        let current_time = Clock::get()?.unix_timestamp;
        
        require!(player.is_alive, ShootError::PlayerNotAlive);
        require!(player.current_game.is_some(), ShootError::PlayerNotInGame);
        require!(!weapon.is_reloading, ShootError::WeaponReloading);
        require!(weapon_slot == 1 || weapon_slot == 2, ShootError::InvalidWeaponSlot);
        
        // Check ammo
        let has_ammo = match weapon_slot {
            1 => weapon.primary_ammo > 0,
            2 => weapon.secondary_ammo > 0,
            _ => false,
        };
        require!(has_ammo, ShootError::NoAmmo);
        
        // Check fire rate (simple cooldown based on weapon)
        let fire_rate_ms = weapon.reload_time / 10; // Simple fire rate calculation
        if current_time - weapon.last_shot_timestamp < (fire_rate_ms as i64) / 1000 {
            return Err(ShootError::FireRateLimited.into());
        }
        
        // Consume ammo
        if weapon_slot == 1 {
            weapon.primary_ammo -= 1;
        } else {
            weapon.secondary_ammo -= 1;
        }
        
        weapon.last_shot_timestamp = current_time;
        weapon.current_weapon = weapon_slot;
        
        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub player: Player,
        pub weapon: Weapon,
        pub position: Position,
    }
}

#[error_code]
pub enum ShootError {
    #[msg("Invalid arguments")]
    InvalidArgs,
    #[msg("Player is not alive")]
    PlayerNotAlive,
    #[msg("Player is not in a game")]
    PlayerNotInGame,
    #[msg("Weapon is currently reloading")]
    WeaponReloading,
    #[msg("Invalid weapon slot")]
    InvalidWeaponSlot,
    #[msg("No ammo remaining")]
    NoAmmo,
    #[msg("Fire rate limited - too fast")]
    FireRateLimited,
}