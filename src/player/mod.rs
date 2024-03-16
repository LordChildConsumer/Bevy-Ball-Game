use bevy::prelude::*;

mod components;
mod systems;


pub const SPEED: f32 = 500.0;
pub const RADIUS: f32 = 32.0;
pub const Z_INDEX: f32 = 1.0;


// Imports
use systems::*;


// System Set
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Movement,
    Confinement,
}



/*
    ---------------------
    ---- Entry Point ----
    ---------------------
*/

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {

        // Configure System Sets
        app.configure_sets(Update, PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement));
        
        // Startup
        app.add_systems(Startup, spawn_player);

        // Update
        app.add_systems(Update, (
            move_player.in_set(PlayerSystemSet::Movement),
            confine_player.in_set(PlayerSystemSet::Confinement),
            collide_with_enemy,
            collide_with_star,
        ));

    }
}