/*
    ########################
    ##                    ##
    ##        STAR        ##
    ##                    ##
    ########################
*/




use bevy::prelude::*;


// Modules
pub mod components;
pub mod resources;
pub mod systems;

// Constants
pub const RADIUS: f32 = 15.0;
const INITIAL_COUNT: i32 = 10;

// Imports
use systems::*;
use resources::*;




pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        
        // Startup
        app.add_systems(Startup, spawn_stars);

        // Update
        app.add_systems(Update, (
            update_score,
            tick_spawn_timer,
            spawn_over_time,
        ));

        // Resources
        app.init_resource::<StarSpawnTimer>();

    }
}