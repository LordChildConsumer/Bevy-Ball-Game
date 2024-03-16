/*
    ########################
    ##                    ##
    ##        STAR        ##
    ##                    ##
    ########################
*/




use bevy::prelude::*;
use crate::AppState;
use super::SimulationState;


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
        
        // Enter Game
        app.add_systems(OnEnter(AppState::Game), spawn_stars);

        // Update
        app.add_systems(Update,
            (
                update_score,
                tick_spawn_timer,
                spawn_over_time,
            )
            .run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running))
        );

        // Exit Game
        app.add_systems(OnExit(AppState::Game), despawn_stars);

        // Resources
        app.init_resource::<StarSpawnTimer>();

    }
}