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




pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        
        // Startup
        app.add_systems(Startup, spawn_camera);


        // Update
        app.add_systems(Update, (
            quit_game,
            handle_game_over,
            update_high_scores,
            high_scores_updated
        ));

        
        // Resources
        app.init_resource::<Score>();
        app.init_resource::<HighScores>();

    }
}