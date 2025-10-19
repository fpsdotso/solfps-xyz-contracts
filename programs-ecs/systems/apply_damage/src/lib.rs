use bolt_lang::*;
use player::Player;
use health::Health;
use weapon::Weapon;
use player_stats::PlayerStats;
use position::Position;
use game::Game;

declare_id!("GYpCrumupdHMPpke9fZf5Y66WfX2sYKs1xx22yRhYpoq");

#[system]
pub mod apply_damage {
    pub fn execute(ctx: Context<Components>, args: Vec<u8>) -> Result<Components> {
        // Args format: [weapon_type(u8), is_headshot(u8), distance(f32 as 4 bytes)]
        require!(args.len() >= 6, ApplyDamageError::InvalidArgs);
        
        let weapon_type = args[0];
        let is_headshot = args[1] == 1;
        let distance_bytes = [args[2], args[3], args[4], args[5]];
        let distance = f32::from_le_bytes(distance_bytes);
        
        let attacker = &mut ctx.accounts.attacker;
        let attacker_weapon = &ctx.accounts.attacker_weapon;
        let victim = &mut ctx.accounts.victim;
        let victim_health = &mut ctx.accounts.victim_health;
        let attacker_stats = &mut ctx.accounts.attacker_stats;
        let victim_stats = &mut ctx.accounts.victim_stats;
        let game = &mut ctx.accounts.game;
        
        require!(victim.is_alive, ApplyDamageError::VictimAlreadyDead);
        require!(attacker.current_game.is_some(), ApplyDamageError::AttackerNotInGame);
        require!(victim.current_game.is_some(), ApplyDamageError::VictimNotInGame);
        require!(attacker.current_game == victim.current_game, ApplyDamageError::PlayersNotInSameGame);
        require!(attacker.team != victim.team, ApplyDamageError::FriendlyFire);
        
        let current_time = Clock::get()?.unix_timestamp;
        
        // Check invulnerability
        if let Some(invuln_time) = victim_health.invulnerable_until {
            if current_time < invuln_time {
                return Err(ApplyDamageError::VictimInvulnerable.into());
            }
        }
        
        // Calculate base damage from weapon
        let base_damage = match weapon_type {
            1 => attacker_weapon.primary_damage,
            2 => attacker_weapon.secondary_damage,
            _ => return Err(ApplyDamageError::InvalidWeaponType.into()),
        };
        
        // Apply headshot multiplier
        let damage = if is_headshot {
            base_damage * 2
        } else {
            base_damage
        };
        
        // Apply armor reduction
        let damage_to_health = if victim_health.armor > 0 {
            let armor_absorbed = damage / 2;
            let armor_damage = armor_absorbed.min(victim_health.armor);
            victim_health.armor = victim_health.armor.saturating_sub(armor_damage);
            damage - armor_absorbed
        } else {
            damage
        };
        
        // Apply damage to health
        let killed = if victim_health.current_hp > damage_to_health {
            victim_health.current_hp -= damage_to_health;
            false
        } else {
            victim_health.current_hp = 0;
            victim_health.is_alive = false;
            victim.is_alive = false;
            victim_health.respawn_timestamp = Some(current_time + 5);
            true
        };
        
        // Update stats
        victim_health.last_damage_timestamp = current_time;
        victim_health.last_damage_amount = damage;
        victim_health.last_damage_source = Some(attacker.key());
        
        attacker_stats.damage_dealt += damage;
        victim_stats.damage_taken += damage;
        
        // Handle kill
        if killed {
            attacker_stats.kills += 1;
            attacker_stats.kill_streak += 1;
            if attacker_stats.kill_streak > attacker_stats.highest_kill_streak {
                attacker_stats.highest_kill_streak = attacker_stats.kill_streak;
            }
            if is_headshot {
                attacker_stats.headshots += 1;
            }
            
            victim_stats.deaths += 1;
            victim_stats.kill_streak = 0;
            
            // Update game score
            if attacker.team == 1 {
                game.team_a_score += 1;
            } else if attacker.team == 2 {
                game.team_b_score += 1;
            }
            
            // Kill tracking handled by frontend via PlayerStats changes
        }
        
        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub attacker: Player,
        pub attacker_weapon: Weapon,
        pub attacker_stats: PlayerStats,
        pub victim: Player,
        pub victim_health: Health,
        pub victim_stats: PlayerStats,
        pub game: Game,
    }
}

#[error_code]
pub enum ApplyDamageError {
    #[msg("Invalid arguments provided")]
    InvalidArgs,
    #[msg("Victim is already dead")]
    VictimAlreadyDead,
    #[msg("Attacker is not in a game")]
    AttackerNotInGame,
    #[msg("Victim is not in a game")]
    VictimNotInGame,
    #[msg("Players are not in the same game")]
    PlayersNotInSameGame,
    #[msg("Cannot damage teammates")]
    FriendlyFire,
    #[msg("Victim is invulnerable")]
    VictimInvulnerable,
    #[msg("Invalid weapon type")]
    InvalidWeaponType,
}