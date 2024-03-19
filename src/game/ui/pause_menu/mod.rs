/*
    ##############################
    ##                          ##
    ##        PAUSE MENU        ##
    ##                          ##
    ##############################
*/

use bevy::prelude::*;

use crate::{
    game::SimulationState,
    AppState
};

mod components;
mod styles;
mod systems;


pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        // On Enter
        app.add_systems(OnEnter(SimulationState::Paused), systems::layout::spawn_pause_menu);

        // Update
        app.add_systems(Update, 
            (
                systems::interactions::interact_with_resume_button,
                systems::interactions::interact_with_menu_button,
                systems::interactions::interact_with_quit_button,
            )
            .run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Paused))
        );

        // On Exit
        app.add_systems(OnExit(SimulationState::Paused), systems::layout::despawn_pause_menu);
    }
}