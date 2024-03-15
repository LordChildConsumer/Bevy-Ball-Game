use bevy::prelude::*;

// Modules
pub mod components;
pub mod resources;
pub mod systems;


// Constants
pub const RADIUS: f32 = 32.0;
const SPEED: f32 = 200.0;
const INITIAL_COUNT: usize = 4;
const Z_INDEX: f32 = 1.0;

const SPAWN_MARGIN: f32 = 32.0;
const SPAWN_TIME: f32 = 3.0;


// Imports
use systems::*;
use resources::*;




/*
    ---------------------
    ---- Entry Point ----
    ---------------------
*/

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {

        // Startup
        app.add_systems(Startup, spawn_enemies);


        // Update
        app.add_systems(Update, (
            move_enemies,
            update_enemy_direction,
            tick_spawn_timer,
            spawn_over_time,
        ));


        // Resources
        app.init_resource::<EnemySpawnTimer>();

    }
}