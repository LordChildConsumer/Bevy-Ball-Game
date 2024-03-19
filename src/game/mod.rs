/*
    ########################
    ##                    ##
    ##        GAME        ##
    ##                    ##
    ########################
*/

mod systems;

pub mod ui;
pub mod enemy;
pub mod score;
pub mod star;
mod player;


use bevy::prelude::*;
use crate::AppState;
use systems::*;




// Simulation State
#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
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

        // Pause on Enter Game
        app.add_systems(OnEnter(AppState::Game), pause_simulation);

        // Plugins
        app.add_plugins((
            enemy::EnemyPlugin,
            score::ScorePlugin,
            star::StarPlugin,
            player::PlayerPlugin,
            ui::GameUIPlugin,
        ));

        // Startup
        app.add_systems(Startup, spawn_camera);

        // Update
        app.add_systems(Update,
            (
                toggle_pause_game.run_if(in_state(AppState::Game)),
                // quit_game,
            )
        );

        // Unpause on Exit Game
        app.add_systems(OnExit(AppState::Game), unpause_simulation);

    }
}