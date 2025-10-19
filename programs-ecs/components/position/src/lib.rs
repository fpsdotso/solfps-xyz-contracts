use bolt_lang::*;

declare_id!("34idayqAQEUBFEQoshs4ZxUDMMaeoGwgGuNA2dN71xFH");

#[component(delegate)]
pub struct Position {
    pub x: f64,                     
    pub y: f64,                     
    pub z: f64,                     
    pub rotation_x: f32,            
    pub rotation_y: f32,             
    pub velocity_x: f32,             
    pub velocity_y: f32,              
    pub velocity_z: f32,              
    pub is_jumping: bool,            
    pub is_moving: bool,              
    pub spawn_point_id: u8,          
}

impl Default for Position {
    fn default() -> Self {
        Self::new(PositionInit{
            x: 0.0,
            y: 0.0,
            z: 0.0,
            rotation_x: 0.0,
            rotation_y: 0.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            velocity_z: 0.0,
            is_moving: false,
            is_jumping: false,
            spawn_point_id: 0,
        }) 
    }
}