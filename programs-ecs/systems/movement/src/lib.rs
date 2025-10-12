use bolt_lang::*;
use position::Position;
use player::Player;

declare_id!("5UiPWCCSbCWu5YdkFhe36MK5YL11yLhojrtT7mtGfP7j");

#[system]
pub mod movement {

    /**
     * Args format:
     * [0] - movement flags (u8)
     * [1..5] - rotation_x (f32)
     * [5] - is_sprinting (u8, 0 or 1)
     */
    pub fn execute(ctx: Context<Components>, args: Vec<u8>) -> Result<Components> {
        let player = &ctx.accounts.player;
        let position = &mut ctx.accounts.position;
        
        if player.is_alive == false {
            return Err(MovementError::PlayerNotAlive.into());
        }

        if args.len() < 6 {
            return Err(MovementError::InvalidInput.into());
        }
        
        let movement_flags = args[0];
        let rotation_x = f32::from_le_bytes([args[1], args[2], args[3], args[4]]);
        let is_sprinting = args[5] == 1;
        
        let forward = (movement_flags & 0x01) != 0;
        let backward = (movement_flags & 0x02) != 0;
        let left = (movement_flags & 0x04) != 0;
        let right = (movement_flags & 0x08) != 0;
        let jump = (movement_flags & 0x10) != 0;
        
        if is_sprinting && (backward || left || right) {
            return Err(MovementError::InvalidInput.into());
        }

        let base_speed = if is_sprinting { 7.0 } else { 4.0 };
        let jump_force = 10.0;
        let gravity = -9.8;
        let dt = 0.016;
        
        let mut move_x = 0.0;
        let mut move_y = 0.0;
        
        if forward { move_y += base_speed; }
        if backward { move_y -= base_speed; }
        if left { move_x -= base_speed; }
        if right { move_x += base_speed; }
        
        let magnitude = ((move_x as f32) * (move_x as f32) + (move_y as f32) * (move_y as f32)).sqrt();
        if magnitude > 0.0 {
            move_x = (move_x / magnitude) * base_speed;
            move_y = (move_y / magnitude) * base_speed;
        }
        
        position.velocity_x = move_x;
        position.velocity_y = move_y;
        
        if jump && !position.is_jumping && position.z <= 0.0 {
            position.velocity_z = jump_force;
            position.is_jumping = true;
        }
        
        if position.is_jumping {
            position.velocity_z += gravity * dt;
        }
        
        position.x += (position.velocity_x * dt) as f64;
        position.y += (position.velocity_y * dt) as f64;
        position.z += (position.velocity_z * dt) as f64;
        
        if position.z <= 0.0 {
            position.z = 0.0;
            position.velocity_z = 0.0;
            position.is_jumping = false;
        }
        
        position.rotation_x = rotation_x;
        position.is_moving = magnitude > 0.0;
        
        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub position: Position,
        pub player: player::Player,
    }

}

#[error_code]
pub enum MovementError {
    #[msg("Invalid movement input")]
    InvalidInput,
    #[msg("Player is not alive")]
    PlayerNotAlive,
}



	