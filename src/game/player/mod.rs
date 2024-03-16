/*
    ##########################
    ##                      ##
    ##        PLAYER        ##
    ##                      ##
    ##########################
*/




use bevy::prelude::*;
use crate::AppState;
use super::SimulationState;

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
        
        // Enter Game
        app.add_systems(OnEnter(AppState::Game), spawn_player);

        // Update
        app.add_systems(Update, 
            (
                move_player.in_set(PlayerSystemSet::Movement),
                confine_player.in_set(PlayerSystemSet::Confinement),
                collide_with_enemy,
                collide_with_star,
            )
            .run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running))
        );

        // Exit Game
        app.add_systems(OnExit(AppState::Game), despawn_player);
    }
}