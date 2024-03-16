/*
    ########################
    ##                    ##
    ##        GAME        ##
    ##                    ##
    ########################
*/

mod systems;


pub mod enemy;
pub mod score;
pub mod star;
mod player;


use bevy::prelude::*;

use crate::AppState;


// Simulation State
#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum SimulationState {
    #[default]
    Paused,
    Running,
}



/*
    ---------------------
    ---- Entry Point ----
    ---------------------
*/

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        
        // Simulation State
        app.init_state::<SimulationState>();

        // Plugins
        app.add_plugins((
            enemy::EnemyPlugin,
            score::ScorePlugin,
            star::StarPlugin,
            player::PlayerPlugin,
        ));

        // Systems
        app.add_systems(Update, 
            systems::toggle_pause_game.run_if(in_state(AppState::Game))
        );

    }
}