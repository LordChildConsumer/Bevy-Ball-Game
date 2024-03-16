/*
    #########################
    ##                     ##
    ##        SCORE        ##
    ##                     ##
    #########################
*/




use bevy::prelude::*;

pub mod resources;
pub mod systems;


// Imports
use systems::*;
use resources::*;

use crate::AppState;




pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        // Resources
        app.init_resource::<HighScores>();

        // Enter Game
        app.add_systems(OnEnter(AppState::Game), insert_score);

        // Update
        app.add_systems(Update, (
            handle_game_over,
            update_score.run_if(in_state(AppState::Game)),
            update_high_scores,
            high_scores_updated
        ));

        // Exit Game
        app.add_systems(OnExit(AppState::Game), remove_score);

    }
}