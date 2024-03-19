/*
    #############################
    ##                         ##
    ##        GAME OVER        ##
    ##                         ##
    #############################
*/

use bevy::prelude::*;

use crate::AppState;


mod systems;
mod components;
mod styles;



pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        // On Enter
        app.add_systems(OnEnter(AppState::GameOver), systems::layout::spawn_game_over);

        // Update
        app.add_systems(Update, 
            (
                systems::interactions::interact_with_restart_button,
                systems::interactions::interact_with_menu_button,
                systems::interactions::interact_with_quit_button,
            ).run_if(in_state(AppState::GameOver))
        );

        // On Exit
        app.add_systems(OnExit(AppState::GameOver), systems::layout::despawn_game_over);
    }
}