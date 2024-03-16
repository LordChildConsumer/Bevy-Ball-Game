use bevy::prelude::*;

mod components;
mod systems;


pub const SPEED: f32 = 500.0;
pub const RADIUS: f32 = 32.0;
pub const Z_INDEX: f32 = 1.0;


// Imports
use systems::*;




pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        
        // Startup
        app.add_systems(Startup, spawn_player);

        // Update
        app.add_systems(Update, (
            move_player,
            // confine_player,
            collide_with_enemy,
            collide_with_star,
        ));

    }
}