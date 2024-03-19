/*
    #############################
    ##                         ##
    ##        MAIN MENU        ##
    ##                         ##
    #############################
*/

mod components;
mod styles;
mod systems;

use bevy::prelude::*;
use systems::layout::*;
use crate::AppState;

use systems::interactions::*;



pub struct MainManuPlugin;

impl Plugin for MainManuPlugin {
    fn build(&self, app: &mut App) {
        
        // On Enter
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu);

        // Systems
        app.add_systems(
            Update,
            (
                interact_with_play_button,
                interact_with_quit_button
            ).run_if(in_state(AppState::MainMenu))
        );


        // On Exit
        app.add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    
    }
}